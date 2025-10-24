use crate::cube::State;
use crate::explorer::modifier::ModifiedSequence;
use crate::explorer::NearbyOperationSearch;
use crate::inspection::{CornerInspection, CornerOperation};

/// ユーザーの操作ミスを検出して提案する機能
///
/// 完成状態を目指したが、誤って近傍の状態に到達してしまった場合に、
/// どの操作列を適用したのかを推測して提示する
pub struct WrongOperationDetector {
    initial_state: State,
    correct_solution: Vec<CornerOperation>,
    nearby_variants: Vec<(ModifiedSequence, State)>,
}

impl WrongOperationDetector {
    /// 新しい WrongOperationDetector を作成（2操作変更検出版）
    ///
    /// # Arguments
    /// * `initial_state` - 初期状態
    ///
    /// # Returns
    /// 初期化された WrongOperationDetector
    ///
    /// # 注意
    /// このバージョンでは、正解の操作列から2つの操作を変更したバリエーションを検出します。
    /// 1つの操作のみを変更するバリエーションは含まれません（2操作変更が1操作変更の上位互換かを評価するため）。
    pub fn new(initial_state: State) -> Self {
        // 正しい解法を計算
        let correct_solution =
            CornerInspection::solve_corner_permutation_with_orientation(&initial_state);

        // 2操作変更の近傍バリエーションを列挙
        let searcher = NearbyOperationSearch::new(correct_solution.clone());

        let nearby_variants_by_one_distance = searcher.explore_variants(&initial_state);
        let nearby_variants_by_two_distance = searcher.explore_variants_two_changes(&initial_state);

        let mut nearby_variants = nearby_variants_by_one_distance;
        nearby_variants.extend(nearby_variants_by_two_distance);

        Self {
            initial_state,
            correct_solution,
            nearby_variants,
        }
    }

    /// 誤った最終状態から、どの操作列を適用したかを検出（2操作変更版）
    ///
    /// # Arguments
    /// * `wrongly_solved_state` - 誤って到達した状態
    ///
    /// # Returns
    /// マッチした操作列（複数の可能性がある）
    ///
    /// # 注意
    /// 正解の操作列から2つの操作が変更されたバリエーションのみを検出します。
    pub fn detect_wrong_operation(&self, wrongly_solved_state: &State) -> Vec<&ModifiedSequence> {
        self.nearby_variants
            .iter()
            .filter(|(_, final_state)| final_state == wrongly_solved_state)
            .map(|(modified_sequence, _)| modified_sequence)
            .collect()
    }

    /// ユーザーフレンドリーな形式で結果を表示
    ///
    /// # Arguments
    /// * `wrongly_solved_state` - 誤って到達した状態
    ///
    /// # Returns
    /// 表示用の文字列
    pub fn format_detection_result(&self, wrongly_solved_state: &State) -> String {
        let mut result = String::new();

        result.push_str(&format!("Initial State:\n"));
        result.push_str(&format!("  cp: {:?}\n", self.initial_state.cp));
        result.push_str(&format!("  co: {:?}\n\n", self.initial_state.co));

        result.push_str("Correct solution:\n");
        for (i, op) in self.correct_solution.iter().enumerate() {
            result.push_str(&format!("  Step {}: {}\n", i + 1, op));
        }
        result.push('\n');

        result.push_str(&format!("Wrong solved State:\n"));
        result.push_str(&format!("  cp: {:?}\n", wrongly_solved_state.cp));
        result.push_str(&format!("  co: {:?}\n\n", wrongly_solved_state.co));

        let wrong_operations = self.detect_wrong_operation(wrongly_solved_state);

        if wrong_operations.is_empty() {
            result.push_str("No matching wrong operation found.\n");
            result.push_str("The state might not be reachable by changing two operations.\n");
        } else {
            result.push_str(&format!(
                "Found {} possible wrong operation(s):\n\n",
                wrong_operations.len()
            ));

            for (idx, wrong_sequence) in wrong_operations.iter().enumerate() {
                result.push_str(&format!("Possibility {}:\n", idx + 1));
                result.push_str("Did you apply:\n");

                // fmt::Display for ModifiedCornerSequence を利用
                result.push_str(&format!("{}\n", wrong_sequence));
                result.push('\n');
            }
        }

        result
    }

    /// 正しい解法への参照を取得
    pub fn correct_solution(&self) -> &[CornerOperation] {
        &self.correct_solution
    }

    /// 初期状態への参照を取得
    pub fn initial_state(&self) -> &State {
        &self.initial_state
    }

