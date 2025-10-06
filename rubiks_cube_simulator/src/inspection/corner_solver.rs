use crate::cube::State;

/// コーナーの2点交換操作を表す（co考慮版）
#[derive(Debug, Clone, PartialEq)]
pub struct CornerSwapOperation {
    /// 交換する2つのインデックス
    pub target1: usize,
    pub target2: usize,
    /// 交換操作を行う前の co[target1] を記録
    pub orientation: u8,
}

impl CornerSwapOperation {
    pub fn new(target1: usize, target2: usize, orientation: u8) -> Self {
        Self {
            target1,
            target2,
            orientation,
        }
    }

    /// この操作を State に適用する
    pub fn apply(&self, state: &State) -> State {
        let mut new_cp = state.cp;
        let mut new_co = state.co;

        // cp の交換
        new_cp.swap(self.target1, self.target2);

        // co の変化: co[target1], co[target2] = (co[target2] + orientation) % 3, (co[target1] - orientation + 3) % 3
        let new_co_target1 = (self.orientation + new_co[self.target2]) % 3;
        let new_co_target2 = (new_co[self.target1] + 3 - self.orientation) % 3;
        new_co[self.target1] = new_co_target1;
        new_co[self.target2] = new_co_target2;

        State::new(new_cp, new_co, state.ep, state.eo)
    }
}

impl std::fmt::Display for CornerSwapOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Swap: {} ↔ {} (ori: {})",
            self.target1, self.target2, self.orientation
        )
    }
}

/// コーナーの向き変更操作を表す
#[derive(Debug, Clone, PartialEq)]
pub struct CornerTwistOperation {
    /// 対象のインデックス
    pub target: usize,
    /// 向きの値 (1 or 2)
    pub orientation: u8,
}

impl CornerTwistOperation {
    pub fn new(target: usize, orientation: u8) -> Self {
        Self {
            target,
            orientation,
        }
    }

    /// この操作を State に適用する
    pub fn apply(&self, state: &State) -> State {
        let mut new_co = state.co;
        new_co[self.target] = (new_co[self.target] + 3 - self.orientation) % 3;

        State::new(state.cp, new_co, state.ep, state.eo)
    }
}

impl std::fmt::Display for CornerTwistOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Twist: corner[{}] (ori: {})",
            self.target, self.orientation
        )
    }
}

/// 操作の種類
#[derive(Debug, Clone, PartialEq)]
pub enum CornerOperation {
    Swap(CornerSwapOperation),
    Twist(CornerTwistOperation),
}

impl CornerOperation {
    /// この操作を State に適用する
    pub fn apply(&self, state: &State) -> State {
        match self {
            CornerOperation::Swap(op) => op.apply(state),
            CornerOperation::Twist(op) => op.apply(state),
        }
    }
}

impl std::fmt::Display for CornerOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CornerOperation::Swap(op) => write!(f, "{}", op),
            CornerOperation::Twist(op) => write!(f, "{}", op),
        }
    }
}

/// コーナーの置換と向きを完成状態に戻すための操作列を計算
pub struct CornerInspection;

