use super::{CornerOperation, CornerSwapOperation, CornerTwistOperation, EdgeOperation, EdgeSwapOperation, EdgeFlipOperation};
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

    /// 空の MoveSequence を作成
    pub fn empty() -> Self {
        Self {
            moves: Vec::new(),
            description: String::new(),
        }
    }

    /// 空かどうか
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
}

impl std::fmt::Display for MoveSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::parser::sequence_to_string;
        write!(f, "{}", sequence_to_string(&self.moves))
    }
}

/// MoveSequenceの集合を管理する構造体
#[derive(Debug, Clone, PartialEq)]
pub struct MoveSequenceCollection {
    sequences: Vec<MoveSequence>,
}

impl MoveSequenceCollection {
    /// 新しい空のコレクションを作成
    pub fn new() -> Self {
        Self {
            sequences: Vec::new(),
        }
    }

    /// 単一のMoveSequenceからコレクションを作成
    pub fn from_single(sequence: MoveSequence) -> Self {
        Self {
            sequences: vec![sequence],
        }
    }

    /// MoveSequenceのVecからコレクションを作成
    pub fn from_vec(sequences: Vec<MoveSequence>) -> Self {
        Self { sequences }
    }

    /// MoveSequenceを追加
    pub fn push(&mut self, sequence: MoveSequence) {
        self.sequences.push(sequence);
    }

    /// 別のコレクションを追加
    pub fn extend(&mut self, other: MoveSequenceCollection) {
        self.sequences.extend(other.sequences);
    }

    /// 内部のVecへの参照を取得
    pub fn sequences(&self) -> &[MoveSequence] {
        &self.sequences
    }

    /// 内部のVecを消費して取得
    pub fn into_sequences(self) -> Vec<MoveSequence> {
        self.sequences
    }

    /// 空かどうか
    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }

    /// コレクション内のシーケンス数
    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    /// すべてのmovesを結合した単一のSequenceを取得
    pub fn flatten_moves(&self) -> Sequence {
        self.sequences
            .iter()
            .flat_map(|seq| seq.moves.clone())
            .collect()
    }

    /// すべてのdescriptionを結合した文字列を取得
    pub fn flatten_description(&self, separator: &str) -> String {
        self.sequences
            .iter()
            .map(|seq| seq.description.as_str())
            .filter(|desc| !desc.is_empty())
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// 単一のMoveSequenceに統合（後方互換性のため）
    pub fn into_single(self) -> MoveSequence {
        let moves = self.flatten_moves();
        let description = self.flatten_description(", ");
        MoveSequence::new(moves, description)
    }
}

impl Default for MoveSequenceCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MoveSequenceCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::parser::sequence_to_string;
        
        for (i, seq) in self.sequences.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            
            // descriptionがある場合は表示
            if !seq.description.is_empty() {
                write!(f, "// {}\n", seq.description)?;
            }
            
            write!(f, "{}", sequence_to_string(&seq.moves))?;
        }
        
        Ok(())
    }
}

impl IntoIterator for MoveSequenceCollection {
    type Item = MoveSequence;
    type IntoIter = std::vec::IntoIter<MoveSequence>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequences.into_iter()
    }
}

