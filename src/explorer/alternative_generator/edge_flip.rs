use crate::explorer::mixed_nearby_search::{AlternativeGenerator, MixedOperation};

/// Edge Flip用のAlternative Generator
pub struct EdgeFlipAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for EdgeFlipAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::EdgeFlip(original_op) => {
                self.generate_edge_flip_alternatives(original_op)
                    .into_iter()
                    .map(|flip_op| MixedOperation::EdgeFlip(flip_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl EdgeFlipAlternativeGenerator {
    fn generate_edge_flip_alternatives(&self, original: &crate::inspection::EdgeFlipOperation) -> Vec<crate::inspection::EdgeFlipOperation> {
        use crate::inspection::EdgeFlipOperation;
        let mut alternatives = Vec::new();

        // 各エッジ位置について
        for target in 0..12 {
            let candidate = EdgeFlipOperation::new(target);
            
            // 元の操作と異なる場合のみ追加
            if candidate != *original {
                alternatives.push(candidate);
            }
        }

        alternatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspection::EdgeFlipOperation;

    #[test]
    fn test_does_not_include_original() {
        let generator = EdgeFlipAlternativeGenerator;
        let original = EdgeFlipOperation::new(0);
        let original_mixed = MixedOperation::EdgeFlip(original.clone());
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // 元の操作が含まれていないことを確認
        assert!(!alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::EdgeFlip(op) => op == &original,
                _ => false,
            }
        }));
        
        // 代替案が生成されていることを確認（12 targets - 1 original = 11）
        assert_eq!(alternatives.len(), 11);
    }

    #[test]
    fn test_different_operations_included() {
        let generator = EdgeFlipAlternativeGenerator;
        let original = EdgeFlipOperation::new(0);
        let original_mixed = MixedOperation::EdgeFlip(original);
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // target=1 の操作は含まれているはず
        let expected = EdgeFlipOperation::new(1);
        assert!(alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::EdgeFlip(op) => op == &expected,
                _ => false,
            }
        }));
        
        // target=11 の操作も含まれているはず
        let expected2 = EdgeFlipOperation::new(11);
        assert!(alternatives.iter().any(|alt| {
            match alt {
                MixedOperation::EdgeFlip(op) => op == &expected2,
                _ => false,
            }
        }));
    }

    #[test]
    fn test_all_other_targets_included() {
        let generator = EdgeFlipAlternativeGenerator;
        let original = EdgeFlipOperation::new(5);
        let original_mixed = MixedOperation::EdgeFlip(original);
        
        let alternatives = generator.generate_alternatives(&original_mixed);
        
        // target=5 以外の11個の操作が全て含まれているはず
        for target in 0..12 {
            if target == 5 {
                continue;
            }
            let expected = EdgeFlipOperation::new(target);
            assert!(alternatives.iter().any(|alt| {
                match alt {
                    MixedOperation::EdgeFlip(op) => op == &expected,
                    _ => false,
                }
            }), "target={} should be included", target);
        }
    }
}
