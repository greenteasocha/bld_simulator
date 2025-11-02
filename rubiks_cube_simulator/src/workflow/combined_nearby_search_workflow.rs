use crate::cube::State;
use crate::explorer::ModifiedMoveSequenceCollection;
use crate::workflow::{BldWorkflow, BldSolution, MixedNearbySearchWorkflow, NearbySequenceSearchWorkflow};
use crate::explorer::ModifiedMixedSequence;

/// Combined Nearby Search の結果
#[derive(Debug)]
pub struct CombinedSearchResult {
    /// 元の解法が見つかったか
    pub solution_found: bool,
    /// 元の解法
    pub original_solution: Option<BldSolution>,
    /// Operation レベルの代替案
    pub operation_variants: Vec<(ModifiedMixedSequence, State)>,
    /// Move レベルの代替案
    pub move_variants: Vec<ModifiedMoveSequenceCollection>,
    /// 初期状態
    pub initial_state: State,
    /// 目標状態
    pub target_state: State,
    /// スクランブル文字列（オプション）
    pub scramble: Option<String>,
}

impl CombinedSearchResult {
    /// 合計の代替案数を取得
    pub fn total_count(&self) -> usize {
        self.operation_variants.len() + self.move_variants.len()
    }

    /// Operation レベルの代替案数を取得
    pub fn operation_count(&self) -> usize {
        self.operation_variants.len()
    }

    /// Move レベルの代替案数を取得
    pub fn move_count(&self) -> usize {
        self.move_variants.len()
    }

    /// 結果のサマリーを文字列で取得
    pub fn summary(&self) -> String {
        if !self.solution_found {
            return "Failed to find original solution".to_string();
        }

        let mut result = String::new();
        result.push_str(&format!("Total alternatives found: {}\n", self.total_count()));
        result.push_str(&format!("  - Operation-level alternatives: {}\n", self.operation_count()));
        result.push_str(&format!("  - Move-level alternatives: {}\n", self.move_count()));
        
        if self.total_count() > 0 {
            result.push_str("\n✓ At least one alternative path exists to reach the target state!");
        } else {
            result.push_str("\n✗ No alternative paths found to reach the target state.");
        }
        
        result
    }

