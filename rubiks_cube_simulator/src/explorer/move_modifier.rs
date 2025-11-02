use crate::parser::move_parser::{NotationMove, Sequence};
use crate::cube::State;
use std::fmt;

/// NotationMoveの変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct MoveModifier {
    /// 変更対象のステップ番号
    pub step: usize,
    /// 新しいNotationMove（Noopの場合も含む）
    pub new_move: NotationMove,
}

impl MoveModifier {
    /// 新しいMoveModifierを作成
    pub fn new(step: usize, new_move: NotationMove) -> Self {
        Self { step, new_move }
    }

    /// 変更対象のステップ番号を取得
    pub fn step(&self) -> usize {
        self.step
    }

    /// 変更後のNotationMoveを取得
    pub fn notation_move(&self) -> &NotationMove {
        &self.new_move
    }
}

/// 変更されたMoveSequence
#[derive(Debug, Clone)]
pub struct ModifiedMoveSequence {
    /// 元のSequence
    pub original_sequence: Sequence,
    /// 変更のリスト
    pub modifiers: Vec<MoveModifier>,
}

impl ModifiedMoveSequence {
    /// 新しいModifiedMoveSequenceを作成
    pub fn new(original_sequence: Sequence) -> Self {
        Self {
            original_sequence,
            modifiers: Vec::new(),
        }
    }

    /// 変更を追加
    pub fn add_modifier(&mut self, modifier: MoveModifier) {
        self.modifiers.push(modifier);
    }

    /// 実際のSequenceを取得（変更を適用済み）
    pub fn get_sequence(&self) -> Sequence {
        let mut result = Vec::new();

        for (i, mv) in self.original_sequence.iter().enumerate() {
            // この位置に変更があるか確認
            if let Some(modifier) = self.modifiers.iter().find(|m| m.step() == i) {
                // 変更がある場合
                let new_move = modifier.notation_move();
                if !matches!(new_move, NotationMove::Noop) {
                    result.push(new_move.clone());
                }
                // Noopの場合は何も追加しない
            } else {
                // 変更がない場合は元のムーブを追加
                result.push(mv.clone());
            }
        }

        result
    }

    /// 元のSequenceの長さを取得
    pub fn len(&self) -> usize {
        self.original_sequence.len()
    }

    /// 空かどうかを判定
    pub fn is_empty(&self) -> bool {
        self.original_sequence.is_empty()
    }

    /// 変更されたSequenceを状態に適用
    pub fn apply_to_state(&self, initial_state: &State) -> State {
        let sequence = self.get_sequence();
        let mut state = initial_state.clone();
        
        for mv in sequence {
            state = crate::cube::operations::apply_notation_move(&state, &mv);
        }
        
        state
    }

    /// 指定したステップが変更されているか確認
    pub fn is_modified(&self, step: usize) -> bool {
        self.modifiers.iter().any(|m| m.step() == step)
    }
}

impl fmt::Display for ModifiedMoveSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        
        let mut first = true;
        for (i, mv) in self.original_sequence.iter().enumerate() {
            let is_modified = self.modifiers.iter().any(|m| m.step() == i);
            
            if is_modified {
                // 変更されたムーブを探す
                if let Some(modifier) = self.modifiers.iter().find(|m| m.step() == i) {
                    let new_move = modifier.notation_move();
                    if matches!(new_move, NotationMove::Noop) {
                        // Noopの場合は **** と表示
                        if !first {
                            write!(f, " ")?;
                        }
                        write!(f, "****")?;
                        first = false;
                    } else {
                        // 通常の変更
                        if !first {
                            write!(f, " ")?;
                        }
                        write!(f, "**{}**", new_move.to_string())?;
                        first = false;
                    }
                }
            } else {
                if !first {
                    write!(f, " ")?;
                }
                write!(f, "{}", mv.to_string())?;
                first = false;
            }
        }
        
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_modifier_basic() {
        let modifier = MoveModifier::new(0, NotationMove::U);
        
        assert_eq!(modifier.step(), 0);
        assert_eq!(modifier.notation_move(), &NotationMove::U);
    }

    #[test]
    fn test_move_modifier_noop() {
        let modifier = MoveModifier::new(0, NotationMove::Noop);
        
        assert_eq!(modifier.step(), 0);
        assert_eq!(modifier.notation_move(), &NotationMove::Noop);
    }

    #[test]
    fn test_modified_move_sequence() {
        let original = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
        ];

        let mut modified = ModifiedMoveSequence::new(original.clone());

        // 変更を追加
        modified.add_modifier(MoveModifier::new(0, NotationMove::R2));

        // 変更が反映されているか確認
        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 3);
        assert_eq!(sequence[0], NotationMove::R2);
        assert_eq!(sequence[1], NotationMove::U);
        assert_eq!(sequence[2], NotationMove::RPrime);
        
        assert!(modified.is_modified(0));
        assert!(!modified.is_modified(1));
        assert!(!modified.is_modified(2));
    }

    #[test]
    fn test_noop_modifier() {
        let original = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
        ];

        let mut modified = ModifiedMoveSequence::new(original);

        // Noopを追加
        modified.add_modifier(MoveModifier::new(1, NotationMove::Noop));

        let sequence = modified.get_sequence();
        assert_eq!(sequence.len(), 2); // Uが削除される
        assert_eq!(sequence[0], NotationMove::R);
        assert_eq!(sequence[1], NotationMove::RPrime);
    }

    #[test]
    fn test_display_formatting() {
        let original = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
            NotationMove::D,
        ];

        let mut modified = ModifiedMoveSequence::new(original);
        modified.add_modifier(MoveModifier::new(0, NotationMove::R2));

        let display = format!("{}", modified);
        println!("Display output: {}", display);

        // **で囲まれていることを確認
        assert!(display.contains("**R2**"));
        assert!(display.contains("U"));
        assert!(display.contains("R'"));
        assert!(display.contains("D"));
    }

    #[test]
    fn test_display_with_noop() {
        let original = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
        ];

        let mut modified = ModifiedMoveSequence::new(original);
        modified.add_modifier(MoveModifier::new(1, NotationMove::Noop)); // Uを削除

        let display = format!("{}", modified);
        println!("Display output: {}", display);

        // **** が表示される
        assert!(display.contains("****"));
        assert!(display.contains("R"));
        assert!(display.contains("R'"));
        // Uは文字としては表示されない（****で置き換えられる）
    }
}
