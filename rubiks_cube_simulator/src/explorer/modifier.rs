use crate::inspection::{CornerOperation, CornerSwapOperation, CornerTwistOperation};
use std::fmt;

/// Swap操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct SwapModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいSwap操作
    pub modifier: CornerSwapOperation,
}

/// Twist操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct TwistModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいTwist操作
    pub modifier: CornerTwistOperation,
}

/// Corner操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub enum CornerModifier {
    Swap(SwapModifier),
    Twist(TwistModifier),
}

impl CornerModifier {
    /// 変更対象のステップ番号を取得
    pub fn step(&self) -> usize {
        match self {
            CornerModifier::Swap(m) => m.step,
            CornerModifier::Twist(m) => m.step,
        }
    }

    /// 変更後のCornerOperationを取得
    pub fn operation(&self) -> CornerOperation {
        match self {
            CornerModifier::Swap(m) => CornerOperation::Swap(m.modifier.clone()),
            CornerModifier::Twist(m) => CornerOperation::Twist(m.modifier.clone()),
        }
    }
}

/// 変更された操作列
#[derive(Debug, Clone)]
pub struct ModifiedSequence {
    /// 元の操作列
    pub original_sequence: Vec<CornerOperation>,
    /// 変更のリスト
    pub modifiers: Vec<CornerModifier>,
}

impl ModifiedSequence {
    /// 新しいModifiedSequenceを作成
    pub fn new(original_sequence: Vec<CornerOperation>) -> Self {
        Self {
            original_sequence,
            modifiers: Vec::new(),
        }
    }

    /// 変更を追加
    pub fn add_modifier(&mut self, modifier: CornerModifier) {
        self.modifiers.push(modifier);
    }

    /// 実際の操作列を取得（変更を適用済み）
    pub fn get_sequence(&self) -> Vec<CornerOperation> {
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

/// 変更されたCorner操作を表示用にラップ
struct ModifiedCornerOperation<'a> {
    operation: &'a CornerOperation,
}

impl<'a> fmt::Display for ModifiedCornerOperation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // **で囲んで強調表示
        write!(f, "**{}**", self.operation)
    }
}

impl fmt::Display for ModifiedSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sequence = self.get_sequence();

        for (i, op) in sequence.iter().enumerate() {
            if self.is_modified(i) {
                // 変更されたステップは**で強調
                writeln!(f, "Step {}: {}", i + 1, ModifiedCornerOperation { operation: op })?;
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
    fn test_modified_sequence_basic() {
        let original = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)),
        ];

        let mut modified = ModifiedSequence::new(original.clone());

        // 変更を追加
        modified.add_modifier(CornerModifier::Swap(SwapModifier {
            step: 0,
            modifier: CornerSwapOperation::new(0, 3, 1),
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
    fn test_modified_sequence_display() {
        let original = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)),
            CornerOperation::Twist(CornerTwistOperation::new(0, 1)),
        ];

        let mut modified = ModifiedSequence::new(original);

        // ステップ1を変更
        modified.add_modifier(CornerModifier::Swap(SwapModifier {
            step: 1,
            modifier: CornerSwapOperation::new(0, 5, 2),
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
    fn test_corner_modifier_step() {
        let swap_mod = CornerModifier::Swap(SwapModifier {
            step: 5,
            modifier: CornerSwapOperation::new(0, 1, 0),
        });
        assert_eq!(swap_mod.step(), 5);

        let twist_mod = CornerModifier::Twist(TwistModifier {
            step: 3,
            modifier: CornerTwistOperation::new(2, 1),
        });
        assert_eq!(twist_mod.step(), 3);
    }

    #[test]
    fn test_multiple_modifiers() {
        let original = vec![
            CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)),
            CornerOperation::Swap(CornerSwapOperation::new(0, 3, 0)),
        ];

        let mut modified = ModifiedSequence::new(original);

        // 複数の変更を追加
        modified.add_modifier(CornerModifier::Swap(SwapModifier {
            step: 0,
            modifier: CornerSwapOperation::new(0, 4, 1),
        }));
        modified.add_modifier(CornerModifier::Swap(SwapModifier {
            step: 2,
            modifier: CornerSwapOperation::new(0, 5, 2),
        }));

        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 3);
        assert!(modified.is_modified(0));
        assert!(!modified.is_modified(1));
        assert!(modified.is_modified(2));
    }
}
