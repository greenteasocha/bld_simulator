use crate::cube::State;
use serde::{Deserialize, Serialize};

const BUFFER_PIECE: usize = 6;
const NEW_LOOP_PRIORITY: [usize; 11] = [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

/// エッジの2点交換操作を表す（eo考慮版）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeSwapOperation {
    /// 交換する2つのインデックス
    pub target1: usize,
    pub target2: usize,
    /// 交換操作を行う前の eo[target1] を記録
    pub orientation: u8,
}

impl EdgeSwapOperation {
    pub fn new(target1: usize, target2: usize, orientation: u8) -> Self {
        Self {
            target1,
            target2,
            orientation,
        }
    }

    /// この操作を State に適用する
    pub fn apply(&self, state: &State) -> State {
        let mut new_ep = state.ep;
        let mut new_eo = state.eo;

        // ep の交換
        new_ep.swap(self.target1, self.target2);

        // eo の変化: 両方のエッジにorientationを加算
        // エッジの場合、交換時の向きの変化は両方同じorientationを加算する
        // 注意: swap前の元の値を使用する必要がある
        let old_eo_target1 = state.eo[self.target1];
        let old_eo_target2 = state.eo[self.target2];
        new_eo[self.target1] = (old_eo_target2 + self.orientation) % 2;
        new_eo[self.target2] = (old_eo_target1 + self.orientation) % 2;

        State::new(state.cp, state.co, new_ep, new_eo)
    }
}

impl std::fmt::Display for EdgeSwapOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // target_sticker[edge_index][orientation]
        const TARGET_STICKERS: [[&str; 2]; 12] = [
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

        let target1_sticker = TARGET_STICKERS[self.target1][0]; // target1は常に0なのでorientation=0
        let target2_sticker = TARGET_STICKERS[self.target2][self.orientation as usize];

        write!(f, "Swap: {} ↔ {}", target1_sticker, target2_sticker)
    }
}

/// エッジの向き変更操作を表す
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeFlipOperation {
    /// 対象のインデックス
    pub target: usize,
}

impl EdgeFlipOperation {
    pub fn new(target: usize) -> Self {
        Self { target }
    }

    /// この操作を State に適用する
    pub fn apply(&self, state: &State) -> State {
        let mut new_eo = state.eo;
        new_eo[self.target] = (new_eo[self.target] + 1) % 2;

        State::new(state.cp, state.co, state.ep, new_eo)
    }
}

impl std::fmt::Display for EdgeFlipOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TARGET_STICKERS: [&str; 12] = [
            "BL", "BR", "FR", "FL", "UB", "UR", "UF", "UL", "DB", "DR", "DF", "DL",
        ];

        const FLIP_EXISTANCE: [&str; 2] = [
            "not flipped", // 0: フリップなし
            "flipped",     // 1: フリップ
        ];

        let target_sticker = TARGET_STICKERS[self.target];

        // 現在の状態からフリップ後の状態を表示
        write!(f, "Flip: {} ({})", target_sticker, FLIP_EXISTANCE[1])
    }
}

/// 操作の種類
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation_type", content = "data")]
pub enum EdgeOperation {
    Swap(EdgeSwapOperation),
    Flip(EdgeFlipOperation),
}

impl EdgeOperation {
    /// この操作を State に適用する
    pub fn apply(&self, state: &State) -> State {
        match self {
            EdgeOperation::Swap(op) => op.apply(state),
            EdgeOperation::Flip(op) => op.apply(state),
        }
    }
}

impl std::fmt::Display for EdgeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeOperation::Swap(op) => write!(f, "{}", op),
            EdgeOperation::Flip(op) => write!(f, "{}", op),
        }
    }
}

/// エッジの置換と向きを完成状態に戻すための操作列を計算
pub struct EdgeInspection;

