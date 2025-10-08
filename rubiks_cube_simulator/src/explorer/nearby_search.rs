use crate::cube::State;
use crate::inspection::{CornerOperation, CornerSwapOperation};

/// 操作列の近傍を探索する構造体
///
/// 与えられた操作列から1ステップだけ変更したバリエーションを生成する
pub struct NearbyOperationSearch {
    /// 元の操作列
    base_operations: Vec<CornerOperation>,
}

impl NearbyOperationSearch {
    /// 新しい NearbyOperationSearch を作成
    pub fn new(base_operations: Vec<CornerOperation>) -> Self {
        Self { base_operations }
    }

    /// 元の操作列から1手だけ変更した全てのバリエーションを生成
    ///
    /// # Arguments
    /// * `initial_state` - 操作列を適用する初期状態
    ///
    /// # Returns
    /// タプルのベクトル: (変更された操作列, 最終状態)
    pub fn explore_variants(&self, initial_state: &State) -> Vec<(Vec<CornerOperation>, State)> {
        let mut variants = Vec::new();

        // 各ステップについて
        for step_index in 0..self.base_operations.len() {
            // Twist の場合はスキップ
            if matches!(self.base_operations[step_index], CornerOperation::Twist(_)) {
                continue;
            }

            // Swap の場合、全ての代替操作を生成
            let alternatives = self.generate_alternatives();

            for alt_swap in alternatives {
                // 操作列を複製して、step_index の位置だけ変更
                let mut modified_ops = self.base_operations.clone();
                modified_ops[step_index] = CornerOperation::Swap(alt_swap);

                // 変更後の操作列を初期状態に適用
                let final_state = self.apply_operations(initial_state, &modified_ops);

                variants.push((modified_ops, final_state));
            }
        }

        variants
    }

    /// 全ての代替Swap操作を生成
    ///
    /// target1=0 固定で、target2=0..7, orientation=0..2 の全組み合わせ
    fn generate_alternatives(&self) -> Vec<CornerSwapOperation> {
        let mut alternatives = Vec::new();

        for target2 in 0..8 {
            for orientation in 0..3 {
                alternatives.push(CornerSwapOperation::new(0, target2, orientation));
            }
        }

        alternatives
    }

    /// 操作列を状態に順次適用
    fn apply_operations(&self, initial_state: &State, operations: &[CornerOperation]) -> State {
        let mut current_state = initial_state.clone();
        for op in operations {
            current_state = op.apply(&current_state);
        }
        current_state
    }

    /// バリエーションを人間が読みやすい形式でフォーマット
    pub fn format_variant(operations: &[CornerOperation]) -> String {
        operations
            .iter()
            .enumerate()
            .map(|(i, op)| format!("Step {}: {}", i + 1, op))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 元の操作列への参照を取得
    pub fn base_operations(&self) -> &[CornerOperation] {
        &self.base_operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspection::{CornerInspection, CornerTwistOperation};

    #[test]
    fn test_nearby_search_basic() {
        // テスト用の初期状態
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // 元の解法を取得
        let original_solution = CornerInspection::solve_corner_permutation_with_orientation(&state);

        println!("\n=== Original Solution ===");
        println!(
            "{}",
            NearbyOperationSearch::format_variant(&original_solution)
        );

        // NearbyOperationSearch を作成
        let searcher = NearbyOperationSearch::new(original_solution.clone());

        // バリエーションを探索
        let variants = searcher.explore_variants(&state);

        println!("\n=== Generated {} variants ===", variants.len());

        // バリエーションの数を確認
        // Swap操作の数 × 代替操作の数(8×3=24)
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        assert_eq!(variants.len(), swap_count * 24);

        // いくつかのバリエーションを表示（デバッグ用）
        for (i, (ops, final_state)) in variants.iter().take(3).enumerate() {
            println!("\n--- Variant {} ---", i + 1);
            println!("{}", NearbyOperationSearch::format_variant(ops));
            println!("Final state solved: {}", final_state.is_solved());
        }
    }

    #[test]
    fn test_nearby_search_with_twist() {
        // Twistを含む操作列のテスト
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [1, 2, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = CornerInspection::solve_corner_permutation_with_orientation(&state);

        // Twist のみの操作列のはず
        assert!(original_solution
            .iter()
            .all(|op| matches!(op, CornerOperation::Twist(_))));

        let searcher = NearbyOperationSearch::new(original_solution);
        let variants = searcher.explore_variants(&state);

        // Twist はスキップされるので、バリエーションは0個
        assert_eq!(variants.len(), 0);
    }

    #[test]
    fn test_nearby_search_complex() {
        // 複雑なケースでのテスト
        let state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = CornerInspection::solve_corner_permutation_with_orientation(&state);

        println!("\n=== Complex Case Original Solution ===");
        println!(
            "{}",
            NearbyOperationSearch::format_variant(&original_solution)
        );

        let searcher = NearbyOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants(&state);

        println!(
            "\n=== Generated {} variants for complex case ===",
            variants.len()
        );

        // 解決したバリエーションがあるか確認
        let solved_variants: Vec<_> = variants
            .iter()
            .filter(|(_, final_state)| final_state.is_solved())
            .collect();

        println!("Found {} solved variants", solved_variants.len());

        if !solved_variants.is_empty() {
            println!("\n--- All solved variant ---");
            for (i, (ops, final_state)) in solved_variants.iter().enumerate() {
                println!("\n--- Solved Variant {} ---", i + 1);
                println!("{}", NearbyOperationSearch::format_variant(ops));
                println!("Final state solved: {}", final_state.is_solved());
            }
        }
    }

    #[test]
    fn test_alternatives_generation() {
        let searcher = NearbyOperationSearch::new(vec![]);
        let alternatives = searcher.generate_alternatives();

        // 8 targets × 3 orientations = 24
        assert_eq!(alternatives.len(), 24);

        // 全てのtarget1が0であることを確認
        assert!(alternatives.iter().all(|alt| alt.target1 == 0));

        // target2が0..7の範囲であることを確認
        assert!(alternatives.iter().all(|alt| alt.target2 < 8));

        // orientationが0..2の範囲であることを確認
        assert!(alternatives.iter().all(|alt| alt.orientation < 3));
    }

    #[test]
    fn test_alternatives_generation_debug() {
        let state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        /*
        apply these operations to the state above:
        Step 1: Swap: 0 ↔ 1 (ori: 0)
        Step 2: Swap: 0 ↔ 2 (ori: 0)
        Step 3: Swap: 0 ↔ 6 (ori: 0)
        Step 4: Swap: 0 ↔ 3 (ori: 2)
        Step 5: Swap: 0 ↔ 2 (ori: 1)
        Step 6: Swap: 0 ↔ 4 (ori: 1)
        Step 7: Swap: 0 ↔ 5 (ori: 0)
        Step 8: Swap: 0 ↔ 4 (ori: 2)
        Step 9: Twist: corner[0] (ori: 2)
        Step 10: Twist: corner[7] (ori: 1)
        Final state solved: true
             */

        let original_solution = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 2)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 6, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 3, 2)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 1)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 4, 1)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 5, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 4, 2)),
            CornerOperation::Twist(CornerTwistOperation::new(0, 2)),
            CornerOperation::Twist(CornerTwistOperation::new(7, 1)),
        ];

        // for each step of solution, apply operation and print state
        let mut current_state = state.clone();
        for (i, op) in original_solution.iter().enumerate() {
            current_state = op.apply(&current_state);
            println!("Step {}: {}", i + 1, op);
            // print cp and co
            println!("  cp: {:?}", current_state.cp);
            println!("  co: {:?}", current_state.co);
        }
    }
}
