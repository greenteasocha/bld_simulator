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
        self.generate_swap_alternatives()
    }

    /// Swapの代替候補を生成
    ///
    /// target1(BUFFER_PIECE) 固定で、target2=0..11, orientation=0..1 の全組み合わせ（24パターン）
    fn generate_swap_alternatives(&self) -> Vec<EdgeSwapOperation> {
        let mut alternatives = Vec::new();

        for target2 in 0..12 {
            for orientation in 0..2 {
                alternatives.push(EdgeSwapOperation::new(BUFFER_PIECE, target2, orientation));
            }
        }

        alternatives
    }

    /// Flipの代替候補を生成
    ///
    /// target=0..11の全組み合わせ（12パターン）
    fn generate_flip_alternatives(&self) -> Vec<EdgeFlipOperation> {
        let mut alternatives = Vec::new();

        for target in 0..12 {
            alternatives.push(EdgeFlipOperation::new(target));
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
    /// - 生成されるバリエーション数:
    ///   - Swap × Swap: C(s, 2) × 24 × 24
    ///   - Swap × Flip: s × f × 24 × 12
    ///   - Flip × Flip: C(f, 2) × 12 × 12
    ///   (s = Swap操作の数, f = Flip操作の数)
    pub fn explore_variants_two_changes(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedEdgeSequence, State)> {
        let mut variants = Vec::new();
        let n = self.base_operations.len();

        // i < j となる全てのペアについて
        for i in 0..n {
            for j in (i + 1)..n {
                // Step i の操作型を判断して代替候補を取得
                let alternatives_i: Vec<EdgeModifier> = match &self.base_operations[i] {
                    EdgeOperation::Swap(_) => {
                        // Swapの代替候補（24パターン）
                        self.generate_swap_alternatives()
                            .into_iter()
                            .map(|alt| {
                                EdgeModifier::Swap(EdgeSwapModifier {
                                    step: i,
                                    modifier: alt,
                                })
                            })
                            .collect()
                    }
                    EdgeOperation::Flip(_) => {
                        // Flipの代替候補（12パターン）
                        self.generate_flip_alternatives()
                            .into_iter()
                            .map(|alt| {
                                EdgeModifier::Flip(EdgeFlipModifier {
                                    step: i,
                                    modifier: alt,
                                })
                            })
                            .collect()
                    }
                };

                // Step j の操作型を判断して代替候補を取得
                let alternatives_j: Vec<EdgeModifier> = match &self.base_operations[j] {
                    EdgeOperation::Swap(_) => self
                        .generate_swap_alternatives()
                        .into_iter()
                        .map(|alt| {
                            EdgeModifier::Swap(EdgeSwapModifier {
                                step: j,
                                modifier: alt,
                            })
                        })
                        .collect(),
                    EdgeOperation::Flip(_) => self
                        .generate_flip_alternatives()
                        .into_iter()
                        .map(|alt| {
                            EdgeModifier::Flip(EdgeFlipModifier {
                                step: j,
                                modifier: alt,
                            })
                        })
                        .collect(),
                };

                // 全ての組み合わせを試す
                for alt_i in &alternatives_i {
                    for alt_j in &alternatives_j {
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

        let original_solution = EdgeInspection::solve_edge_permutation_with_orientation(&state, false);

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

        // バリエーションの数を確認
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
            (swap_count * (swap_count - 1) / 2) * 24 * 24
        } else {
            0
        } + swap_count * flip_count * 24 * 12
            + if flip_count >= 2 {
                (flip_count * (flip_count - 1) / 2) * 12 * 12
            } else {
                0
            };

        assert_eq!(variants.len(), expected_count);

        // いくつかのバリエーションを表示（デバッグ用）
        for (i, (modified, final_state)) in variants.iter().take(3).enumerate() {
            println!("\n--- Variant {} ---", i + 1);
            println!("{}", NearbyEdgeOperationSearch::format_variant(modified));
            println!("Final state solved: {}", final_state.is_solved());
        }
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
    fn test_two_changes_edge_insufficient_swaps() {
        // Swapが1つしかない場合
        let operations = vec![
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 1, 0)),
            EdgeOperation::Flip(EdgeFlipOperation::new(0)),
            EdgeOperation::Flip(EdgeFlipOperation::new(1)),
        ];

        let state = State::solved();
        let searcher = NearbyEdgeOperationSearch::new(operations);
        let variants = searcher.explore_variants_two_changes(&state);

        // Swapが2つ未満なのでバリエーションは0個
        assert_eq!(variants.len(), 0);
    }
}
