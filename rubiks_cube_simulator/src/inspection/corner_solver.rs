use crate::cube::State;

/// コーナーの2点交換操作を表す（co考慮版）
#[derive(Debug, Clone, PartialEq)]
pub struct CornerSwapOperation {
    /// 交換する2つのインデックス
    pub index1: usize,
    pub index2: usize,
    /// 交換操作を行う前の co[index1] を記録
    pub orientation: u8,
}

impl CornerSwapOperation {
    pub fn new(index1: usize, index2: usize, orientation: u8) -> Self {
        Self {
            index1,
            index2,
            orientation,
        }
    }
}

impl std::fmt::Display for CornerSwapOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ↔ {} (ori: {})", self.index1, self.index2, self.orientation)
    }
}

/// コーナーの向き変更操作を表す
#[derive(Debug, Clone, PartialEq)]
pub struct CornerTwistOperation {
    /// 対象のインデックス
    pub index: usize,
    /// 向きの値 (1 or 2)
    pub co: u8,
}

impl std::fmt::Display for CornerTwistOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Twist corner[{}] (co: {})", self.index, self.co)
    }
}

/// 操作の種類
#[derive(Debug, Clone, PartialEq)]
pub enum CornerOperation {
    Swap(CornerSwapOperation),
    Twist(CornerTwistOperation),
}

impl std::fmt::Display for CornerOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CornerOperation::Swap(op) => write!(f, "Swap: {}", op),
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
    /// * `state` - 現在のキューブ状態
    /// 
    /// # Returns
    /// コーナー操作の列（Swap と Twist）
    pub fn solve_corner_permutation_with_orientation(state: &State) -> Vec<CornerOperation> {
        let mut cp = state.cp.clone();
        let mut co = state.co.clone();
        let mut operations = Vec::new();
        
        loop {
            // cp[0]との二点交換ループ
            while cp[0] != 0 {
                let target = cp[0] as usize;
                let ori = co[0]; // 交換前のco[0]を記録
                
                operations.push(CornerOperation::Swap(
                    CornerSwapOperation::new(0, target, ori)
                ));
                
                // cp の交換
                cp.swap(0, target);
                
                // co の変化: co[0], co[target] = (co[0] + co[target]) % 3, 0
                let new_co0 = (co[0] + co[target]) % 3;
                co[target] = 0;
                co[0] = new_co0;
            }
            
            // 別ループ探索
            if let Some(next_index) = Self::find_next_misplaced_cp(&cp) {
                let ori = co[0]; // 交換前のco[0]を記録
                
                operations.push(CornerOperation::Swap(
                    CornerSwapOperation::new(0, next_index, ori)
                ));
                
                // cp の交換
                cp.swap(0, next_index);
                
                // co の変化
                let new_co0 = (co[0] + co[next_index]) % 3;
                co[next_index] = 0;
                co[0] = new_co0;
            } else {
                // cp の終了条件が満たされた
                break;
            }
        }
        
        // co 専用の終了処理
        for i in 0..8 {
            if co[i] != 0 {
                operations.push(CornerOperation::Twist(
                    CornerTwistOperation { 
                        index: i,
                        co: co[i] 
                    }
                ));
                co[i] = 0;
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
    
    /// 操作列を適用した結果の cp, co を計算（検証用）
    pub fn apply_operations(
        initial_cp: [u8; 8],
        initial_co: [u8; 8],
        operations: &[CornerOperation]
    ) -> ([u8; 8], [u8; 8]) {
        let mut cp = initial_cp;
        let mut co = initial_co;
        
        for op in operations {
            match op {
                CornerOperation::Swap(swap_op) => {
                    let i1 = swap_op.index1;
                    let i2 = swap_op.index2;
                    
                    // cp の交換
                    cp.swap(i1, i2);
                    
                    // co の変化
                    let new_co_i1 = (co[i1] + co[i2]) % 3;
                    co[i2] = 0;
                    co[i1] = new_co_i1;
                }
                CornerOperation::Twist(twist_op) => {
                    // Twist操作はco[index]を0にする
                    co[twist_op.index] = 0;
                }
            }
        }
        
        (cp, co)
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
        
        // 結果が完成状態になることを確認
        let (result_cp, result_co) = CornerInspection::apply_operations(
            state.cp, state.co, &operations
        );
        assert_eq!(result_cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result_co, [0, 0, 0, 0, 0, 0, 0, 0]);
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
        
        // 結果が完成状態になることを確認
        let (result_cp, result_co) = CornerInspection::apply_operations(
            state.cp, state.co, &operations
        );
        assert_eq!(result_cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result_co, [0, 0, 0, 0, 0, 0, 0, 0]);
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
        assert!(operations.iter().all(|op| matches!(op, CornerOperation::Twist(_))));
        
        // 結果が完成状態になることを確認
        let (result_cp, result_co) = CornerInspection::apply_operations(
            state.cp, state.co, &operations
        );
        assert_eq!(result_cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result_co, [0, 0, 0, 0, 0, 0, 0, 0]);
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
            [3, 1, 2, 0, 5, 7, 4, 6],
            [2, 1, 0, 1, 0, 2, 1, 0],
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
        
        let (result_cp, result_co) = CornerInspection::apply_operations(
            state.cp, state.co, &operations
        );
        
        println!("\nFinal state:");
        println!("  cp: {:?}", result_cp);
        println!("  co: {:?}", result_co);
        println!("=========================\n");
        
        assert_eq!(result_cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result_co, [0, 0, 0, 0, 0, 0, 0, 0]);
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
        
        let mut cp = state.cp;
        let mut co = state.co;
        
        for (i, op) in operations.iter().enumerate() {
            println!("Step {}: {}", i + 1, op);
            
            match op {
                CornerOperation::Swap(swap_op) => {
                    let i1 = swap_op.index1;
                    let i2 = swap_op.index2;
                    
                    println!("  Before: cp: {:?}, co: {:?}", cp, co);
                    
                    // cp の交換
                    cp.swap(i1, i2);
                    
                    // co の変化
                    let new_co_i1 = (co[i1] + co[i2]) % 3;
                    co[i2] = 0;
                    co[i1] = new_co_i1;
                    
                    println!("  After:  cp: {:?}, co: {:?}", cp, co);
                }
                CornerOperation::Twist(twist_op) => {
                    println!("  Before: co: {:?}", co);
                    
                    // Twist操作
                    co[twist_op.index] = 0;
                    println!("  Twisted co[{}] from {} to 0", twist_op.index, twist_op.co);
                    
                    println!("  After:  co: {:?}", co);
                }
            }
            println!();
        }
        
        println!("Final state:");
        println!("  cp: {:?}", cp);
        println!("  co: {:?}", co);
        println!("=== End of debug output ===\n");
        
        assert_eq!(cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(co, [0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
