use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Corner Twist用のAlternative Generator
pub struct CornerTwistAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for CornerTwistAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::CornerTwist(original_op) => {
                self.generate_corner_twist_alternatives(original_op)
                    .into_iter()
                    .map(|twist_op| MixedOperation::CornerTwist(twist_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl CornerTwistAlternativeGenerator {
    fn generate_corner_twist_alternatives(&self, original: &crate::inspection::CornerTwistOperation) -> Vec<crate::inspection::CornerTwistOperation> {
        use crate::inspection::CornerTwistOperation;
        let mut alternatives = Vec::new();

        // 各コーナー位置について
        for target in 0..8 {
            // 各回転方向について (1=CW, 2=CCW)
            for twist in 1..=2 {
                let candidate = CornerTwistOperation::new(target, twist);
                
                // 元の操作と異なる場合のみ追加
                if candidate != *original {
                    alternatives.push(candidate);
                }
            }
        }

        alternatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspection::CornerTwistOperation;

    #[test]
    fn test_does_not_include_original() {
        let generator = CornerTwistAlternativeGenerator;
        let original = CornerTwistOperation::new(0, 1);
        let original_mixed = MixedOperation::CornerTwist(original.clone());
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // 元の操作が含まれていないことを確認
        assert!(!alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::CornerTwist(op) => op == &original,
                _ => false,
            }
        }));
        
        // 代替案が生成されていることを確認（8 targets × 2 twists - 1 original = 15）
        assert_eq!(alternatives.len(), 15);
    }

    #[test]
    fn test_different_operations_included() {
        let generator = CornerTwistAlternativeGenerator;
        let original = CornerTwistOperation::new(0, 1);
        let original_mixed = MixedOperation::CornerTwist(original);
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // target=0, twist=2 の操作は含まれているはず
        let expected = CornerTwistOperation::new(0, 2);
        assert!(alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::CornerTwist(op) => op == &expected,
                _ => false,
            }
        }));
        
        // target=1, twist=1 の操作も含まれているはず
        let expected2 = CornerTwistOperation::new(1, 1);
        assert!(alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::CornerTwist(op) => op == &expected2,
                _ => false,
            }
        }));
    }
}
