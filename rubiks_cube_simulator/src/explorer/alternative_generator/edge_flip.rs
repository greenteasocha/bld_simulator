use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Edge Flip用のAlternative Generator
pub struct EdgeFlipAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for EdgeFlipAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::EdgeFlip(_) => {
                self.generate_edge_flip_alternatives()
                    .into_iter()
                    .map(|flip_op| MixedOperation::EdgeFlip(flip_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl EdgeFlipAlternativeGenerator {
    fn generate_edge_flip_alternatives(&self) -> Vec<crate::inspection::EdgeFlipOperation> {
        use crate::inspection::EdgeFlipOperation;
        let mut alternatives = Vec::new();

        // 各エッジ位置について
        for target in 0..12 {
            alternatives.push(EdgeFlipOperation::new(target));
        }

        alternatives
    }
}