use crate::cube::State;
use super::mixed_nearby_search::{MixedOperation, ApplyableToState};
use crate::inspection::{CornerSwapOperation, CornerTwistOperation, EdgeSwapOperation, EdgeFlipOperation};
use std::fmt;

/// Corner Swap操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct MixedCornerSwapModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいCorner Swap操作
    pub modifier: CornerSwapOperation,
}

/// Corner Twist操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct MixedCornerTwistModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいCorner Twist操作
    pub modifier: CornerTwistOperation,
}

/// Edge Swap操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct MixedEdgeSwapModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいEdge Swap操作
    pub modifier: EdgeSwapOperation,
}

/// Edge Flip操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct MixedEdgeFlipModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいEdge Flip操作
    pub modifier: EdgeFlipOperation,
}

/// Mixed操作の変更を表す
#[derive(Debug, Clone, PartialEq)]
pub enum MixedModifier {
    CornerSwap(MixedCornerSwapModifier),
    CornerTwist(MixedCornerTwistModifier),
    EdgeSwap(MixedEdgeSwapModifier),
    EdgeFlip(MixedEdgeFlipModifier),
}

impl MixedModifier {
    /// 変更対象のステップ番号を取得
    pub fn step(&self) -> usize {
        match self {
            MixedModifier::CornerSwap(m) => m.step,
            MixedModifier::CornerTwist(m) => m.step,
            MixedModifier::EdgeSwap(m) => m.step,
            MixedModifier::EdgeFlip(m) => m.step,
        }
    }

    /// 変更後のMixedOperationを取得
    pub fn operation(&self) -> MixedOperation {
        match self {
            MixedModifier::CornerSwap(m) => MixedOperation::CornerSwap(m.modifier.clone()),
            MixedModifier::CornerTwist(m) => MixedOperation::CornerTwist(m.modifier.clone()),
            MixedModifier::EdgeSwap(m) => MixedOperation::EdgeSwap(m.modifier.clone()),
            MixedModifier::EdgeFlip(m) => MixedOperation::EdgeFlip(m.modifier.clone()),
        }
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
                result[step] = modifier.operation();
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
}

impl fmt::Display for ModifiedMixedSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operations = self.get_sequence();
        for (i, op) in operations.iter().enumerate() {
            let is_modified = self.modifiers.iter().any(|m| m.step() == i);
            let marker = if is_modified { "**" } else { "" };
            writeln!(f, "Step {}: {}{}{}", i + 1, marker, op, marker)?;
        }
        Ok(())
    }
}