impl CornerInspection {
    /// cp と co を完成状態に戻すための操作列を計算
    ///
    /// # Arguments
    /// * `state` - 現在のキューブ状態（この関数内で変更されない）
    ///
    /// # Returns
    /// コーナー操作の列（Swap と Twist）
    pub fn solve_corner_permutation_with_orientation(state: &State) -> Vec<CornerOperation> {
        // let mut cp = state.cp.clone();
        // let mut co = state.co.clone();
        let mut current_state = state.clone();
        let mut operations = Vec::new();

        loop {
            // cp[0]との二点交換ループ
            while current_state.cp[0] != 0 {
                let target = current_state.cp[0] as usize;
                let ori = current_state.co[0]; // 交換前のco[0]を記録

                let operation = CornerOperation::Swap(CornerSwapOperation::new(0, target, ori));
                operations.push(operation.clone());

                current_state = operation.apply(&current_state);
            }

            // 別ループ探索
            if let Some(next_index) = Self::find_next_misplaced_cp(&current_state.cp) {
                let ori = current_state.co[0]; // 交換前のco[0]を記録

                let operation = CornerOperation::Swap(CornerSwapOperation::new(0, next_index, ori));
                operations.push(operation.clone());

                current_state = operation.apply(&current_state);
            } else {
                // cp の終了条件が満たされた
                break;
            }
        }

        // co 専用の終了処理
        for i in 0..8 {
            if current_state.co[i] != 0 {
                let operation =
                    CornerOperation::Twist(CornerTwistOperation::new(i, current_state.co[i]));

                operations.push(operation.clone());
                current_state = operation.apply(&current_state);
            }
        }

        operations
    }

    /// cp[i] ≠ i となる最小の i を探す
    fn find_next_misplaced_cp(cp: &[u8; 8]) -> Option<usize> {
        for i in 0..8 {
            if cp[i] != i as u8 {
                return Some(i);
            }
        }
        None
    }

    /// 操作列を人間が読みやすい形式で出力
    pub fn format_operations(operations: &[CornerOperation]) -> String {
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
    fn test_cp_only_example() {
        // cp のみの例: [3,1,2,0,5,7,4,6]
        let state = State::new(
            [3, 1, 2, 0, 5, 7, 4, 6],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        assert_eq!(current_state.cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(current_state.co, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_with_orientation() {
        // co も含む例
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [1, 2, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        assert_eq!(current_state.cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(current_state.co, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_twist_only() {
        // cp は完成、co のみ変更が必要
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [1, 2, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);

        // Twist操作のみが含まれるはず
        assert!(operations
            .iter()
            .all(|op| matches!(op, CornerOperation::Twist(_))));

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        assert_eq!(current_state.cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(current_state.co, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_already_solved() {
        let state = State::solved();
        let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);
        assert!(operations.is_empty());
    }

    #[test]
    fn test_complex_case() {
        // 複雑なケース
        let state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== test_complex_case ===");
        println!("Initial state:");
        println!("  cp: {:?}", state.cp);
        println!("  co: {:?}", state.co);

        let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);

        println!("\nOperations:");
        println!("{}", CornerInspection::format_operations(&operations));

        // 初期 state に対して operation を順次 apply して検証
        let mut current_state = state.clone();
        for op in &operations {
            current_state = op.apply(&current_state);
        }

        println!("\nFinal state:");
        println!("  cp: {:?}", current_state.cp);
        println!("  co: {:?}", current_state.co);
        println!("=========================\n");

        assert_eq!(current_state.cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(current_state.co, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }

    #[test]
    fn test_debug_step_by_step() {
        // デバッグ用: ステップごとの状態を出力
        let state = State::new(
            [3, 1, 2, 0, 5, 7, 4, 6],
            [2, 1, 0, 1, 0, 2, 1, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== Debug: Step-by-step execution ===");
        println!("Initial state:");
        println!("  cp: {:?}", state.cp);
        println!("  co: {:?}", state.co);
        println!();

        let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);

        let mut current_state = state.clone();

        for (i, op) in operations.iter().enumerate() {
            println!("Step {}: {}", i + 1, op);
            println!(
                "  Before: cp: {:?}, co: {:?}",
                current_state.cp, current_state.co
            );

            current_state = op.apply(&current_state);

            println!(
                "  After:  cp: {:?}, co: {:?}",
                current_state.cp, current_state.co
            );
            println!();
        }

        println!("Final state:");
        println!("  cp: {:?}", current_state.cp);
        println!("  co: {:?}", current_state.co);
        println!("=== End of debug output ===\n");

        assert_eq!(current_state.cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(current_state.co, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(current_state.is_solved());
    }
}
