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
            // Twist の場合はスキップ
            if matches!(self.base_operations[step_index], CornerOperation::Twist(_)) {
                continue;
            }

            // Swap の場合、全ての代替操作を生成
            let alternatives = self.generate_alternatives();

            for alt_swap in alternatives {
                // ModifiedSequenceを作成
                let mut modified = ModifiedSequence::new(self.base_operations.clone());
                modified.add_modifier(CornerModifier::Swap(SwapModifier {
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
    /// target1=0 固定で、target2=0..7, orientation=0..2 の全組み合わせ
    fn generate_alternatives(&self) -> Vec<CornerSwapOperation> {
        self.generate_swap_alternatives()
    }

    /// Swapの代替候補を生成
    ///
    /// target1=BUFFER_PIECE 固定で、target2=0..7, orientation=0..2 の全組み合わせ（24パターン）
    fn generate_swap_alternatives(&self) -> Vec<CornerSwapOperation> {
        let mut alternatives = Vec::new();

        for target2 in 0..8 {
            for orientation in 0..3 {
                alternatives.push(CornerSwapOperation::new(BUFFER_PIECE, target2, orientation));
            }
        }

        alternatives
    }

    /// Twistの代替候補を生成
    ///
    /// target=0..7, orientation=1..2（CW, CCW）の全組み合わせ（16パターン）
    fn generate_twist_alternatives(&self) -> Vec<CornerTwistOperation> {
        let mut alternatives = Vec::new();

        for target in 0..8 {
            for orientation in 1..3 {
                alternatives.push(CornerTwistOperation::new(target, orientation));
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
    /// - 生成されるバリエーション数:
    ///   - Swap × Swap: C(s, 2) × 24 × 24
    ///   - Swap × Twist: s × t × 24 × 16
    ///   - Twist × Twist: C(t, 2) × 16 × 16
    ///   (s = Swap操作の数, t = Twist操作の数)
    pub fn explore_variants_two_changes(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedSequence, State)> {
        let mut variants = Vec::new();
        let n = self.base_operations.len();

        // i < j となる全てのペアについて
        for i in 0..n {
            for j in (i + 1)..n {
                // Step i の操作型を判断して代替候補を取得
                let alternatives_i: Vec<CornerModifier> = match &self.base_operations[i] {
                    CornerOperation::Swap(_) => {
                        // Swapの代替候補（24パターン）
                        self.generate_swap_alternatives()
                            .into_iter()
                            .map(|alt| {
                                CornerModifier::Swap(SwapModifier {
                                    step: i,
                                    modifier: alt,
                                })
                            })
                            .collect()
                    }
                    CornerOperation::Twist(_) => {
                        // Twistの代替候補（16パターン）
                        self.generate_twist_alternatives()
                            .into_iter()
                            .map(|alt| {
                                CornerModifier::Twist(TwistModifier {
                                    step: i,
                                    modifier: alt,
                                })
                            })
                            .collect()
                    }
                };

                // Step j の操作型を判断して代替候補を取得
                let alternatives_j: Vec<CornerModifier> = match &self.base_operations[j] {
                    CornerOperation::Swap(_) => self
                        .generate_swap_alternatives()
                        .into_iter()
                        .map(|alt| {
                            CornerModifier::Swap(SwapModifier {
                                step: j,
                                modifier: alt,
                            })
                        })
                        .collect(),
                    CornerOperation::Twist(_) => self
                        .generate_twist_alternatives()
                        .into_iter()
                        .map(|alt| {
                            CornerModifier::Twist(TwistModifier {
                                step: j,
                                modifier: alt,
                            })
                        })
                        .collect(),
                };

                // 全ての組み合わせを試す
                for alt_i in &alternatives_i {
                    for alt_j in &alternatives_j {
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
        // Swap操作の数 × 代替操作の数(8×3=24)
        let swap_count = original_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        assert_eq!(variants.len(), swap_count * 24);

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

        // バリエーションの数を確認
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
            (swap_count * (swap_count - 1) / 2) * 24 * 24
        } else {
            0
        } + swap_count * twist_count * 24 * 16
            + if twist_count >= 2 {
                (twist_count * (twist_count - 1) / 2) * 16 * 16
            } else {
                0
            };

        assert_eq!(variants.len(), expected_count);

        // いくつかのバリエーションを表示（デバッグ用）
        for (i, (modified, final_state)) in variants.iter().take(3).enumerate() {
            println!("\n--- Variant {} ---", i + 1);
            println!("{}", NearbyOperationSearch::format_variant(modified));
            println!("Final state solved: {}", final_state.is_solved());
        }
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
    fn test_two_changes_insufficient_swaps() {
        // Swapが1つしかない場合
        let operations = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)),
            CornerOperation::Twist(CornerTwistOperation::new(0, 1)),
            CornerOperation::Twist(CornerTwistOperation::new(1, 2)),
        ];

        let state = State::solved();
        let searcher = NearbyOperationSearch::new(operations);
        let variants = searcher.explore_variants_two_changes(&state);

        // Swapが2つ未満なのでバリエーションは0個
        assert_eq!(variants.len(), 0);
    }

    #[test]
    fn test_two_changes_example_from_docs() {
        // ドキュメントの例を再現
        let state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let original_solution = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)), // Step 1
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)), // Step 2
            CornerOperation::Swap(CornerSwapOperation::new(0, 6, 0)), // Step 3
        ];

        println!("\n=== Document Example Original Solution ===");
        for (i, op) in original_solution.iter().enumerate() {
            println!("Step {}: {}", i + 1, op);
        }

        let searcher = NearbyOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants_two_changes(&state);

        // C(3, 2) × 24 × 24 = 3 × 24 × 24 = 1728
        assert_eq!(variants.len(), 1728);

        // ドキュメントの例に近いバリエーションを探す
        // Step 2: target2=3, Step 3: target2=5, orientation=1
        let example_found = variants.iter().any(|(modified, _)| {
            let seq = modified.get_sequence();
            if seq.len() != 3 {
                return false;
            }
            matches!(
                (&seq[0], &seq[1], &seq[2]),
                (
                    CornerOperation::Swap(s1),
                    CornerOperation::Swap(s2),
                    CornerOperation::Swap(s3)
                ) if s1.target2 == 1 && s1.orientation == 0
                    && s2.target2 == 3 && s2.orientation == 0
                    && s3.target2 == 5 && s3.orientation == 1
            )
        });

        assert!(
            example_found,
            "ドキュメントの例と同じパターンが見つかりませんでした"
        );

        println!("\n✓ ドキュメントの例と一致するバリエーションが見つかりました！");
    }

    #[test]
    fn test_two_changes_with_twist() {
        // ドキュメントの追記の例：SwapとTwistの組み合わせ
        let state = State::solved(); // ダミー状態

        let original_solution = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)), // Step 1
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)), // Step 2
            CornerOperation::Swap(CornerSwapOperation::new(0, 6, 0)), // Step 3
            CornerOperation::Twist(CornerTwistOperation::new(2, 1)),  // Step 4: Clockwise
        ];

        println!("\n=== Document Example with Twist ===");
        for (i, op) in original_solution.iter().enumerate() {
            println!("Step {}: {}", i + 1, op);
        }

        let searcher = NearbyOperationSearch::new(original_solution.clone());
        let variants = searcher.explore_variants_two_changes(&state);

        // Swap × Swap: C(3, 2) × 24 × 24 = 3 × 576 = 1,728
        // Swap × Twist: 3 × 1 × 24 × 16 = 1,152
        // Twist × Twist: 0 (Twistが1つだけ)
        // 合計: 2,880
        assert_eq!(variants.len(), 2880);

        // ドキュメントの例を探す
        // Step 3: target2=3, Step 4: corner=4, orientation=2(CCW)
        let example_found = variants.iter().any(|(modified, _)| {
            let seq = modified.get_sequence();
            if seq.len() != 4 {
                return false;
            }
            matches!(
                (&seq[0], &seq[1], &seq[2], &seq[3]),
                (
                    CornerOperation::Swap(s1),
                    CornerOperation::Swap(s2),
                    CornerOperation::Swap(s3),
                    CornerOperation::Twist(t)
                ) if s1.target2 == 1 && s1.orientation == 0
                    && s2.target2 == 2 && s2.orientation == 0
                    && s3.target2 == 3 && s3.orientation == 0
                    && t.target == 4 && t.orientation == 2
            )
        });

        assert!(
            example_found,
            "ドキュメントの追記の例と同じパターンが見つかりませんでした"
        );

        println!("\n✓ ドキュメントの追記の例と一致するバリエーションが見つかりました！");
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

        // Twist × Twist: C(3, 2) × 16 × 16 = 3 × 256 = 768
        assert_eq!(variants.len(), 768);
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

        // Swap × Swap: C(2, 2) × 24 × 24 = 1 × 576 = 576
        // Swap × Twist: 2 × 2 × 24 × 16 = 1,536
        // Twist × Twist: C(2, 2) × 16 × 16 = 1 × 256 = 256
        // 合計: 2,368
        assert_eq!(variants.len(), 2368);
    }
}
