use crate::cube::State;
use super::mixed_nearby_search::{MixedOperation, ApplyableToState};
use std::fmt;

/// Mixed操作の変更を表す
/// 
/// MixedOperationに新しい操作タイプを追加しても、この構造体は変更不要
#[derive(Debug, Clone, PartialEq)]
pub struct MixedModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しい操作
    pub modifier: MixedOperation,
}

impl MixedModifier {
    /// 新しいMixedModifierを作成
    pub fn new(step: usize, modifier: MixedOperation) -> Self {
        Self { step, modifier }
    }

    /// 変更対象のステップ番号を取得
    pub fn step(&self) -> usize {
        self.step
    }

    /// 変更後のMixedOperationを取得
    pub fn operation(&self) -> &MixedOperation {
        &self.modifier
    }
}

/// 変更された混合操作列
#[derive(Debug, Clone)]
pub struct ModifiedMixedSequence {
    /// 元の操作列
    pub original_sequence: Vec<MixedOperation>,
    /// 変更のリスト
    pub modifiers: Vec<MixedModifier>,
}

impl ModifiedMixedSequence {
    /// 新しいModifiedMixedSequenceを作成
    pub fn new(original_sequence: Vec<MixedOperation>) -> Self {
        Self {
            original_sequence,
            modifiers: Vec::new(),
        }
    }

    /// 変更を追加
    pub fn add_modifier(&mut self, modifier: MixedModifier) {
        self.modifiers.push(modifier);
    }

    /// 実際の操作列を取得（変更を適用済み）
    pub fn get_sequence(&self) -> Vec<MixedOperation> {
        let mut result = self.original_sequence.clone();

        // 全ての変更を適用
        for modifier in &self.modifiers {
            let step = modifier.step();
            if step < result.len() {
                result[step] = modifier.operation().clone();
            }
        }

        result
    }

    /// 変更の説明を取得
    pub fn get_description(&self) -> String {
        let descriptions: Vec<String> = self.modifiers
            .iter()
            .map(|modifier| format!("Step {}: {:?}", modifier.step() + 1, modifier))
            .collect();
        
        descriptions.join(", ")
    }

    /// 元の操作列の長さを取得
    pub fn len(&self) -> usize {
        self.original_sequence.len()
    }

    /// 空かどうかを判定
    pub fn is_empty(&self) -> bool {
        self.original_sequence.is_empty()
    }

    /// 変更された操作列を状態に適用
    pub fn apply_to_state(&self, initial_state: &State) -> State {
        let operations = self.get_sequence();
        let mut state = initial_state.clone();
        
        for operation in operations {
            state = operation.apply_to_state(&state);
        }
        
        state
    }

    /// 指定したステップが変更されているか確認
    pub fn is_modified(&self, step: usize) -> bool {
        self.modifiers.iter().any(|m| m.step() == step)
    }
}

impl fmt::Display for ModifiedMixedSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operations = self.get_sequence();
        for (i, op) in operations.iter().enumerate() {
            let is_modified = self.is_modified(i);
            let marker = if is_modified { "**" } else { "" };
            writeln!(f, "Step {}: {}{}{}", i + 1, marker, op, marker)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspection::{CornerSwapOperation, EdgeSwapOperation};

    #[test]
    fn test_mixed_modifier_basic() {
        let corner_swap = MixedOperation::CornerSwap(CornerSwapOperation::new(0, 1, 0));
        let modifier = MixedModifier::new(0, corner_swap.clone());
        
        assert_eq!(modifier.step(), 0);
        assert_eq!(modifier.operation(), &corner_swap);
    }

    #[test]
    fn test_modified_mixed_sequence() {
        let original = vec![
            MixedOperation::CornerSwap(CornerSwapOperation::new(0, 1, 0)),
            MixedOperation::EdgeSwap(EdgeSwapOperation::new(8, 0, 0)),
        ];

        let mut modified = ModifiedMixedSequence::new(original.clone());

        // 変更を追加
        let new_op = MixedOperation::CornerSwap(CornerSwapOperation::new(0, 2, 1));
        modified.add_modifier(MixedModifier::new(0, new_op));

        // 変更が反映されているか確認
        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 2);
        assert!(modified.is_modified(0));
        assert!(!modified.is_modified(1));

        // 元の操作列は変更されていないことを確認
        assert_eq!(modified.original_sequence, original);
    }

    #[test]
    fn test_multiple_modifiers() {
        let original = vec![
            MixedOperation::CornerSwap(CornerSwapOperation::new(0, 1, 0)),
            MixedOperation::EdgeSwap(EdgeSwapOperation::new(8, 0, 0)),
            MixedOperation::CornerSwap(CornerSwapOperation::new(0, 2, 0)),
        ];

        let mut modified = ModifiedMixedSequence::new(original);

        // 複数の変更を追加
        modified.add_modifier(MixedModifier::new(
            0,
            MixedOperation::CornerSwap(CornerSwapOperation::new(0, 3, 1)),
        ));
        modified.add_modifier(MixedModifier::new(
            2,
            MixedOperation::EdgeSwap(EdgeSwapOperation::new(8, 5, 1)),
        ));

        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 3);
        assert!(modified.is_modified(0));
        assert!(!modified.is_modified(1));
        assert!(modified.is_modified(2));
    }

    #[test]
    fn test_display_formatting() {
        let original = vec![
            MixedOperation::CornerSwap(CornerSwapOperation::new(0, 1, 0)),
            MixedOperation::EdgeSwap(EdgeSwapOperation::new(8, 0, 0)),
        ];

        let mut modified = ModifiedMixedSequence::new(original);
        modified.add_modifier(MixedModifier::new(
            1,
            MixedOperation::EdgeSwap(EdgeSwapOperation::new(8, 2, 1)),
        ));

        let display = format!("{}", modified);
        println!("{}", display);

        // **で囲まれていることを確認
        assert!(display.contains("**"));
        assert!(display.contains("Step 1:"));
        assert!(display.contains("Step 2:"));
    }
}
