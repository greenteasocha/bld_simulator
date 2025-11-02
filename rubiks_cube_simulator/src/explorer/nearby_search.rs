use super::modifier::{CornerModifier, ModifiedSequence, SwapModifier, TwistModifier};
use crate::cube::State;
use crate::inspection::{CornerOperation, CornerSwapOperation, CornerTwistOperation};

/// 操作列の近傍を探索する構造体
///
/// 与えられた操作列から1ステップだけ変更したバリエーションを生成する
pub struct NearbyOperationSearch {
    /// 元の操作列
    base_operations: Vec<CornerOperation>,
}

const BUFFER_PIECE: usize = 2; // コーナー操作のバッファ位置

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
    /// タプルのベクトル: (ModifiedSequence, 最終状態)
    pub fn explore_variants(&self, initial_state: &State) -> Vec<(ModifiedSequence, State)> {
        let mut variants = Vec::new();

        // 各ステップについて
        for step_index in 0..self.base_operations.len() {
            let alternatives = self.get_alternative_operations(&self.base_operations[step_index]);

            for alternative in alternatives {
                let mut modified = ModifiedSequence::new(self.base_operations.clone());
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
    fn get_alternative_operations(&self, operation: &CornerOperation) -> Vec<CornerOperation> {
        match operation {
            CornerOperation::Swap(original_op) => self
                .generate_swap_alternatives(original_op)
                .into_iter()
                .map(CornerOperation::Swap)
                .collect(),
            CornerOperation::Twist(original_op) => self
                .generate_twist_alternatives(original_op)
                .into_iter()
                .map(CornerOperation::Twist)
                .collect(),
        }
    }

    /// Swapの代替候補を生成
    ///
    /// target1=BUFFER_PIECE 固定で、target2=0..7, orientation=0..2 の全組み合わせから元の操作を除外
    fn generate_swap_alternatives(
        &self,
        original: &CornerSwapOperation,
    ) -> Vec<CornerSwapOperation> {
        let mut alternatives = Vec::new();

        for target2 in 0..8 {
            for orientation in 0..3 {
                let candidate = CornerSwapOperation::new(BUFFER_PIECE, target2, orientation);
                if candidate != *original {
                    alternatives.push(candidate);
                }
            }
        }

        alternatives
    }

    /// Twistの代替候補を生成
    ///
    /// target=0..7, orientation=1..2（CW, CCW）の全組み合わせから元の操作を除外
    fn generate_twist_alternatives(
        &self,
        original: &CornerTwistOperation,
    ) -> Vec<CornerTwistOperation> {
        let mut alternatives = Vec::new();

        for target in 0..8 {
            for orientation in 1..3 {
                let candidate = CornerTwistOperation::new(target, orientation);
                if candidate != *original {
                    alternatives.push(candidate);
                }
            }
        }

        alternatives
    }

    /// CornerOperationからCornerModifierを作成
    fn create_modifier(&self, step: usize, operation: &CornerOperation) -> CornerModifier {
        match operation {
            CornerOperation::Swap(op) => CornerModifier::Swap(SwapModifier {
                step,
                modifier: op.clone(),
            }),
            CornerOperation::Twist(op) => CornerModifier::Twist(TwistModifier {
                step,
                modifier: op.clone(),
            }),
        }
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
    pub fn format_variant(modified: &ModifiedSequence) -> String {
        format!("{}", modified)
    }

    /// 元の操作列への参照を取得
    pub fn base_operations(&self) -> &[CornerOperation] {
        &self.base_operations
    }

    /// 元の操作列から2手変更した全てのバリエーションを生成
    ///
    /// # Arguments
    /// * `initial_state` - 操作列を適用する初期状態
    ///
    /// # Returns
    /// タプルのベクトル: (ModifiedSequence, 最終状態)
    ///
    /// # 注意
    /// - 2つの異なるステップを選択します（同じステップを2回変更することはありません）
    /// - Swap操作はSwapの代替候補に、Twist操作はTwistの代替候補に置き換えられます
    /// - 元の操作と同じ操作は代替候補から除外されます
    pub fn explore_variants_two_changes(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedSequence, State)> {
        let mut variants = Vec::new();
        let n = self.base_operations.len();

        // 各ステップの代替案を事前に生成
        let alternatives_for_each_step: Vec<Vec<CornerModifier>> = self
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
                        let mut modified = ModifiedSequence::new(self.base_operations.clone());
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
        let original_modified = ModifiedSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyOperationSearch::format_variant(&original_modified)
        );

        // NearbyOperationSearch を作成
        let searcher = NearbyOperationSearch::new(original_solution.clone());

        // バリエーションを探索
        let variants = searcher.explore_variants(&state);

        println!("\n=== Generated {} variants ===", variants.len());

        // バリエーションの数を確認
        // Swap操作の数 × (代替操作の数 - 1) = Swap操作の数 × 23
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        assert_eq!(variants.len(), swap_count * 23);

        // いくつかのバリエーションを表示（デバッグ用）
        for (i, (modified, final_state)) in variants.iter().take(3).enumerate() {
            println!("\n--- Variant {} ---", i + 1);
            println!("{}", NearbyOperationSearch::format_variant(modified));
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

        let searcher = NearbyOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants(&state);

        // Twist操作の数 × (代替操作の数 - 1) = Twist操作の数 × 15
        let twist_count = original_solution.len();
        assert_eq!(variants.len(), twist_count * 15);
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
        let original_modified = ModifiedSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyOperationSearch::format_variant(&original_modified)
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
            for (i, (modified, final_state)) in solved_variants.iter().enumerate() {
                println!("\n--- Solved Variant {} ---", i + 1);
                println!("{}", NearbyOperationSearch::format_variant(modified));
                println!("Final state solved: {}", final_state.is_solved());
            }
        }
    }

    #[test]
    fn test_does_not_include_original_operation() {
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = CornerInspection::solve_corner_permutation_with_orientation(&state);
        let searcher = NearbyOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants(&state);

        // 元の操作列と完全に同じバリエーションが含まれていないことを確認
        let original_found = variants.iter().any(|(modified, _)| {
            modified.get_sequence() == original_solution
        });

        assert!(!original_found, "元の操作列と同じバリエーションが含まれています");
    }

    #[test]
    fn test_two_changes_basic() {
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
        let original_modified = ModifiedSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyOperationSearch::format_variant(&original_modified)
        );

        // NearbyOperationSearch を作成
        let searcher = NearbyOperationSearch::new(original_solution.clone());

        // 2手変更のバリエーションを探索
        let variants = searcher.explore_variants_two_changes(&state);

        println!("\n=== Generated {} two-change variants ===", variants.len());

        // バリエーションの数を確認（元の操作を除外するため23 × 23）
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        let twist_count = original_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Twist(_)))
            .count();

        // Swap × Swap + Swap × Twist + Twist × Twist
        let expected_count = if swap_count >= 2 {
            (swap_count * (swap_count - 1) / 2) * 23 * 23
        } else {
            0
        } + swap_count * twist_count * 23 * 15
            + if twist_count >= 2 {
                (twist_count * (twist_count - 1) / 2) * 15 * 15
            } else {
                0
            };

        assert_eq!(variants.len(), expected_count);
    }

    #[test]
    fn test_two_changes_complex() {
        // 複雑なケースでのテスト
        let state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = CornerInspection::solve_corner_permutation_with_orientation(&state);

        println!("\n=== Complex Case Original Solution ===");
        let original_modified = ModifiedSequence::new(original_solution.clone());
        println!(
            "{}",
            NearbyOperationSearch::format_variant(&original_modified)
        );

        let searcher = NearbyOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants_two_changes(&state);

        println!(
            "\n=== Generated {} two-change variants for complex case ===",
            variants.len()
        );

        // 解決したバリエーションがあるか確認
        let solved_variants: Vec<_> = variants
            .iter()
            .filter(|(_, final_state)| final_state.is_solved())
            .collect();

        println!("Found {} solved two-change variants", solved_variants.len());

        if !solved_variants.is_empty() {
            println!("\n--- First few solved two-change variants ---");
            for (i, (modified, _)) in solved_variants.iter().take(3).enumerate() {
                println!("\n--- Solved Variant {} ---", i + 1);
                println!("{}", NearbyOperationSearch::format_variant(modified));
            }
        }
    }

    #[test]
    fn test_two_changes_twist_only() {
        // Twistのみの操作列
        let state = State::solved();

        let original_solution = vec![
            CornerOperation::Twist(CornerTwistOperation::new(0, 1)),
            CornerOperation::Twist(CornerTwistOperation::new(1, 2)),
            CornerOperation::Twist(CornerTwistOperation::new(2, 1)),
        ];

        let searcher = NearbyOperationSearch::new(original_solution);
        let variants = searcher.explore_variants_two_changes(&state);

        // Twist × Twist: C(3, 2) × 15 × 15 = 3 × 225 = 675
        assert_eq!(variants.len(), 675);
    }

    #[test]
    fn test_two_changes_mixed_comprehensive() {
        // SwapとTwistが混在する複雑なケース
        let state = State::solved();

        let original_solution = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)),
            CornerOperation::Twist(CornerTwistOperation::new(0, 1)),
            CornerOperation::Twist(CornerTwistOperation::new(3, 2)),
        ];

        let searcher = NearbyOperationSearch::new(original_solution);
        let variants = searcher.explore_variants_two_changes(&state);

        // Swap × Swap: C(2, 2) × 23 × 23 = 1 × 529 = 529
        // Swap × Twist: 2 × 2 × 23 × 15 = 1,380
        // Twist × Twist: C(2, 2) × 15 × 15 = 1 × 225 = 225
        // 合計: 2,134
        assert_eq!(variants.len(), 2134);
    }
}
