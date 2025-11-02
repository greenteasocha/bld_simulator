use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Corner Twist用のAlternative Generator
pub struct CornerTwistAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for CornerTwistAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::CornerTwist(_) => {
                self.generate_corner_twist_alternatives()
                    .into_iter()
                    .map(|twist_op| MixedOperation::CornerTwist(twist_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl CornerTwistAlternativeGenerator {
    fn generate_corner_twist_alternatives(&self) -> Vec<crate::inspection::CornerTwistOperation> {
        use crate::inspection::CornerTwistOperation;
        let mut alternatives = Vec::new();

        // 各コーナー位置について
        for target in 0..8 {
            // 各回転方向について (1=CW, 2=CCW)
            for twist in 1..=2 {
                alternatives.push(CornerTwistOperation::new(target, twist));
            }
        }

        alternatives
    }
}