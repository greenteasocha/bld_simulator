use crate::cube::State;
use crate::inspection::{EdgeOperation, EdgeSwapOperation};
use super::edge_modifier::{ModifiedEdgeSequence, EdgeSwapModifier, EdgeModifier};

/// エッジ操作列の近傍を探索する構造体
///
/// 与えられた操作列から1ステップだけ変更したバリエーションを生成する
pub struct NearbyEdgeOperationSearch {
    /// 元の操作列
    base_operations: Vec<EdgeOperation>,
}

impl NearbyEdgeOperationSearch {
    /// 新しい NearbyEdgeOperationSearch を作成
    pub fn new(base_operations: Vec<EdgeOperation>) -> Self {
        Self { base_operations }
    }

    /// 元の操作列から1手だけ変更した全てのバリエーションを生成
    ///
    /// # Arguments
    /// * `initial_state` - 操作列を適用する初期状態
    ///
    /// # Returns
    /// タプルのベクトル: (ModifiedEdgeSequence, 最終状態)
    pub fn explore_variants(&self, initial_state: &State) -> Vec<(ModifiedEdgeSequence, State)> {
        let mut variants = Vec::new();

        // 各ステップについて
        for step_index in 0..self.base_operations.len() {
            // Flip の場合はスキップ
            if matches!(self.base_operations[step_index], EdgeOperation::Flip(_)) {
                continue;
            }

            // Swap の場合、全ての代替操作を生成
            let alternatives = self.generate_alternatives();

            for alt_swap in alternatives {
                // ModifiedEdgeSequenceを作成
                let mut modified = ModifiedEdgeSequence::new(self.base_operations.clone());
                modified.add_modifier(EdgeModifier::Swap(EdgeSwapModifier {
                    step: step_index,
                    modifier: alt_swap,
                }));

                // 変更後の操作列を取得
                let operations = modified.get_sequence();

                // 変更後の操作列を初期状態に適用
                let final_state = self.apply_operations(initial_state, &operations);

                variants.push((modified, final_state));
            }
        }

        variants
    }

    /// 全ての代替Swap操作を生成
    ///
    /// target1=8 (buffer) 固定で、target2=0..11, orientation=0..1 の全組み合わせ
    fn generate_alternatives(&self) -> Vec<EdgeSwapOperation> {
        let mut alternatives = Vec::new();

        for target2 in 0..12 {
            for orientation in 0..2 {
                alternatives.push(EdgeSwapOperation::new(8, target2, orientation));
            }
        }

        alternatives
    }

    /// 操作列を状態に順次適用
    fn apply_operations(&self, initial_state: &State, operations: &[EdgeOperation]) -> State {
        let mut current_state = initial_state.clone();
        for op in operations {
            current_state = op.apply(&current_state);
        }
        current_state
    }

    /// バリエーションを人間が読みやすい形式でフォーマット
    pub fn format_variant(modified: &ModifiedEdgeSequence) -> String {
        format!("{}", modified)
    }

    /// 元の操作列への参照を取得
    pub fn base_operations(&self) -> &[EdgeOperation] {
        &self.base_operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspection::{EdgeInspection, EdgeFlipOperation};

    #[test]
    fn test_nearby_edge_search_basic() {
        // テスト用の初期状態
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // 元の解法を取得
        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state);

        println!("\n=== Original Solution ===");
        let original_modified = ModifiedEdgeSequence::new(original_solution.clone());
        println!("{}", NearbyEdgeOperationSearch::format_variant(&original_modified));

        // NearbyEdgeOperationSearch を作成
        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());

        // バリエーションを探索
        let variants = searcher.explore_variants(&state);

        println!("\n=== Generated {} variants ===", variants.len());

        // バリエーションの数を確認
        // Swap操作の数 × 代替操作の数(12×2=24)
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, EdgeOperation::Swap(_)))
            .count();
        assert_eq!(variants.len(), swap_count * 24);

        // いくつかのバリエーションを表示（デバッグ用）
        for (i, (modified, final_state)) in variants.iter().take(3).enumerate() {
            println!("\n--- Variant {} ---", i + 1);
            println!("{}", NearbyEdgeOperationSearch::format_variant(modified));
            println!("Final state solved: {}", final_state.is_solved());
        }
    }

    #[test]
    fn test_nearby_edge_search_with_flip() {
        // Flipを含む操作列のテスト
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state);

        // Flip のみの操作列のはず
        assert!(original_solution
            .iter()
            .all(|op| matches!(op, EdgeOperation::Flip(_))));

        let searcher = NearbyEdgeOperationSearch::new(original_solution);
        let variants = searcher.explore_variants(&state);

        // Flip はスキップされるので、バリエーションは0個
        assert_eq!(variants.len(), 0);
    }

    #[test]
    fn test_nearby_edge_search_complex() {
        // 複雑なケースでのテスト
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 6, 2, 5, 4, 3, 7, 9, 10, 11, 8],
            [1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state);

        println!("\n=== Complex Case Original Solution ===");
        let original_modified = ModifiedEdgeSequence::new(original_solution.clone());
        println!("{}", NearbyEdgeOperationSearch::format_variant(&original_modified));

        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());
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
            for (i, (modified, final_state)) in solved_variants.iter().enumerate() {
                println!("\n--- Solved Variant {} ---", i + 1);
                println!("{}", NearbyEdgeOperationSearch::format_variant(modified));
                println!("Final state solved: {}", final_state.is_solved());
            }
        }
    }

    #[test]
    fn test_alternatives_generation() {
        let searcher = NearbyEdgeOperationSearch::new(vec![]);
        let alternatives = searcher.generate_alternatives();

        // 12 targets × 2 orientations = 24
        assert_eq!(alternatives.len(), 24);

        // 全てのtarget1が8 (buffer)であることを確認
        assert!(alternatives.iter().all(|alt| alt.target1 == 8));

        // target2が0..11の範囲であることを確認
        assert!(alternatives.iter().all(|alt| alt.target2 < 12));

        // orientationが0..1の範囲であることを確認
        assert!(alternatives.iter().all(|alt| alt.orientation < 2));
    }
}
