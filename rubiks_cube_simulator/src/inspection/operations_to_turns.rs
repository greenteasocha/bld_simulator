use super::{CornerOperation, CornerSwapOperation, CornerTwistOperation};
use crate::parser::{parse_sequence, Sequence};
use serde_json::Value;
use std::collections::HashMap;

/// 一連の操作から得られた手順
#[derive(Debug, Clone, PartialEq)]
pub struct MoveSequence {
    pub moves: Sequence,
    pub description: String,
}

impl MoveSequence {
    pub fn new(moves: Sequence, description: String) -> Self {
        Self { moves, description }
    }
}

/// 2点交換における TARGET_STICKER の定義
const TARGET_STICKERS: [[&str; 3]; 8] = [
    ["UBL", "BUL", "LUB"], // 0
    ["UBR", "RUB", "BUR"], // 1
    ["UFR", "FUR", "RUF"], // 2
    ["UFL", "LUF", "FUL"], // 3
    ["DBL", "LDB", "BDL"], // 4
    ["DBR", "BDR", "RDB"], // 5
    ["DFR", "RDF", "FDR"], // 6
    ["DFL", "FDL", "LDF"], // 7
];

/// Twist における TARGET_STICKER の定義
const TWIST_TARGET_STICKERS: [[&str; 3]; 8] = [
    ["UBL", "LUB", "BUL"], // 0
    ["UBR", "BUR", "RUB"], // 1
    ["UFR", "RUF", "FUR"], // 2
    ["UFL", "FUL", "LUF"], // 3
    ["DBL", "BDL", "LDB"], // 4
    ["DBR", "RDB", "BDR"], // 5
    ["DFR", "FDR", "RDF"], // 6
    ["DFL", "LDF", "FDL"], // 7
];

/// JSONファイルを読み込んでHashMapに変換
fn load_json_to_map(json_str: &str) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let json: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;

    let mut result = HashMap::new();

    if let Some(obj) = json.as_object() {
        for (key1, val1) in obj {
            if let Some(inner_obj) = val1.as_object() {
                let mut inner_map = HashMap::new();
                for (key2, val2) in inner_obj {
                    if let Some(s) = val2.as_str() {
                        inner_map.insert(key2.clone(), s.to_string());
                    }
                }
                result.insert(key1.clone(), inner_map);
            }
        }
    }

    Ok(result)
}

/// コーナー操作列を手順列に変換する
pub struct OperationsToTurns {
    ufr_expanded: HashMap<String, HashMap<String, String>>,
    ufr_parity: HashMap<String, String>,
    ufr_twist: HashMap<String, String>,
}

impl OperationsToTurns {
    /// JSONファイルの内容を受け取って初期化
    pub fn new(
        ufr_expanded_json: &str,
        ufr_parity_json: &str,
        ufr_twist_json: &str,
    ) -> Result<Self, String> {
        let ufr_expanded = load_json_to_map(ufr_expanded_json)?;

        // parity と twist は1階層のJSON
        let ufr_parity: HashMap<String, String> = serde_json::from_str(ufr_parity_json)
            .map_err(|e| format!("Failed to parse ufr_parity: {}", e))?;

        let ufr_twist: HashMap<String, String> = serde_json::from_str(ufr_twist_json)
            .map_err(|e| format!("Failed to parse ufr_twist: {}", e))?;

        Ok(Self {
            ufr_expanded,
            ufr_parity,
            ufr_twist,
        })
    }

    /// CornerOperation列を MoveSequence列に変換
    pub fn convert(&self, operations: &[CornerOperation]) -> Result<Vec<MoveSequence>, String> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < operations.len() {
            // 1. 連続する2つの Swap を変換できるか試す
            if i + 1 < operations.len() {
                if let (CornerOperation::Swap(swap1), CornerOperation::Swap(swap2)) =
                    (&operations[i], &operations[i + 1])
                {
                    if let Some(seq) = self.try_convert_two_swaps(swap1, swap2)? {
                        result.push(seq);
                        i += 2;
                        continue;
                    }
                }
            }

            // 2. 1つの Swap を変換
            if let CornerOperation::Swap(swap) = &operations[i] {
                let seq = self.convert_single_swap(swap)?;
                result.push(seq);
                i += 1;
                continue;
            }

            // 3. 1つの Twist を変換
            if let CornerOperation::Twist(twist) = &operations[i] {
                let seq = self.convert_twist(twist)?;
                result.push(seq);
                i += 1;
                continue;
            }

            i += 1;
        }