impl<'a> IntoIterator for &'a MoveSequenceCollection {
    type Item = &'a MoveSequence;
    type IntoIter = std::slice::Iter<'a, MoveSequence>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequences.iter()
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

/// エッジ交換における TARGET_STICKER の定義 (Swap)
const EDGE_TARGET_STICKERS: [[&str; 2]; 12] = [
    ["BL", "LB"], // 0
    ["BR", "RB"], // 1
    ["FR", "RF"], // 2
    ["FL", "LF"], // 3
    ["UB", "BU"], // 4
    ["UR", "RU"], // 5
    ["UF", "FU"], // 6
    ["UL", "LU"], // 7
    ["DB", "BD"], // 8
    ["DR", "RD"], // 9
    ["DF", "FD"], // 10
    ["DL", "LD"], // 11
];

/// エッジフリップにおける TARGET_STICKER の定義 (Flip)
const EDGE_FLIP_TARGET_STICKERS: [&str; 12] = [
    "BL", // 0
    "BR", // 1
    "FR", // 2
    "FL", // 3
    "UB", // 4
    "UR", // 5
    "UF", // 6
    "UL", // 7
    "DB", // 8
    "DR", // 9
    "DF", // 10
    "DL", // 11
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

/// コーナーとエッジ操作列を手順列に変換する
pub struct OperationsToTurns {
    // Corner data
    ufr_expanded: HashMap<String, HashMap<String, String>>,
    ufr_parity: HashMap<String, String>,
    ufr_twist: HashMap<String, String>,
    // Edge data
    uf_expanded: HashMap<String, HashMap<String, String>>,
    uf_flip: HashMap<String, String>,
}

impl OperationsToTurns {
    /// JSONファイルの内容を受け取って初期化
    pub fn new(
        ufr_expanded_json: &str,
        ufr_parity_json: &str,
        ufr_twist_json: &str,
        uf_expanded_json: &str,
        uf_flip_json: &str,
    ) -> Result<Self, String> {
        let ufr_expanded = load_json_to_map(ufr_expanded_json)?;

        // parity と twist は1階層のJSON
        let ufr_parity: HashMap<String, String> = serde_json::from_str(ufr_parity_json)
            .map_err(|e| format!("Failed to parse ufr_parity: {}", e))?;

        let ufr_twist: HashMap<String, String> = serde_json::from_str(ufr_twist_json)
            .map_err(|e| format!("Failed to parse ufr_twist: {}", e))?;

        // Edge data loading
        let uf_expanded = load_json_to_map(uf_expanded_json)?;
        let uf_flip: HashMap<String, String> = serde_json::from_str(uf_flip_json)
            .map_err(|e| format!("Failed to parse uf_flip: {}", e))?;

        Ok(Self {
            ufr_expanded,
            ufr_parity,
            ufr_twist,
            uf_expanded,
            uf_flip,
        })
    }

    /// CornerOperation列を MoveSequenceCollection に変換
    pub fn convert(&self, operations: &[CornerOperation]) -> Result<MoveSequenceCollection, String> {
        let sequences = self.convert_to_vec(operations)?;
        Ok(MoveSequenceCollection::from_vec(sequences))
    }

    /// CornerOperation列を MoveSequence の Vec に変換（内部用）
    fn convert_to_vec(&self, operations: &[CornerOperation]) -> Result<Vec<MoveSequence>, String> {
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

    /// EdgeOperation列を MoveSequenceCollection に変換
    pub fn convert_edge_operations(&self, operations: &[EdgeOperation]) -> Result<MoveSequenceCollection, String> {
        let sequences = self.convert_edge_to_vec(operations)?;
        Ok(MoveSequenceCollection::from_vec(sequences))
    }

    /// EdgeOperation列を MoveSequence の Vec に変換（内部用）
    fn convert_edge_to_vec(&self, operations: &[EdgeOperation]) -> Result<Vec<MoveSequence>, String> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < operations.len() {
            // 1. 連続する2つの Swap を変換できるか試す
            if i + 1 < operations.len() {
                if let (EdgeOperation::Swap(swap1), EdgeOperation::Swap(swap2)) =
                    (&operations[i], &operations[i + 1])
                {
                    if let Some(seq) = self.try_convert_two_edge_swaps(swap1, swap2)? {
                        result.push(seq);
                        i += 2;
                        continue;
                    }
                }
            }

            // 2. 1つの Flip を変換
            if let EdgeOperation::Flip(flip) = &operations[i] {
                let seq = self.convert_edge_flip(flip)?;
                result.push(seq);
                i += 1;
                continue;
            }

            // Note: 1つの Swap からの変換は存在しない（ドキュメントより）
            i += 1;
        }

        Ok(result)
    }

    /// 連続する2つのEdge Swapを変換
    /// 
    /// # 前提条件
    /// - swap1.target1 と swap2.target1 は両方とも BUFFER_PIECE (6 = UF) であること
    /// - これは EdgeInspection::solve_edge_permutation_with_orientation の
    ///   アルゴリズムによって保証される
    fn try_convert_two_edge_swaps(
        &self,
        swap1: &EdgeSwapOperation,
        swap2: &EdgeSwapOperation,
    ) -> Result<Option<MoveSequence>, String> {
        // target1 は両方とも BUFFER_PIECE (6) であることを前提とする
        // swap1.target2 と swap2.target2 の orientation を使って target_sticker を決定
        let target_sticker1 =
            EDGE_TARGET_STICKERS[swap1.target2][swap1.orientation as usize].to_string();
        let target_sticker2 =
            EDGE_TARGET_STICKERS[swap2.target2][swap2.orientation as usize].to_string();

        // uf_expanded から target_sticker1 → target_sticker2 の手順を取得
        if let Some(inner_map) = self.uf_expanded.get(&target_sticker1) {
            if let Some(move_str) = inner_map.get(&target_sticker2) {
                let moves = parse_sequence(move_str)?;
                let description = format!("{} → {}", target_sticker1, target_sticker2);
                return Ok(Some(MoveSequence::new(moves, description)));
            }
        }

        Ok(None)
    }

    /// 1つのEdge Flipを変換
    fn convert_edge_flip(&self, flip: &EdgeFlipOperation) -> Result<MoveSequence, String> {
        let target_sticker = EDGE_FLIP_TARGET_STICKERS[flip.target].to_string();

        if let Some(move_str) = self.uf_flip.get(&target_sticker) {
            let moves = parse_sequence(move_str)?;
            let description = format!("Flip: {}", target_sticker);
            Ok(MoveSequence::new(moves, description))
        } else {
            Err(format!(
                "Flip move not found for target_sticker: {}",
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

    // テスト用のエッジJSONデータ
    const TEST_UF_EXPANDED: &str = r#"{
        "FR": {
            "DL": "R U R' F R F'",
            "DR": "U' R U R'"
        },
        "DR": {
            "UL": "R2 F R2 F'"
        }
    }"#;

    const TEST_UF_FLIP: &str = r#"{
        "UB": "R U R' U R U2 R'",
        "UR": "R U R' U R U2 R' U"
    }"#;

    #[test]
    fn test_convert_two_swaps() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        // Swap: UFR ↔ DBR (ori: 2) → target_sticker = "RDB"
        // Swap: UFR ↔ DFR (ori: 1) → target_sticker = "RDF"
        let swap1 = CornerSwapOperation::new(2, 5, 2); // UFR ↔ DBR (ori: 2)
        let swap2 = CornerSwapOperation::new(2, 6, 1); // UFR ↔ DFR (ori: 1)

        let operations = vec![
            CornerOperation::Swap(swap1),
            CornerOperation::Swap(swap2),
        ];

        let collection = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 1);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "RDB → RDF");
        assert_eq!(
            sequences[0].moves,
            parse_sequence("D' R U R' D R U' R'").unwrap()
        );
    }

    #[test]
    fn test_convert_single_swap() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        // Swap: UFR ↔ DBR (ori: 2) → target_sticker = "RDB"
        let swap = CornerSwapOperation::new(2, 5, 2);
        let operations = vec![CornerOperation::Swap(swap)];

        let collection = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 1);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "Parity: RDB");
    }

    #[test]
    fn test_convert_twist() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        // Twist: UFL (counter-clockwise) → target=3, orientation=1 → "FUL"
        let twist = CornerTwistOperation::new(3, 1);
        let operations = vec![CornerOperation::Twist(twist)];

        let collection = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 1);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "Twist: FUL");
        assert_eq!(
            sequences[0].moves,
            parse_sequence("R' D R D' R' D R U' R' D' R D R' D' R U").unwrap()
        );
    }

    #[test]
    fn test_mixed_operations() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        let swap1 = CornerSwapOperation::new(2, 5, 2); // RDB
        let swap2 = CornerSwapOperation::new(2, 6, 1); // RDF
        let twist = CornerTwistOperation::new(3, 1); // FUL

        let operations = vec![
            CornerOperation::Swap(swap1),
            CornerOperation::Swap(swap2),
            CornerOperation::Twist(twist),
        ];

        let collection = converter
            .convert(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 2);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "RDB → RDF");
        assert_eq!(sequences[1].description, "Twist: FUL");
    }

    #[test]
    fn test_collection_operations() {
        let seq1 = MoveSequence::new(vec![], "First".to_string());
        let seq2 = MoveSequence::new(vec![], "Second".to_string());
        let seq3 = MoveSequence::new(vec![], "Third".to_string());

        let mut collection = MoveSequenceCollection::new();
        collection.push(seq1);
        collection.push(seq2);

        assert_eq!(collection.len(), 2);
        assert!(!collection.is_empty());

        let mut collection2 = MoveSequenceCollection::from_single(seq3);
        collection.extend(collection2);

        assert_eq!(collection.len(), 3);

        let descriptions = collection.flatten_description(", ");
        assert_eq!(descriptions, "First, Second, Third");
    }

    #[test]
    fn test_collection_iteration() {
        let seq1 = MoveSequence::new(vec![], "First".to_string());
        let seq2 = MoveSequence::new(vec![], "Second".to_string());

        let collection = MoveSequenceCollection::from_vec(vec![seq1, seq2]);

        let mut count = 0;
        for seq in &collection {
            count += 1;
            assert!(!seq.description.is_empty());
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_convert_two_edge_swaps() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        // Swap: UF ↔ FR (ori: 0) → target_sticker = "FR"
        // Swap: UF ↔ DL (ori: 0) → target_sticker = "DL"
        let swap1 = EdgeSwapOperation::new(6, 2, 0); // UF ↔ FR (ori: 0)
        let swap2 = EdgeSwapOperation::new(6, 11, 0); // UF ↔ DL (ori: 0)

        let operations = vec![
            EdgeOperation::Swap(swap1),
            EdgeOperation::Swap(swap2),
        ];

        let collection = converter
            .convert_edge_operations(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 1);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "FR → DL");
        assert_eq!(
            sequences[0].moves,
            parse_sequence("R U R' F R F'").unwrap()
        );
    }

    #[test]
    fn test_convert_edge_flip() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        // Flip: UB → target=4 → "UB"
        let flip = EdgeFlipOperation::new(4);
        let operations = vec![EdgeOperation::Flip(flip)];

        let collection = converter
            .convert_edge_operations(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 1);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "Flip: UB");
        assert_eq!(
            sequences[0].moves,
            parse_sequence("R U R' U R U2 R'").unwrap()
        );
    }

    #[test]
    fn test_mixed_edge_operations() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        let swap1 = EdgeSwapOperation::new(6, 2, 0); // FR
        let swap2 = EdgeSwapOperation::new(6, 9, 0); // DR
        let flip = EdgeFlipOperation::new(5); // UR

        let operations = vec![
            EdgeOperation::Swap(swap1),
            EdgeOperation::Swap(swap2),
            EdgeOperation::Flip(flip),
        ];

        let collection = converter
            .convert_edge_operations(&operations)
            .expect("Failed to convert");

        assert_eq!(collection.len(), 2);
        let sequences = collection.sequences();
        assert_eq!(sequences[0].description, "FR → DR");
        assert_eq!(sequences[1].description, "Flip: UR");
    }

    #[test]
    fn test_collection_display() {
        let converter = OperationsToTurns::new(TEST_UFR_EXPANDED, TEST_UFR_PARITY, TEST_UFR_TWIST, TEST_UF_EXPANDED, TEST_UF_FLIP)
            .expect("Failed to create converter");

        let swap1 = CornerSwapOperation::new(2, 5, 2); // RDB
        let swap2 = CornerSwapOperation::new(2, 6, 1); // RDF
        let twist = CornerTwistOperation::new(3, 1); // FUL

        let operations = vec![
            CornerOperation::Swap(swap1),
            CornerOperation::Swap(swap2),
            CornerOperation::Twist(twist),
        ];

        let collection = converter
            .convert(&operations)
            .expect("Failed to convert");

        let display = format!("{}", collection);
        println!("\n=== Display Output ===\n{}", display);

        // 改行が含まれていることを確認
        assert!(display.contains('\n'));
        
        // descriptionがコメントとして含まれていることを確認
        assert!(display.contains("// RDB → RDF"));
        assert!(display.contains("// Twist: FUL"));
    }
}
