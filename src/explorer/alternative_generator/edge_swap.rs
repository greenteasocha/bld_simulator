use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Edge Swap用のAlternative Generator
pub struct EdgeSwapAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for EdgeSwapAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::EdgeSwap(original_op) => {
                self.generate_edge_swap_alternatives(original_op)
                    .into_iter()
                    .map(|swap_op| MixedOperation::EdgeSwap(swap_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl EdgeSwapAlternativeGenerator {
    fn generate_edge_swap_alternatives(&self, original: &crate::inspection::EdgeSwapOperation) -> Vec<crate::inspection::EdgeSwapOperation> {
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
                let candidate = EdgeSwapOperation::new(EDGE_BUFFER, target, orientation);
                
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
    use crate::inspection::EdgeSwapOperation;

    #[test]
    fn test_does_not_include_original() {
        let generator = EdgeSwapAlternativeGenerator;
        let original = EdgeSwapOperation::new(6, 0, 0);
        let original_mixed = MixedOperation::EdgeSwap(original.clone());
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // 元の操作が含まれていないことを確認
        assert!(!alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::EdgeSwap(op) => op == &original,
                _ => false,
            }
        }));
        
        // 代替案が生成されていることを確認（11 targets × 2 orientations - 1 original = 21）
        assert_eq!(alternatives.len(), 21);
    }

    #[test]
    fn test_different_operations_included() {
        let generator = EdgeSwapAlternativeGenerator;
        let original = EdgeSwapOperation::new(6, 0, 0);
        let original_mixed = MixedOperation::EdgeSwap(original);
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // target=0, orientation=1 の操作は含まれているはず
        let expected = EdgeSwapOperation::new(6, 0, 1);
        assert!(alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::EdgeSwap(op) => op == &expected,
                _ => false,
            }
        }));
    }
}