    /// 結果を詳細に表示
    pub fn display_detailed(&self, max_variants_per_type: usize) -> String {
        let mut result = String::new();

        if !self.solution_found {
            result.push_str("Failed to find original solution\n");
            return result;
        }

        result.push_str("=== Combined Nearby Search Results ===\n\n");

        // Scramble (if provided)
        if let Some(ref scramble) = self.scramble {
            result.push_str("Scramble:\n");
            result.push_str(&format!("  {}\n\n", scramble));
        }

        // Initial and Target states
        result.push_str("Initial state:\n");
        result.push_str(&format!("  cp: {:?}\n", self.initial_state.cp));
        result.push_str(&format!("  co: {:?}\n", self.initial_state.co));
        result.push_str(&format!("  ep: {:?}\n", self.initial_state.ep));
        result.push_str(&format!("  eo: {:?}\n", self.initial_state.eo));
        result.push_str("\n");

        result.push_str("Target state:\n");
        result.push_str(&format!("  cp: {:?}\n", self.target_state.cp));
        result.push_str(&format!("  co: {:?}\n", self.target_state.co));
        result.push_str(&format!("  ep: {:?}\n", self.target_state.ep));
        result.push_str(&format!("  eo: {:?}\n", self.target_state.eo));
        result.push_str("\n");

        // Original solution
        if let Some(ref solution) = self.original_solution {
            result.push_str("=== Original Solution ===\n\n");
            
            // All operations (Edge → Corner)
            result.push_str("All Operations:\n");
            if solution.all_operations.is_empty() {
                result.push_str("  (none)\n");
            } else {
                for (i, op) in solution.all_operations.operations().iter().enumerate() {
                    result.push_str(&format!("  Step {}: {}\n", i + 1, op));
                }
            }
            result.push_str("\n");

            // Move Sequences
            result.push_str("Move Sequences:\n");
            if solution.move_sequences.is_empty() {
                result.push_str("  (none)\n");
            } else {
                for (i, seq) in solution.move_sequences.sequences().iter().enumerate() {
                    if !seq.description.is_empty() {
                        result.push_str(&format!("  Sequence {}: {}\n", i + 1, seq.description));
                    }
                    result.push_str(&format!("    {}\n", seq));
                }
            }
            result.push_str("\n");
        }

        // Operation variants
        if !self.operation_variants.is_empty() {
            result.push_str(&format!(
                "=== Operation Variants ({} found) ===\n",
                self.operation_variants.len()
            ));
            
            for (i, (modified_seq, final_state)) in self.operation_variants
                .iter()
                .take(max_variants_per_type)
                .enumerate()
            {
                result.push_str(&format!("\nOperation Variant {}:\n", i + 1));
                result.push_str(&format!("{}\n", modified_seq));
                result.push_str("  Final state verification:\n");
                result.push_str(&format!("    cp: {:?}\n", final_state.cp));
                result.push_str(&format!("    co: {:?}\n", final_state.co));
                result.push_str(&format!("    ep: {:?}\n", final_state.ep));
                result.push_str(&format!("    eo: {:?}\n", final_state.eo));
            }
            
            if self.operation_variants.len() > max_variants_per_type {
                result.push_str(&format!(
                    "\n... and {} more operation variants\n",
                    self.operation_variants.len() - max_variants_per_type
                ));
            }
            result.push_str("\n");
        } else {
            result.push_str("=== Operation Variants ===\n");
            result.push_str("No operation variants found.\n\n");
        }

        // Move variants
        if !self.move_variants.is_empty() {
            result.push_str(&format!(
                "=== Move Variants ({} found) ===\n",
                self.move_variants.len()
            ));
            
            for (i, modified_collection) in self.move_variants
                .iter()
                .take(max_variants_per_type)
                .enumerate()
            {
                result.push_str(&format!("\nMove Variant {}:\n", i + 1));
                result.push_str(&format!("{}\n", modified_collection));
                
                let final_state = modified_collection.apply_to_state(&self.initial_state);
                result.push_str("  Final state verification:\n");
                result.push_str(&format!("    cp: {:?}\n", final_state.cp));
                result.push_str(&format!("    co: {:?}\n", final_state.co));
                result.push_str(&format!("    ep: {:?}\n", final_state.ep));
                result.push_str(&format!("    eo: {:?}\n", final_state.eo));
            }
            
            if self.move_variants.len() > max_variants_per_type {
                result.push_str(&format!(
                    "\n... and {} more move variants\n",
                    self.move_variants.len() - max_variants_per_type
                ));
            }
            result.push_str("\n");
        } else {
            result.push_str("=== Move Variants ===\n");
            result.push_str("No move variants found.\n\n");
        }

        // Summary
        result.push_str(&format!("=== Summary ===\n{}\n", self.summary()));

        result
    }
}

/// Operation と Move の両方の近傍探索を統合したワークフロー
pub struct CombinedNearbySearchWorkflow {
    bld_workflow: BldWorkflow,
}

impl CombinedNearbySearchWorkflow {
    /// 新しい CombinedNearbySearchWorkflow を作成
    pub fn new(bld_workflow: BldWorkflow) -> Self {
        Self { bld_workflow }
    }

    /// JSON ファイルから直接初期化
    pub fn from_json(
        ufr_expanded_json: &str,
        ufr_parity_json: &str,
        ufr_twist_json: &str,
        uf_expanded_json: &str,
        uf_flip_json: &str,
    ) -> Result<Self, String> {
        let bld_workflow = BldWorkflow::new(
            ufr_expanded_json,
            ufr_parity_json,
            ufr_twist_json,
            uf_expanded_json,
            uf_flip_json,
        )?;
        Ok(Self::new(bld_workflow))
    }

    /// スクランブル文字列と目標状態から代替案を探索
    ///
    /// # Arguments
    /// * `scramble` - スクランブル文字列（例: "R U R' D"）
    /// * `target_state` - 目標となる状態
    ///
    /// # Returns
    /// Operation と Move の両方の代替案を含む結果
    pub fn search_from_scramble(
        &self,
        scramble: &str,
        target_state: &State,
    ) -> Result<CombinedSearchResult, String> {
        // スクランブル文字列から初期状態を生成
        let cube = crate::cube::operations::RubiksCube::new();
        let initial_state = cube.scramble_to_state(scramble);

        self.search_internal(&initial_state, target_state, Some(scramble.to_string()))
    }

    /// 初期状態と目標状態から代替案を探索
    ///
    /// # Arguments
    /// * `initial_state` - 初期状態
    /// * `target_state` - 目標となる状態
    ///
    /// # Returns
    /// Operation と Move の両方の代替案を含む結果
    pub fn search(
        &self,
        initial_state: &State,
        target_state: &State,
    ) -> Result<CombinedSearchResult, String> {
        self.search_internal(initial_state, target_state, None)
    }