        Ok(result)
    }

    /// 連続する2つのSwapを変換
    /// 
    /// # 前提条件
    /// - swap1.target1 と swap2.target1 は両方とも BUFFER_PIECE (2 = UFR) であること
    /// - これは CornerInspection::solve_corner_permutation_with_orientation の
    ///   アルゴリズムによって保証される
    fn try_convert_two_swaps(
        &self,
        swap1: &CornerSwapOperation,
        swap2: &CornerSwapOperation,
    ) -> Result<Option<MoveSequence>, String> {
        // target1 は両方とも BUFFER_PIECE (2) であることを前提とする
        // swap1.target2 と swap2.target2 の orientation を使って target_sticker を決定
        let target_sticker1 =
            TARGET_STICKERS[swap1.target2][swap1.orientation as usize].to_string();
        let target_sticker2 =
            TARGET_STICKERS[swap2.target2][swap2.orientation as usize].to_string();

        // ufr_expanded から target_sticker1 → target_sticker2 の手順を取得
        if let Some(inner_map) = self.ufr_expanded.get(&target_sticker1) {
            if let Some(move_str) = inner_map.get(&target_sticker2) {
                let moves = parse_sequence(move_str)?;
                let description = format!("{} → {}", target_sticker1, target_sticker2);
                return Ok(Some(MoveSequence::new(moves, description)));
            }
        }

        Ok(None)
    }

    /// 1つのSwapを変換
    fn convert_single_swap(&self, swap: &CornerSwapOperation) -> Result<MoveSequence, String> {
        let target_sticker = TARGET_STICKERS[swap.target2][swap.orientation as usize].to_string();

        if let Some(move_str) = self.ufr_parity.get(&target_sticker) {
            let moves = parse_sequence(move_str)?;
            let description = format!("Parity: {}", target_sticker);
            Ok(MoveSequence::new(moves, description))
        } else {
            Err(format!(
                "Parity move not found for target_sticker: {}",
                target_sticker
            ))
        }
    }

    /// 1つのTwistを変換
    fn convert_twist(&self, twist: &CornerTwistOperation) -> Result<MoveSequence, String> {
        let target_sticker =
            TWIST_TARGET_STICKERS[twist.target][twist.orientation as usize].to_string();

        if let Some(move_str) = self.ufr_twist.get(&target_sticker) {
            let moves = parse_sequence(move_str)?;
            let description = format!("Twist: {}", target_sticker);
            Ok(MoveSequence::new(moves, description))
        } else {
            Err(format!(
                "Twist move not found for target_sticker: {}",
                target_sticker
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テスト用の簡単なJSONデータ
    const TEST_UFR_EXPANDED: &str = r#"{
        "RDB": {
            "RDF": "D' R U R' D R U' R'"
        },
        "RDF": {
            "UFL": "U' R' D' R U R' D R"
        }
    }"#;

    const TEST_UFR_PARITY: &str = r#"{
        "RDB": "U2 D' R' F R2 U' R' U' R U R' F' R U R' U D"
    }"#;

    const TEST_UFR_TWIST: &str = r#"{
        "FUL": "R' D R D' R' D R U' R' D' R D R' D' R U"
    }"#;

    #[test]
    fn test_convert_two_swaps() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST)
            .expect("Failed to create converter");

        // Swap: UFR ↔ DBR (ori: 2) → target_sticker = "BDR"
        // Swap: UFR ↔ DFR (ori: 1) → target_sticker = "RDF"
        let swap1 = CornerSwapOperation::new(2, 5, 2); // UFR ↔ DBR (ori: 2)
        let swap2 = CornerSwapOperation::new(2, 6, 1); // UFR ↔ DFR (ori: 1)

        let operations = vec![
            CornerOperation::Swap(swap1),
            CornerOperation::Swap(swap2),
        ];

        let sequences = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(sequences.len(), 1);
        assert_eq!(sequences[0].description, "RDB → RDF");
        assert_eq!(
            sequences[0].moves,
            parse_sequence("D' R U R' D R U' R'").unwrap()
        );
    }

    #[test]
    fn test_convert_single_swap() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST)
            .expect("Failed to create converter");

        // Swap: UFR ↔ DBR (ori: 2) → target_sticker = "BDR"
        let swap = CornerSwapOperation::new(2, 5, 2);
        let operations = vec![CornerOperation::Swap(swap)];

        let sequences = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(sequences.len(), 1);
        assert_eq!(sequences[0].description, "Parity: RDB");
    }

    #[test]
    fn test_convert_twist() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST)
            .expect("Failed to create converter");

        // Twist: UFL (counter-clockwise) → target=3, orientation=1 → "FUL"
        let twist = CornerTwistOperation::new(3, 1);
        let operations = vec![CornerOperation::Twist(twist)];

        let sequences = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(sequences.len(), 1);
        assert_eq!(sequences[0].description, "Twist: FUL");
        assert_eq!(
            sequences[0].moves,
            parse_sequence("R' D R D' R' D R U' R' D' R D R' D' R U").unwrap()
        );
    }

    #[test]
    fn test_mixed_operations() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST)
            .expect("Failed to create converter");

        let swap1 = CornerSwapOperation::new(2, 5, 2); // BDR
        let swap2 = CornerSwapOperation::new(2, 6, 1); // RDF
        let twist = CornerTwistOperation::new(3, 1); // FUL

        let operations = vec![
            CornerOperation::Swap(swap1),
            CornerOperation::Swap(swap2),
            CornerOperation::Twist(twist),
        ];

        let sequences = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(sequences.len(), 2);
        assert_eq!(sequences[0].description, "RDB → RDF");
        assert_eq!(sequences[1].description, "Twist: FUL");
    }
}