impl EdgeInspection {
    /// 交換分析モードでの値変換を行う
    /// 
    /// # Arguments
    /// * `value` - 変換対象の値
    /// * `use_swap_inspection` - 交換分析モードを使用するか
    /// 
    /// # Returns
    /// 変換後の値（交換分析モードの場合、5→6, 6→5、それ以外はそのまま）
    fn convert_value_for_swap_inspection(value: u8, use_swap_inspection: bool) -> u8 {
        if !use_swap_inspection {
            return value;
        }
        
        match value {
            5 => 6,
            6 => 5,
            _ => value,
        }
    }
    /// ep と eo を完成状態に戻すための操作列を計算
    ///
    /// # Arguments
    /// * `state` - 現在のキューブ状態（この関数内で変更されない）
    /// * `use_swap_inspection` - 交換分析モードを使用するか（デフォルト: false）
    ///
    /// # Returns
    /// エッジ操作の列（Swap と Flip）
    /// 
    /// # 交換分析モード
    /// `use_swap_inspection` が true の場合、ep 配列の値 5 と 6 を入れ替えて扱います。
    /// これにより、UF(6) と UR(5) の交換を特別に扱うことができます。
    pub fn solve_edge_permutation_with_orientation(state: &State, use_swap_inspection: bool) -> Vec<EdgeOperation> {
        let mut current_state = state.clone();
        let mut operations = Vec::new();

        loop {
            // ep[BUFFER_PIECE]との二点交換ループ
            // 交換分析モードの場合、値を変換して比較
            loop {
                let buffer_value = Self::convert_value_for_swap_inspection(
                    current_state.ep[BUFFER_PIECE],
                    use_swap_inspection
                );
                let expected_buffer_value = u8::try_from(BUFFER_PIECE).expect("value too large for u8");
                
                if buffer_value == expected_buffer_value {
                    break;
                }
                
                // 交換先ターゲットを決定（変換後の値を使用）
                let raw_target_value = current_state.ep[BUFFER_PIECE];
                let converted_target_value = Self::convert_value_for_swap_inspection(
                    raw_target_value,
                    use_swap_inspection
                );
                let target = converted_target_value as usize;
                let ori = current_state.eo[BUFFER_PIECE]; // 交換前のeo[BUFFER_PIECE]を記録

                let operation =
                    EdgeOperation::Swap(EdgeSwapOperation::new(BUFFER_PIECE, target, ori));
                operations.push(operation.clone());

                current_state = operation.apply(&current_state);
            }

            // 別ループ探索
            if let Some(next_index) = Self::find_next_misplaced_ep(&current_state.ep, use_swap_inspection) {
                let ori = current_state.eo[BUFFER_PIECE]; // 交換前のeo[BUFFER_PIECE]を記録

                let operation =
                    EdgeOperation::Swap(EdgeSwapOperation::new(BUFFER_PIECE, next_index, ori));
                operations.push(operation.clone());

                current_state = operation.apply(&current_state);
            } else {
                // ep の終了条件が満たされた
                break;
            }
        }

        // eo 専用の終了処理
        for i in 0..12 {
            if current_state.eo[i] != 0 {
                let operation = EdgeOperation::Flip(EdgeFlipOperation::new(i));

                operations.push(operation.clone());
                current_state = operation.apply(&current_state);
            }
        }

        operations
    }

    /// ep[i] ≠ i となる最小の i を探す
    /// 
    /// # Arguments
    /// * `ep` - エッジ配列
    /// * `use_swap_inspection` - 交換分析モードを使用するか
    fn find_next_misplaced_ep(ep: &[u8; 12], use_swap_inspection: bool) -> Option<usize> {
        for i in NEW_LOOP_PRIORITY {
            let converted_value = Self::convert_value_for_swap_inspection(ep[i], use_swap_inspection);
            if converted_value != i as u8 {
                return Some(i);
            }
        }
        None
    }

    /// 操作列を人間が読みやすい形式で出力
    pub fn format_operations(operations: &[EdgeOperation]) -> String {
        operations
            .iter()
            .enumerate()
            .map(|(i, op)| format!("Step {}: {}", i + 1, op))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ep_only_example() {
        // ep のみの例: [1,0,2,3,4,5,6,7,9,8,10,11]
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 7, 6, 9, 8, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            println!("Applying operation: {}", op);
            current_state = op.apply(&current_state);
        }

        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert!(current_state.is_solved());
    }

