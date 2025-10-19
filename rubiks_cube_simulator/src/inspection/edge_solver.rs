use crate::cube::State;

const BUFFER_PIECE: usize = 8;
const NEW_LOOP_PRIORITY: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11];

/// エッジの2点交換操作を表す（eo考慮版）
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
    /// ep と eo を完成状態に戻すための操作列を計算
    ///
    /// # Arguments
    /// * `state` - 現在のキューブ状態（この関数内で変更されない）
    ///
    /// # Returns
    /// エッジ操作の列（Swap と Flip）
    pub fn solve_edge_permutation_with_orientation(state: &State) -> Vec<EdgeOperation> {
        let mut current_state = state.clone();
        let mut operations = Vec::new();

        loop {
            // ep[BUFFER_PIECE]との二点交換ループ
            while current_state.ep[BUFFER_PIECE]
                != u8::try_from(BUFFER_PIECE).expect("value too large for u8")
            {
                let target = current_state.ep[BUFFER_PIECE] as usize;
                let ori = current_state.eo[BUFFER_PIECE]; // 交換前のeo[BUFFER_PIECE]を記録

                let operation =
                    EdgeOperation::Swap(EdgeSwapOperation::new(BUFFER_PIECE, target, ori));
                operations.push(operation.clone());

                current_state = operation.apply(&current_state);
            }

            // 別ループ探索
            if let Some(next_index) = Self::find_next_misplaced_ep(&current_state.ep) {
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
    fn find_next_misplaced_ep(ep: &[u8; 12]) -> Option<usize> {
        for i in NEW_LOOP_PRIORITY {
            if ep[i] != i as u8 {
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
            [1, 0, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

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
    fn test_with_orientation() {
        // eo も含む例
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

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

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

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
        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);
        assert!(operations.is_empty());
    }

    #[test]
    fn test_complex_case() {
        // 複雑なケース
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 6, 2, 5, 4, 3, 7, 9, 10, 11, 8],
            [1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );

        println!("\n=== test_complex_case ===");
        println!("Initial state:");
        println!("  ep: {:?}", state.ep);
        println!("  eo: {:?}", state.eo);

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

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

        let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

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
}