    /// 近傍バリアントの数を取得
    pub fn variant_count(&self) -> usize {
        self.nearby_variants.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrong_operation_detection_two_changes() {
        // テスト用の初期状態（3つのSwap操作を必要とする状態）
        let initial_state = State::new(
            [1, 2, 0, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state.clone());

        println!("\n=== Wrong Operation Detection Test (Two Changes) ===");
        println!("Correct solution:");
        for (i, op) in detector.correct_solution().iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }
        println!("\nTotal variants generated: {}", detector.variant_count());

        // 2つの操作を変更した状態を手動で作成してテスト
        // 正解: [0→1→2→0] のサイクルを解消
        // ここでは簡単なテストとして、検出器が機能していることを確認
        assert!(
            detector.variant_count() > 0,
            "Should generate some variants"
        );
    }

    #[test]
    fn test_no_match_case() {
        let initial_state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state);

        // 全く関係ない状態を作成
        let unrelated_state = State::new(
            [7, 6, 5, 4, 3, 2, 1, 0],
            [2, 2, 2, 2, 1, 1, 1, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detected = detector.detect_wrong_operation(&unrelated_state);
        assert!(
            detected.is_empty(),
            "Should not detect any matching operation"
        );

        println!("\n=== No Match Case (Two Changes) ===");
        println!("{}", detector.format_detection_result(&unrelated_state));
    }

    #[test]
    fn test_variant_count_calculation() {
        // バリアント数の計算ロジックを検証
        // Case 1: 2 Swaps, 0 Twists
        let state1 = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        let detector1 = WrongOperationDetector::new(state1);
        // C(2, 2) × 24 × 24 = 1 × 576 = 576
        assert_eq!(detector1.variant_count(), 576, "Case 1: 2 Swaps");

        // Case 2: 0 Swaps, 2 Twists
        let state2 = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [1, 2, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        let detector2 = WrongOperationDetector::new(state2);
        // C(2, 2) × 16 × 16 = 1 × 256 = 256
        assert_eq!(detector2.variant_count(), 256, "Case 2: 2 Twists");

        println!("\n=== Variant Count Calculation Test ===");
        println!("Case 1 (2 Swaps): {} variants", detector1.variant_count());
        println!("Case 2 (2 Twists): {} variants", detector2.variant_count());
    }

    #[test]
    fn test_complex_case_detection() {
        let initial_state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state.clone());

        println!("\n=== Complex Case Detection Test ===");
        println!("\nScramble: U' F U2 D2 F' U B U F U D2 F' B' D2 F' U2 D2 B");
        println!("\nCorrect solution:");
        for (i, op) in detector.correct_solution().iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }
        println!("\nTotal variants generated: {}", detector.variant_count());

        // 2つの操作を誤った場合の状態例
        let wrong_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 1, 2, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n{}", detector.format_detection_result(&wrong_state));
    }

    #[test]
    fn test_two_operations_changed_detection() {
        // 簡単な初期状態: 複数のSwapとTwistが必要
        let initial_state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state.clone());
        let correct_solution = detector.correct_solution();

        println!("\n=== Two Operations Changed Detection Test ===");
        println!("Correct solution has {} steps", correct_solution.len());

        // SwapとTwistの数を数える
        let swap_count = correct_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        let twist_count = correct_solution
            .iter()
            .filter(|op| matches!(op, CornerOperation::Twist(_)))
            .count();

        println!("Swaps: {}, Twists: {}", swap_count, twist_count);

        // 期待されるバリアント数を計算
        let expected_swap_swap = if swap_count >= 2 {
            (swap_count * (swap_count - 1) / 2) * 24 * 24
        } else {
            0
        };
        let expected_swap_twist = swap_count * twist_count * 24 * 16;
        let expected_twist_twist = if twist_count >= 2 {
            (twist_count * (twist_count - 1) / 2) * 16 * 16
        } else {
            0
        };
        let expected_total = expected_swap_swap + expected_swap_twist + expected_twist_twist;

        println!("\nExpected variants:");
        println!("  Swap × Swap:   {}", expected_swap_swap);
        println!("  Swap × Twist:  {}", expected_swap_twist);
        println!("  Twist × Twist: {}", expected_twist_twist);
        println!("  Total:         {}", expected_total);
        println!("\nActual variants: {}", detector.variant_count());

        assert_eq!(
            detector.variant_count(),
            expected_total,
            "Variant count mismatch"
        );
    }

    #[test]
    fn test_insufficient_operations() {
        // 操作が1つしかない場合（2つ変更できないケース）
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // この状態の正解は1つのSwap操作のみ
        let detector = WrongOperationDetector::new(state);

        // 操作が1つしかない場合、2つの操作を変更できないので0バリアント
        assert_eq!(
            detector.variant_count(),
            0,
            "Should generate 0 variants when only 1 operation exists"
        );

        println!("\n=== Insufficient Operations Test ===");
        println!("Operations needed: {}", detector.correct_solution().len());
        println!("Variants generated: {}", detector.variant_count());
    }
}