    #[test]
    fn test_with_orientation() {
        // eo も含む例
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_flip_only() {
        // ep は完成、eo のみ変更が必要
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        // Flip操作のみが含まれるはず
        assert!(operations
            .iter()
            .all(|op| matches!(op, EdgeOperation::Flip(_))));

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_already_solved() {
        let state = State::solved();
        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);
        assert!(operations.is_empty());
    }

    #[test]
    fn test_complex_case() {
        // 複雑なケース
        // L2 B2 D2 R2 B F2 L2 D' U' B2 L' B' D2 L R B2 D'
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [4, 1, 10, 8, 5, 0, 9, 7, 6, 2, 11, 3],
            [0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1],
        );

        println!("\n=== test_complex_case ===");

        println!("L2 B2 D2 R2 B F2 L2 D' U' B2 L' B' D2 L R B2 D'");
        println!("Initial state:");
        println!("  ep: {:?}", state.ep);
        println!("  eo: {:?}", state.eo);

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        println!("\nOperations:");
        println!("{}", EdgeInspection::format_operations(&operations));

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        println!("\nFinal state:");
        println!("  ep: {:?}", current_state.ep);
        println!("  eo: {:?}", current_state.eo);
        println!("=========================\n");

        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_debug_step_by_step() {
        // デバッグ用: ステップごとの状態を出力
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [3, 1, 2, 0, 5, 7, 4, 6, 9, 8, 10, 11],
            [1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0],
        );

        println!("\n=== Debug: Step-by-step execution ===");
        println!("Initial state:");
        println!("  ep: {:?}", state.ep);
        println!("  eo: {:?}", state.eo);
        println!();

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        let mut current_state = state.clone();

        for (i, op) in operations.iter().enumerate() {
            println!("Step {}: {}", i + 1, op);
            println!(
                "  Before: ep: {:?}, eo: {:?}",
                current_state.ep, current_state.eo
            );

            current_state = op.apply(&current_state);

            println!(
                "  After:  ep: {:?}, eo: {:?}",
                current_state.ep, current_state.eo
            );
            println!();
        }

        println!("Final state:");
        println!("  ep: {:?}", current_state.ep);
        println!("  eo: {:?}", current_state.eo);
        println!("=== End of debug output ===\n");

        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_swap_inspection_mode() {
        // 交換分析モードのテストケース
        // ドキュメントの例
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 6, 5, 4, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== Swap Inspection Mode Test ===");
        println!("Initial state:");
        println!("  ep: {:?}", state.ep);
        println!("  eo: {:?}", state.eo);

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, true);

        println!("\nOperations:");
        println!("{}", EdgeInspection::format_operations(&operations));

        // 想定される操作：
        // Step 1: Swap: 6 ↔ 4 (ori: 0)
        // Step 2: Swap: 6 ↔ 5 (ori: 0)
        assert_eq!(operations.len(), 2, "Should have exactly 2 swap operations");

        // 操作を順次適用
        let mut current_state = state.clone();
        for (i, op) in operations.iter().enumerate() {
            println!("\nStep {}:", i + 1);
            println!("  Before: ep: {:?}, eo: {:?}", current_state.ep, current_state.eo);
            current_state = op.apply(&current_state);
            println!("  After:  ep: {:?}, eo: {:?}", current_state.ep, current_state.eo);
        }

        // 交換分析モードでの終了状態: ep[5]=6, ep[6]=5
        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 6, 5, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        println!("\nFinal state:");
        println!("  ep: {:?}", current_state.ep);
        println!("  eo: {:?}", current_state.eo);
        println!("=========================\n");
    }

    #[test]
    fn test_swap_inspection_mode_detailed() {
        // 交換分析モードの詳細テスト
        // ドキュメントの動作例を検証
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 9, 6, 7, 8, 5, 10, 11], // ドキュメントの例
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== Swap Inspection Mode Detailed Test ===");
        println!("Initial state:");
        println!("  ep: {:?}", state.ep);
        println!("\n期待される動作:");
        println!("  ep[6] = 6 → 5に変換 → target = 5");
        println!("  交換後: [0, 1, 2, 3, 4, 6, 9, 7, 8, 5, 10, 11]");
        println!("  ep[6] = 9 (変換なし) → target = 9");
        println!("  交換後: [0, 1, 2, 3, 4, 6, 5, 7, 8, 9, 10, 11]");
        println!("  ep[6] = 5 → 6に変換 → バッファが自分自身を指す\n");

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state, true);

        println!("\nActual operations:");
        for (i, op) in operations.iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }

        // 操作を順次適用
        let mut current_state = state.clone();
        println!("\nステップごとの状態変化:");
        for (i, op) in operations.iter().enumerate() {
            println!("\nStep {}: {}", i + 1, op);
            println!("  Before: ep: {:?}", current_state.ep);
            current_state = op.apply(&current_state);
            println!("  After:  ep: {:?}", current_state.ep);
        }

        // 最終状態
        assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 6, 5, 7, 8, 9, 10, 11]);
        assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        println!("\n最終状態: {:?}", current_state.ep);
        println!("================================\n");
    }

    #[test]
    fn test_swap_inspection_vs_normal_mode() {
        // 通常モードと交換分析モードの比較
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 6, 5, 4, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== Normal Mode vs Swap Inspection Mode ===");
        println!("Initial state: {:?}", state.ep);

        // 通常モード
        let normal_ops = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);
        println!("\n通常モード ({} operations):", normal_ops.len());
        for (i, op) in normal_ops.iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }

        let mut normal_state = state.clone();
        for op in &normal_ops {
            normal_state = op.apply(&normal_state);
        }
        println!("  Final: {:?}", normal_state.ep);
        assert!(normal_state.is_solved());

        // 交換分析モード
        let swap_ops = EdgeInspection::solve_edge_permutation_with_orientation(&state, true);
        println!("\n交換分析モード ({} operations):", swap_ops.len());
        for (i, op) in swap_ops.iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }

        let mut swap_state = state.clone();
        for op in &swap_ops {
            swap_state = op.apply(&swap_state);
        }
        println!("  Final: {:?}", swap_state.ep);
        assert_eq!(swap_state.ep, [0, 1, 2, 3, 4, 6, 5, 7, 8, 9, 10, 11]);

        println!("\n注意: 交換分析モードでは ep[5]=6, ep[6]=5 が終了状態");
        println!("======================================\n");
    }
}
