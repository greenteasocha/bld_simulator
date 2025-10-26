use crate::cube::State;
use crate::explorer::edge_modifier::ModifiedEdgeSequence;
use crate::explorer::edge_nearby_search::NearbyEdgeOperationSearch;
use crate::inspection::{EdgeInspection, EdgeOperation};

/// エッジのユーザー操作ミスを検出して提案する機能
///
/// 完成状態を目指したが、誤って近傍の状態に到達してしまった場合に、
/// どの操作列を適用したのかを推測して提示する
pub struct WrongEdgeOperationDetector {
    initial_state: State,
    correct_solution: Vec<EdgeOperation>,
    nearby_variants: Vec<(ModifiedEdgeSequence, State)>,
}

impl WrongEdgeOperationDetector {
    /// 新しい WrongEdgeOperationDetector を作成
    ///
    /// # Arguments
    /// * `initial_state` - 初期状態
    ///
    /// # Returns
    /// 初期化された WrongEdgeOperationDetector
    pub fn new(initial_state: State) -> Self {
        // 正しい解法を計算
        let correct_solution =
            EdgeInspection::solve_edge_permutation_with_orientation(&initial_state, false);

        // 近傍の操作列を列挙
        let searcher = NearbyEdgeOperationSearch::new(correct_solution.clone());

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
    ) -> Vec<&ModifiedEdgeSequence> {
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

        result.push_str("Initial State:\n");
        result.push_str(&format!("  ep: {:?}\n", self.initial_state.ep));
        result.push_str(&format!("  eo: {:?}\n\n", self.initial_state.eo));

        result.push_str("Correct solution:\n");
        for (i, op) in self.correct_solution.iter().enumerate() {
            result.push_str(&format!("  Step {}: {}\n", i + 1, op));
        }
        result.push('\n');

        result.push_str("Wrong solved State:\n");
        result.push_str(&format!("  ep: {:?}\n", wrongly_solved_state.ep));
        result.push_str(&format!("  eo: {:?}\n\n", wrongly_solved_state.eo));

        let wrong_operations = self.detect_wrong_operation(wrongly_solved_state);

        if wrong_operations.is_empty() {
            result.push_str("No matching wrong operation found.\n");
            result.push_str("The state might not be reachable by a single operation change.\n");
        } else {
            result.push_str(&format!(
                "Found {} possible wrong operation(s):\n\n",
                wrong_operations.len()
            ));

            for (idx, wrong_sequence) in wrong_operations.iter().enumerate() {
                result.push_str(&format!("Possibility {}:\n", idx + 1));
                result.push_str("Did you apply:\n");

                // fmt::Display for ModifiedEdgeSequence を利用
                result.push_str(&format!("{}\n", wrong_sequence));

                result.push('\n');
            }
        }

        result
    }

    /// 正しい解法への参照を取得
    pub fn correct_solution(&self) -> &[EdgeOperation] {
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
    fn test_wrong_edge_operation_detection() {
        // Initial State:
        // ep: [1, 2, 0, 3, 4, 7, 5, 6, 8, 9, 10, 11]
        // eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        // Correct solution:
        // Step 1: Swap: UF ↔ UR
        // Step 2: Swap: UF ↔ UL
        // Step 3: Swap: UF ↔ BL
        // Step 4: Swap: UF ↔ BR
        // Step 5: Swap: UF ↔ FR
        // Step 6: Swap: UF ↔ BL

        // Wrong solved State:
        // ep: [0, 1, 2, 3, 4, 6, 7, 5, 8, 9, 10, 11]
        // eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        // Found 1 possible wrong operation(s):

        // Possibility 1:
        // Did you apply:
        // Step 1: **Swap: UF ↔ UL**
        // Step 2: **Swap: UF ↔ UR**
        // Step 3: Swap: UF ↔ BL
        // Step 4: Swap: UF ↔ BR
        // Step 5: Swap: UF ↔ FR
        // Step 6: Swap: UF ↔ BL

        // テスト用の初期状態
        let initial_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 2, 0, 3, 4, 7, 5, 6, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let wrong_solved_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 6, 7, 5, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongEdgeOperationDetector::new(initial_state.clone());
        detector.detect_wrong_operation(&wrong_solved_state);

        println!("\n=== Wrong Edge Operation Detection Test ===");
        println!(
            "\n{}",
            detector.format_detection_result(&wrong_solved_state)
        );
    }

    #[test]
    fn test_wrong_one_edge_operation_detection() {
        // FIXME: 距離1の探索で良いのに距離2からも結果が重複する
        // Initial State:
        // ep: [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11]
        // eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        // Correct solution:
        // Step 1: Swap: UF ↔ UR
        // Step 2: Swap: UF ↔ UL

        // Wrong solved State:
        // ep: [0, 1, 2, 3, 7, 5, 4, 6, 8, 9, 10, 11]
        // eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        // Found 2 possible wrong operation(s):

        // Possibility 1:
        // Did you apply:
        // Step 1: Swap: UF ↔ UR
        // Step 2: **Swap: UF ↔ UB**

        // Possibility 2:
        // Did you apply:
        // Step 1: **Swap: UF ↔ UR**
        // Step 2: **Swap: UF ↔ UB**

        // テスト用の初期状態
        let initial_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let wrong_solved_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 7, 5, 4, 6, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongEdgeOperationDetector::new(initial_state.clone());
        detector.detect_wrong_operation(&wrong_solved_state);

        println!("\n=== Wrong Edge Operation Detection Test ===");
        println!(
            "\n{}",
            detector.format_detection_result(&wrong_solved_state)
        );
    }

    #[test]
    fn test_no_match_case() {
        let initial_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let detector = WrongEdgeOperationDetector::new(initial_state);

        // 全く関係ない状態を作成
        let unrelated_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
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
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 6, 2, 5, 4, 3, 7, 9, 10, 11, 8],
            [1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );

        let detector = WrongEdgeOperationDetector::new(initial_state.clone());

        println!("\n=== Complex Edge Case Detection Test ===");

        let wrong_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n{}", detector.format_detection_result(&wrong_state));
    }
}
