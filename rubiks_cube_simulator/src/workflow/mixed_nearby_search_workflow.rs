use crate::cube::State;
use crate::explorer::{
    MixedOperation, 
    NearbyMixedOperationSearch, 
    ModifiedMixedSequence, 
    AlternativeGenerator,
    CornerSwapAlternativeGenerator, 
    EdgeSwapAlternativeGenerator
};
use super::bld_workflow::{BldWorkflow, BldSolution};

/// Mixed Nearby Search を使用したワークフロー
pub struct MixedNearbySearchWorkflow {
    bld_workflow: BldWorkflow,
}

impl MixedNearbySearchWorkflow {
    /// 新しい MixedNearbySearchWorkflow を作成
    pub fn new(bld_workflow: BldWorkflow) -> Self {
        Self { bld_workflow }
    }

    /// 正しい操作列を取得
    pub fn get_correct_solution(&self, state: &State) -> Result<BldSolution, String> {
        self.bld_workflow.solve(state)
    }

    /// 正しい操作列を MixedOperation 列に変換
    pub fn solution_to_mixed_operations(&self, solution: &BldSolution) -> Vec<MixedOperation> {
        let mut mixed_ops = Vec::new();

        // Edge 操作を追加
        for edge_op in &solution.edge_operations {
            match edge_op {
                crate::inspection::EdgeOperation::Swap(swap_op) => {
                    mixed_ops.push(MixedOperation::EdgeSwap(swap_op.clone()));
                }
                crate::inspection::EdgeOperation::Flip(flip_op) => {
                    mixed_ops.push(MixedOperation::EdgeFlip(flip_op.clone()));
                }
            }
        }

        // Corner 操作を追加  
        for corner_op in &solution.corner_operations {
            match corner_op {
                crate::inspection::CornerOperation::Swap(swap_op) => {
                    mixed_ops.push(MixedOperation::CornerSwap(swap_op.clone()));
                }
                crate::inspection::CornerOperation::Twist(twist_op) => {
                    mixed_ops.push(MixedOperation::CornerTwist(twist_op.clone()));
                }
            }
        }

        mixed_ops
    }

    /// 指定した状態にたどり着く近傍操作列を探索
    pub fn find_variants_reaching_target(&self, 
        initial_state: &State, 
        target_state: &State
    ) -> Result<Vec<(ModifiedMixedSequence<MixedOperation>, State)>, String> {
        // 1. 正しい操作列を取得
        let solution = self.get_correct_solution(initial_state)?;
        let mixed_operations = self.solution_to_mixed_operations(&solution);

        // 2. 近傍探索を実行
        let generators: Vec<Box<dyn AlternativeGenerator<MixedOperation>>> = vec![
            Box::new(CornerSwapAlternativeGenerator),
            Box::new(EdgeSwapAlternativeGenerator),
        ];
        let search = NearbyMixedOperationSearch::with_alternative_generators(mixed_operations, generators);
        let variants = search.explore_variants_two_changes(initial_state);

        // 3. ターゲット状態に一致するものを探す
        let matching_variants: Vec<_> = variants
            .into_iter()
            .filter(|(_, final_state)| final_state == target_state)
            .collect();

        Ok(matching_variants)
    }

    /// 全てのバリエーションを探索（最大2つの変更）
    pub fn explore_all_variants(&self, initial_state: &State) -> Result<Vec<(ModifiedMixedSequence<MixedOperation>, State)>, String> {
        // 1. 正しい操作列を取得
        let solution = self.get_correct_solution(initial_state)?;
        let mixed_operations = self.solution_to_mixed_operations(&solution);

        // 2. 近傍探索を実行
        let generators: Vec<Box<dyn AlternativeGenerator<MixedOperation>>> = vec![
            Box::new(CornerSwapAlternativeGenerator),
            Box::new(EdgeSwapAlternativeGenerator),
        ];
        let search = NearbyMixedOperationSearch::with_alternative_generators(mixed_operations, generators);
        let variants = search.explore_variants_two_changes(initial_state);

        Ok(variants)
    }

    /// BldWorkflow への参照を取得
    pub fn bld_workflow(&self) -> &BldWorkflow {
        &self.bld_workflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_workflow() -> MixedNearbySearchWorkflow {
        let ufr_expanded = r#"{
            "RDB": { "RDF": "R U R' U R U2 R'" },
            "RDF": { "UFL": "U R U' R' U R U2 R'" },
            "UBR": { "UBL": "L' U' L U' L' U2 L" },
            "BUR": { "BUL": "U L' U L U' L' U2 L" }
        }"#;
        
        let ufr_parity = r#"{
            "RDB": "R U R' U R U2 R'",
            "UBR": "R U R' U R U2 R'",
            "RUB": "U R U' R' U R U2 R'",
            "BUR": "R U2 R' U' R U' R'",
            "UBL": "L' U' L U' L' U2 L",
            "BUL": "U' L' U L U' L' U2 L",
            "LUB": "L' U2 L U L' U L"
        }"#;
        
        let ufr_twist = r#"{
            "FUL": "R' D R D' R' D R U' R' D' R D R' D' R U"
        }"#;
        
        let uf_expanded = r#"{
            "FR": { "DL": "R U R' F R F'" },
            "RU": { "BL": "U R U' R' U R U R'" }
        }"#;
        
        let uf_flip = r#"{
            "UB": "R U R' U R U2 R'",
            "UR": "R U R' U R U2 R' U"
        }"#;
        
        let bld_workflow = BldWorkflow::new(&ufr_expanded, &ufr_parity, &ufr_twist, &uf_expanded, &uf_flip)
            .expect("Failed to create test workflow");
            
        MixedNearbySearchWorkflow::new(bld_workflow)
    }

    #[test]
    fn test_correct_solution_conversion() {
        let workflow = create_test_workflow();
        
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let solution = workflow.get_correct_solution(&state).unwrap();
        let mixed_ops = workflow.solution_to_mixed_operations(&solution);

        println!("Mixed operations count: {}", mixed_ops.len());
        for (i, op) in mixed_ops.iter().enumerate() {
            println!("  Step {}: {}", i + 1, op);
        }

        // Edge + Corner の操作数と一致することを確認
        assert_eq!(
            mixed_ops.len(),
            solution.edge_operations.len() + solution.corner_operations.len()
        );
    }

    #[test] 
    fn test_variants_exploration() {
        let workflow = create_test_workflow();
        
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let variants = workflow.explore_all_variants(&state).unwrap();
        
        println!("Found {} variants", variants.len());
        
        // 少なくともいくつかのバリエーションが見つかることを確認
        assert!(!variants.is_empty());
        
        // 各バリエーションの状態を確認
        for (i, (modified_seq, final_state)) in variants.iter().take(5).enumerate() {
            println!("Variant {}: {} -> {:?}", 
                i + 1, 
                modified_seq.get_description(),
                final_state
            );
        }
    }
}