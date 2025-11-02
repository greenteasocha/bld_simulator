use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Edge Swap用のAlternative Generator
pub struct EdgeSwapAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for EdgeSwapAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::EdgeSwap(_) => {
                self.generate_edge_swap_alternatives()
                    .into_iter()
                    .map(|swap_op| MixedOperation::EdgeSwap(swap_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl EdgeSwapAlternativeGenerator {
    fn generate_edge_swap_alternatives(&self) -> Vec<crate::inspection::EdgeSwapOperation> {
        use crate::inspection::EdgeSwapOperation;
        let mut alternatives = Vec::new();

        const EDGE_BUFFER: usize = 6; // UF

        // 各エッジ位置について
        for target in 0..12 {
            if target == EDGE_BUFFER {
                continue;
            }

            // 各方向について
            for orientation in 0..2 {
                alternatives.push(EdgeSwapOperation::new(EDGE_BUFFER, target, orientation));
            }
        }

        alternatives
    }
}