    /// 内部的な探索処理
    fn search_internal(
        &self,
        initial_state: &State,
        target_state: &State,
        scramble: Option<String>,
    ) -> Result<CombinedSearchResult, String> {
        // 1. 解法を取得
        let solution = self.bld_workflow.solve(initial_state)?;

        // 2. Operation 近傍探索
        let mixed_workflow = MixedNearbySearchWorkflow::new_from_bld_workflow(&self.bld_workflow);
        let operation_variants = mixed_workflow
            .find_variants_reaching_target(initial_state, target_state)
            .unwrap_or_else(|_| Vec::new());

        // 3. Move 近傍探索
        let move_collection = solution.move_sequence_collection();
        let sequences: Vec<_> = move_collection
            .sequences()
            .iter()
            .map(|seq| seq.moves.clone())
            .collect();

        let move_workflow = NearbySequenceSearchWorkflow::new(sequences);
        let move_variants = move_workflow.find_alternatives(initial_state, target_state);

        Ok(CombinedSearchResult {
            solution_found: true,
            original_solution: Some(solution),
            operation_variants,
            move_variants,
            initial_state: initial_state.clone(),
            target_state: target_state.clone(),
            scramble,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_workflow() -> CombinedNearbySearchWorkflow {
        let ufr_expanded = r#"{
            "RDB": { "RDF": "R U R' U R U2 R'" },
            "RDF": { "UFL": "U R U' R' U R U2 R'" }
        }"#;

        let ufr_parity = r#"{
            "RDB": "R U R' U R U2 R'",
            "UBR": "R U R' U R U2 R'"
        }"#;

        let ufr_twist = r#"{
            "FUL": "R' D R D' R' D R U' R' D' R D R' D' R U"
        }"#;

        let uf_expanded = r#"{
            "FR": { "DL": "R U R' F R F'" }
        }"#;

        let uf_flip = r#"{
            "UB": "R U R' U R U2 R'"
        }"#;

        CombinedNearbySearchWorkflow::from_json(
            &ufr_expanded,
            &ufr_parity,
            &ufr_twist,
            &uf_expanded,
            &uf_flip,
        )
        .expect("Failed to create test workflow")
    }

    #[test]
    fn test_search_from_scramble() {
        let workflow = create_test_workflow();

        let scramble = "R U R'";
        let target_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let result = workflow
            .search_from_scramble(scramble, &target_state)
            .expect("Failed to search");

        assert!(result.solution_found);
        assert!(result.original_solution.is_some());
        assert_eq!(result.scramble, Some("R U R'".to_string()));
        println!("Total alternatives: {}", result.total_count());
    }

    #[test]
    fn test_search_with_state() {
        let workflow = create_test_workflow();

        let initial_state = State::new(
            [0, 1, 7, 3, 4, 5, 2, 6],
            [0, 0, 1, 0, 0, 0, 2, 0],
            [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let target_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 11, 6, 8, 9, 10, 7],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let result = workflow
            .search(&initial_state, &target_state)
            .expect("Failed to search");

        assert!(result.solution_found);
        assert!(result.original_solution.is_some());
        assert!(result.scramble.is_none());
        println!("{}", result.summary());
    }

    #[test]
    fn test_display_detailed() {
        let workflow = create_test_workflow();

        let initial_state = State::new(
            [0, 1, 7, 3, 4, 5, 2, 6],
            [0, 0, 1, 0, 0, 0, 2, 0],
            [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let target_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 11, 6, 8, 9, 10, 7],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let result = workflow
            .search(&initial_state, &target_state)
            .expect("Failed to search");

        let detailed = result.display_detailed(3);
        println!("\n{}", detailed);

        assert!(detailed.contains("=== Combined Nearby Search Results ==="));
        assert!(detailed.contains("=== Original Solution ==="));
        assert!(detailed.contains("=== Summary ==="));
    }

    #[test]
    fn test_result_methods() {
        let initial_state = State::solved();
        let target_state = State::solved();

        let result = CombinedSearchResult {
            solution_found: true,
            original_solution: None,
            operation_variants: vec![],
            move_variants: vec![],
            initial_state,
            target_state,
            scramble: None,
        };

        assert_eq!(result.total_count(), 0);
        assert_eq!(result.operation_count(), 0);
        assert_eq!(result.move_count(), 0);

        let summary = result.summary();
        assert!(summary.contains("Total alternatives found: 0"));
    }
}
