use crate::cube::State;
use crate::explorer::NearbyOperationSearch;
use crate::inspection::{CornerInspection, CornerOperation};

/// ユーザーの操作ミスを検出して提案する機能
///
/// 完成状態を目指したが、誤って近傍の状態に到達してしまった場合に、
/// どの操作列を適用したのかを推測して提示する
pub struct WrongOperationDetector {
    initial_state: State,
    correct_solution: Vec<CornerOperation>,
    nearby_variants: Vec<(Vec<CornerOperation>, State)>,
}

impl WrongOperationDetector {
    /// 新しい WrongOperationDetector を作成
    ///
    /// # Arguments
    /// * `initial_state` - 初期状態
    ///
    /// # Returns
    /// 初期化された WrongOperationDetector
    pub fn new(initial_state: State) -> Self {
        // 正しい解法を計算
        let correct_solution =
            CornerInspection::solve_corner_permutation_with_orientation(&initial_state);

        // 近傍の操作列を列挙
        let searcher = NearbyOperationSearch::new(correct_solution.clone());
        let nearby_variants = searcher.explore_variants(&initial_state);

        Self {
            initial_state,
            correct_solution,
            nearby_variants,
        }
    }

    /// 誤った最終状態から、どの操作列を適用したかを検出
    ///
    /// # Arguments
    /// * `wrongly_solved_state` - 誤って到達した状態
    ///
    /// # Returns
    /// マッチした操作列（複数の可能性がある）
    pub fn detect_wrong_operation(
        &self,
        wrongly_solved_state: &State,
    ) -> Vec<&Vec<CornerOperation>> {
        self.nearby_variants
            .iter()
            .filter(|(_, final_state)| final_state == wrongly_solved_state)
            .map(|(operations, _)| operations)
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
            result.push_str("The state might not be reachable by a single operation change.\n");
        } else {
            result.push_str(&format!(
                "Found {} possible wrong operation(s):\n\n",
                wrong_operations.len()
            ));

            for (idx, wrong_ops) in wrong_operations.iter().enumerate() {
                result.push_str(&format!("Possibility {}:\n", idx + 1));
                result.push_str("Did you apply:\n");

                for (i, op) in wrong_ops.iter().enumerate() {
                    result.push_str(&format!("  Step {}: {}\n", i + 1, op));
                }
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
    fn test_wrong_operation_detection() {
        // テスト用の初期状態
        let initial_state = State::new(
            [1, 2, 0, 3, 4, 5, 6, 7],
            [1, 2, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let wrong_solved_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 1, 2, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state.clone());

        println!("\n=== Wrong Operation Detection Test ===");

        println!(
            "\n{}",
            detector.format_detection_result(&wrong_solved_state)
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

        println!("\n=== No Match Case ===");
        println!("{}", detector.format_detection_result(&unrelated_state));
    }

    #[test]
    fn test_complex_case_detection() {
        let initial_state = State::new(
            [3, 1, 2, 0, 5, 7, 4, 6],
            [2, 1, 0, 1, 0, 2, 1, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state.clone());

        println!("\n=== Complex Case Detection Test ===");
        println!(
            "Initial state: cp={:?}, co={:?}",
            initial_state.cp, initial_state.co
        );
        println!("Correct solution:");
        for (i, op) in detector.correct_solution().iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }

        let wrong_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 2, 0, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        detector.format_detection_result(&wrong_state);
        // いくつかのバリアントをテスト
        // let test_count = std::cmp::min(5, detector.variant_count());
        // println!("\nTesting {} variants:", test_count);

        // for i in 0..test_count {
        //     let (_, wrong_state) = &detector.nearby_variants[i];
        //     let detected = detector.detect_wrong_operation(wrong_state);
        //     println!("  Variant {}: {} match(es) found", i + 1, detected.len());
    }

    #[test]
    fn test_multiple_matches() {
        // 同じ最終状態に到達する複数の操作列が存在する可能性をテスト
        let initial_state = State::new(
            [2, 1, 0, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongOperationDetector::new(initial_state);

        // 各バリアントの最終状態でグループ化
        use std::collections::HashMap;
        let mut state_groups: HashMap<State, usize> = HashMap::new();

        for (_, final_state) in &detector.nearby_variants {
            *state_groups.entry(final_state.clone()).or_insert(0) += 1;
        }

        let duplicates: Vec<_> = state_groups
            .iter()
            .filter(|(_, &count)| count > 1)
            .collect();

        println!("\n=== Multiple Matches Test ===");
        println!("Total variants: {}", detector.variant_count());
        println!("Unique final states: {}", state_groups.len());
        println!("States with multiple paths: {}", duplicates.len());

        if let Some((state, count)) = duplicates.first() {
            println!("\nExample: {} operations lead to the same state", count);
            let detected = detector.detect_wrong_operation(state);
            assert_eq!(detected.len(), **count);
        }
    }
}
