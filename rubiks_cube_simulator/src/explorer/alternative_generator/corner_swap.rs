use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Corner Swap用のAlternative Generator
pub struct CornerSwapAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for CornerSwapAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::CornerSwap(original_op) => {
                self.generate_corner_swap_alternatives(original_op)
                    .into_iter()
                    .map(|swap_op| MixedOperation::CornerSwap(swap_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl CornerSwapAlternativeGenerator {
    fn generate_corner_swap_alternatives(&self, original: &crate::inspection::CornerSwapOperation) -> Vec<crate::inspection::CornerSwapOperation> {
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
                let candidate = CornerSwapOperation::new(CORNER_BUFFER, target, orientation);
                
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
    use crate::inspection::CornerSwapOperation;

    #[test]
    fn test_does_not_include_original() {
        let generator = CornerSwapAlternativeGenerator;
        let original = CornerSwapOperation::new(2, 1, 0);
        let original_mixed = MixedOperation::CornerSwap(original.clone());
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // 元の操作が含まれていないことを確認
        assert!(!alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::CornerSwap(op) => op == &original,
                _ => false,
            }
        }));
        
        // 代替案が生成されていることを確認（7 targets × 3 orientations - 1 original = 20）
        assert_eq!(alternatives.len(), 20);
    }

    #[test]
    fn test_different_operations_included() {
        let generator = CornerSwapAlternativeGenerator;
        let original = CornerSwapOperation::new(2, 1, 0);
        let original_mixed = MixedOperation::CornerSwap(original);
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // target=1, orientation=1 の操作は含まれているはず
        let expected = CornerSwapOperation::new(2, 1, 1);
        assert!(alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::CornerSwap(op) => op == &expected,
                _ => false,
            }
        }));
    }
}
