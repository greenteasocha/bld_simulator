use crate::inspection::{EdgeOperation, EdgeSwapOperation, EdgeFlipOperation};
use std::fmt;

/// Edge Swap操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeSwapModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいSwap操作
    pub modifier: EdgeSwapOperation,
}

/// Edge Flip操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeFlipModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいFlip操作
    pub modifier: EdgeFlipOperation,
}

/// Edge操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeModifier {
    Swap(EdgeSwapModifier),
    Flip(EdgeFlipModifier),
}

impl EdgeModifier {
    /// 変更対象のステップ番号を取得
    pub fn step(&self) -> usize {
        match self {
            EdgeModifier::Swap(m) => m.step,
            EdgeModifier::Flip(m) => m.step,
        }
    }

    /// 変更後のEdgeOperationを取得
    pub fn operation(&self) -> EdgeOperation {
        match self {
            EdgeModifier::Swap(m) => EdgeOperation::Swap(m.modifier.clone()),
            EdgeModifier::Flip(m) => EdgeOperation::Flip(m.modifier.clone()),
        }
    }
}

/// 変更されたエッジ操作列
#[derive(Debug, Clone)]
pub struct ModifiedEdgeSequence {
    /// 元の操作列
    pub original_sequence: Vec<EdgeOperation>,
    /// 変更のリスト
    pub modifiers: Vec<EdgeModifier>,
}

impl ModifiedEdgeSequence {
    /// 新しいModifiedEdgeSequenceを作成
    pub fn new(original_sequence: Vec<EdgeOperation>) -> Self {
        Self {
            original_sequence,
            modifiers: Vec::new(),
        }
    }

    /// 変更を追加
    pub fn add_modifier(&mut self, modifier: EdgeModifier) {
        self.modifiers.push(modifier);
    }

    /// 実際の操作列を取得（変更を適用済み）
    pub fn get_sequence(&self) -> Vec<EdgeOperation> {
        let mut result = self.original_sequence.clone();

        // 全ての変更を適用
        for modifier in &self.modifiers {
            let step = modifier.step();
            if step < result.len() {
                result[step] = modifier.operation();
            }
        }

        result
    }

    /// 指定したステップが変更されているか確認
    pub fn is_modified(&self, step: usize) -> bool {
        self.modifiers.iter().any(|m| m.step() == step)
    }
}

/// 変更されたEdge操作を表示用にラップ
struct ModifiedEdgeOperation<'a> {
    operation: &'a EdgeOperation,
}

impl<'a> fmt::Display for ModifiedEdgeOperation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // **で囲んで強調表示
        write!(f, "**{}**", self.operation)
    }
}

impl fmt::Display for ModifiedEdgeSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sequence = self.get_sequence();

        for (i, op) in sequence.iter().enumerate() {
            if self.is_modified(i) {
                // 変更されたステップは**で強調
                writeln!(f, "Step {}: {}", i + 1, ModifiedEdgeOperation { operation: op })?;
            } else {
                // 変更されていないステップは通常表示
                writeln!(f, "Step {}: {}", i + 1, op)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modified_edge_sequence_basic() {
        let original = vec![
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 0, 0)),
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 1, 0)),
        ];

        let mut modified = ModifiedEdgeSequence::new(original.clone());

        // 変更を追加
        modified.add_modifier(EdgeModifier::Swap(EdgeSwapModifier {
            step: 0,
            modifier: EdgeSwapOperation::new(8, 2, 1),
        }));

        // 変更が反映されているか確認
        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 2);
        assert!(modified.is_modified(0));
        assert!(!modified.is_modified(1));

        // 元の操作列は変更されていないことを確認
        assert_eq!(modified.original_sequence, original);
    }

    #[test]
    fn test_modified_edge_sequence_display() {
        let original = vec![
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 0, 0)),
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 1, 0)),
            EdgeOperation::Flip(EdgeFlipOperation::new(2)),
        ];

        let mut modified = ModifiedEdgeSequence::new(original);

        // ステップ1を変更
        modified.add_modifier(EdgeModifier::Swap(EdgeSwapModifier {
            step: 1,
            modifier: EdgeSwapOperation::new(8, 5, 1),
        }));

        let display = format!("{}", modified);
        println!("{}", display);

        // **で囲まれていることを確認
        assert!(display.contains("**"));
        assert!(display.contains("Step 1:"));
        assert!(display.contains("Step 2:"));
        assert!(display.contains("Step 3:"));
    }

    #[test]
    fn test_edge_modifier_step() {
        let swap_mod = EdgeModifier::Swap(EdgeSwapModifier {
            step: 5,
            modifier: EdgeSwapOperation::new(8, 0, 0),
        });
        assert_eq!(swap_mod.step(), 5);

        let flip_mod = EdgeModifier::Flip(EdgeFlipModifier {
            step: 3,
            modifier: EdgeFlipOperation::new(2),
        });
        assert_eq!(flip_mod.step(), 3);
    }

    #[test]
    fn test_multiple_modifiers() {
        let original = vec![
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 0, 0)),
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 1, 0)),
            EdgeOperation::Swap(EdgeSwapOperation::new(8, 2, 0)),
        ];

        let mut modified = ModifiedEdgeSequence::new(original);

        // 複数の変更を追加
        modified.add_modifier(EdgeModifier::Swap(EdgeSwapModifier {
            step: 0,
            modifier: EdgeSwapOperation::new(8, 4, 1),
        }));
        modified.add_modifier(EdgeModifier::Swap(EdgeSwapModifier {
            step: 2,
            modifier: EdgeSwapOperation::new(8, 5, 1),
        }));

        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 3);
        assert!(modified.is_modified(0));
        assert!(!modified.is_modified(1));
        assert!(modified.is_modified(2));
    }
}
