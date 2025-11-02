use super::edge_modifier::{
    EdgeFlipModifier, EdgeModifier, EdgeSwapModifier, ModifiedEdgeSequence,
};
use crate::cube::State;
use crate::inspection::{EdgeFlipOperation, EdgeOperation, EdgeSwapOperation};

const BUFFER_PIECE: usize = 6;

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
            let alternatives = self.get_alternative_operations(&self.base_operations[step_index]);

            for alternative in alternatives {
                let mut modified = ModifiedEdgeSequence::new(self.base_operations.clone());
                let modifier = self.create_modifier(step_index, &alternative);
                modified.add_modifier(modifier);

                let operations = modified.get_sequence();
                let final_state = self.apply_operations(initial_state, &operations);

                variants.push((modified, final_state));
            }
        }

        variants
    }

    /// 指定した操作に対する代替案を生成
    fn get_alternative_operations(&self, operation: &EdgeOperation) -> Vec<EdgeOperation> {
        match operation {
            EdgeOperation::Swap(original_op) => self
                .generate_swap_alternatives(original_op)
                .into_iter()
                .map(EdgeOperation::Swap)
                .collect(),
            EdgeOperation::Flip(original_op) => self
                .generate_flip_alternatives(original_op)
                .into_iter()
                .map(EdgeOperation::Flip)
                .collect(),
        }
    }

    /// Swapの代替候補を生成
    ///
    /// target1(BUFFER_PIECE) 固定で、target2=0..11, orientation=0..1 の全組み合わせから元の操作を除外
    fn generate_swap_alternatives(
        &self,
        original: &EdgeSwapOperation,
    ) -> Vec<EdgeSwapOperation> {
        let mut alternatives = Vec::new();

        for target2 in 0..12 {
            for orientation in 0..2 {
                let candidate = EdgeSwapOperation::new(BUFFER_PIECE, target2, orientation);
                if candidate != *original {
                    alternatives.push(candidate);
                }
            }
        }

        alternatives
    }

    /// Flipの代替候補を生成
    ///
    /// target=0..11の全組み合わせから元の操作を除外
    fn generate_flip_alternatives(
        &self,
        original: &EdgeFlipOperation,
    ) -> Vec<EdgeFlipOperation> {
        let mut alternatives = Vec::new();

        for target in 0..12 {
            let candidate = EdgeFlipOperation::new(target);
            if candidate != *original {
                alternatives.push(candidate);
            }
        }

        alternatives
    }

    /// EdgeOperationからEdgeModifierを作成
    fn create_modifier(&self, step: usize, operation: &EdgeOperation) -> EdgeModifier {
        match operation {
            EdgeOperation::Swap(op) => EdgeModifier::Swap(EdgeSwapModifier {
                step,
                modifier: op.clone(),
            }),
            EdgeOperation::Flip(op) => EdgeModifier::Flip(EdgeFlipModifier {
                step,
                modifier: op.clone(),
            }),
        }
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

    /// 元の操作列から2手変更した全てのバリエーションを生成
    ///
    /// # Arguments
    /// * `initial_state` - 操作列を適用する初期状態
    ///
    /// # Returns
    /// タプルのベクトル: (ModifiedEdgeSequence, 最終状態)
    ///
    /// # 注意
    /// - 2つの異なるステップを選択します（同じステップを2回変更することはありません）
    /// - Swap操作はSwapの代替候補に、Flip操作はFlipの代替候補に置き換えられます
    /// - 元の操作と同じ操作は代替候補から除外されます
    pub fn explore_variants_two_changes(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedEdgeSequence, State)> {
        let mut variants = Vec::new();
        let n = self.base_operations.len();

        // 各ステップの代替案を事前に生成
        let alternatives_for_each_step: Vec<Vec<EdgeModifier>> = self
            .base_operations
            .iter()
            .enumerate()
            .map(|(step, op)| {
                self.get_alternative_operations(op)
                    .into_iter()
                    .map(|alt_op| self.create_modifier(step, &alt_op))
                    .collect()
            })
            .collect();

        // i < j となる全てのペアについて
        for i in 0..n {
            for j in (i + 1)..n {
                // 全ての組み合わせを試す
                for alt_i in &alternatives_for_each_step[i] {
                    for alt_j in &alternatives_for_each_step[j] {
                        let mut modified = ModifiedEdgeSequence::new(self.base_operations.clone());
                        modified.add_modifier(alt_i.clone());
                        modified.add_modifier(alt_j.clone());

                        let operations = modified.get_sequence();
                        let final_state = self.apply_operations(initial_state, &operations);

                        variants.push((modified, final_state));
                    }
                }
            }
        }

        variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspection::{EdgeFlipOperation, EdgeInspection};

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
        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        println!("\n=== Original Solution ===");
        let original_modified = ModifiedEdgeSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyEdgeOperationSearch::format_variant(&original_modified)
        );

        // NearbyEdgeOperationSearch を作成
        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());

        // バリエーションを探索
        let variants = searcher.explore_variants(&state);

        println!("\n=== Generated {} variants ===", variants.len());

        // バリエーションの数を確認（元の操作を除外するため23）
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, EdgeOperation::Swap(_)))
            .count();
        assert_eq!(variants.len(), swap_count * 23);
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

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        // Flip のみの操作列のはず
        assert!(original_solution
            .iter()
            .all(|op| matches!(op, EdgeOperation::Flip(_))));

        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants(&state);

        // Flip操作の数 × (代替操作の数 - 1) = Flip操作の数 × 11
        let flip_count = original_solution.len();
        assert_eq!(variants.len(), flip_count * 11);
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

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        println!("\n=== Complex Case Original Solution ===");
        let original_modified = ModifiedEdgeSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyEdgeOperationSearch::format_variant(&original_modified)
        );

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
    fn test_does_not_include_original_operation() {
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);
        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants(&state);

        // 元の操作列と完全に同じバリエーションが含まれていないことを確認
        let original_found = variants.iter().any(|(modified, _)| {
            modified.get_sequence() == original_solution
        });

        assert!(!original_found, "元の操作列と同じバリエーションが含まれています");
    }

    #[test]
    fn test_two_changes_edge_basic() {
        // テスト用の初期状態
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // 元の解法を取得
        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        println!("\n=== Original Solution ===");
        let original_modified = ModifiedEdgeSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyEdgeOperationSearch::format_variant(&original_modified)
        );

        // NearbyEdgeOperationSearch を作成
        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());

        // 2手変更のバリエーションを探索
        let variants = searcher.explore_variants_two_changes(&state);

        println!(
            "\n=== Generated {} two-change edge variants ===",
            variants.len()
        );

        // バリエーションの数を確認（元の操作を除外するため23 × 23, 23 × 11, 11 × 11）
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, EdgeOperation::Swap(_)))
            .count();
        let flip_count = original_solution
            .iter()
            .filter(|op| matches!(op, EdgeOperation::Flip(_)))
            .count();

        // Swap × Swap + Swap × Flip + Flip × Flip
        let expected_count = if swap_count >= 2 {
            (swap_count * (swap_count - 1) / 2) * 23 * 23
        } else {
            0
        } + swap_count * flip_count * 23 * 11
            + if flip_count >= 2 {
                (flip_count * (flip_count - 1) / 2) * 11 * 11
            } else {
                0
            };

        assert_eq!(variants.len(), expected_count);
    }

    #[test]
    fn test_two_changes_edge_complex() {
        // 複雑なケースでのテスト
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 6, 2, 5, 4, 3, 7, 9, 10, 11, 8],
            [1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

        println!("\n=== Complex Case Original Solution ===");
        let original_modified = ModifiedEdgeSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyEdgeOperationSearch::format_variant(&original_modified)
        );

        let searcher = NearbyEdgeOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants_two_changes(&state);

        println!(
            "\n=== Generated {} two-change edge variants for complex case ===",
            variants.len()
        );

        // 解決したバリエーションがあるか確認
        let solved_variants: Vec<_> = variants
            .iter()
            .filter(|(_, final_state)| final_state.is_solved())
            .collect();

        println!(
            "Found {} solved two-change edge variants",
            solved_variants.len()
        );

        if !solved_variants.is_empty() {
            println!("\n--- First few solved two-change edge variants ---");
            for (i, (modified, _)) in solved_variants.iter().take(3).enumerate() {
                println!("\n--- Solved Variant {} ---", i + 1);
                println!("{}", NearbyEdgeOperationSearch::format_variant(modified));
            }
        }
    }

    #[test]
    fn test_two_changes_edge_flip_only() {
        // Flipのみの操作列
        let state = State::solved();

        let original_solution = vec![
            EdgeOperation::Flip(EdgeFlipOperation::new(0)),
            EdgeOperation::Flip(EdgeFlipOperation::new(1)),
            EdgeOperation::Flip(EdgeFlipOperation::new(2)),
        ];

        let searcher = NearbyEdgeOperationSearch::new(original_solution);
        let variants = searcher.explore_variants_two_changes(&state);

        // Flip × Flip: C(3, 2) × 11 × 11 = 3 × 121 = 363
        assert_eq!(variants.len(), 363);
    }

    #[test]
    fn test_two_changes_edge_mixed_comprehensive() {
        // SwapとFlipが混在する複雑なケース
        let state = State::solved();

        let original_solution = vec![
            EdgeOperation::Swap(EdgeSwapOperation::new(6, 0, 0)),
            EdgeOperation::Swap(EdgeSwapOperation::new(6, 1, 0)),
            EdgeOperation::Flip(EdgeFlipOperation::new(0)),
            EdgeOperation::Flip(EdgeFlipOperation::new(3)),
        ];

        let searcher = NearbyEdgeOperationSearch::new(original_solution);
        let variants = searcher.explore_variants_two_changes(&state);

        // Swap × Swap: C(2, 2) × 23 × 23 = 1 × 529 = 529
        // Swap × Flip: 2 × 2 × 23 × 11 = 1,012
        // Flip × Flip: C(2, 2) × 11 × 11 = 1 × 121 = 121
        // 合計: 1,662
        assert_eq!(variants.len(), 1662);
    }
}
