use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Corner Swap用のAlternative Generator
pub struct CornerSwapAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for CornerSwapAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::CornerSwap(_) => {
                self.generate_corner_swap_alternatives()
                    .into_iter()
                    .map(|swap_op| MixedOperation::CornerSwap(swap_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl CornerSwapAlternativeGenerator {
    fn generate_corner_swap_alternatives(&self) -> Vec<crate::inspection::CornerSwapOperation> {
        use crate::inspection::CornerSwapOperation;
        let mut alternatives = Vec::new();

        const CORNER_BUFFER: usize = 2; // UFR

        // 各コーナー位置について
        for target in 0..8 {
            if target == CORNER_BUFFER {
                continue;
            }

            // 各方向について
            for orientation in 0..3 {
                alternatives.push(CornerSwapOperation::new(CORNER_BUFFER, target, orientation));
            }
        }

        alternatives
    }
}