use crate::parser::move_parser::Sequence;
use crate::explorer::ModifiedMoveSequence;
use crate::cube::State;
use std::fmt;

/// MoveSequenceCollectionの変更を表す
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionModifier {
    /// 変更対象のSequenceのインデックス
    pub sequence_index: usize,
    /// 変更されたSequence
    pub modified_sequence: ModifiedMoveSequence,
}

impl CollectionModifier {
    /// 新しいCollectionModifierを作成
    pub fn new(sequence_index: usize, modified_sequence: ModifiedMoveSequence) -> Self {
        Self {
            sequence_index,
            modified_sequence,
        }
    }

    /// Sequenceインデックスを取得
    pub fn sequence_index(&self) -> usize {
        self.sequence_index
    }

    /// ModifiedMoveSequenceを取得
    pub fn modified_sequence(&self) -> &ModifiedMoveSequence {
        &self.modified_sequence
    }
}

/// 変更されたMoveSequenceCollection
#[derive(Debug, Clone)]
pub struct ModifiedMoveSequenceCollection {
    /// 元のMoveSequenceCollection
    pub original_collection: Vec<Sequence>,
    /// 変更のリスト
    pub modifiers: Vec<CollectionModifier>,
}

impl ModifiedMoveSequenceCollection {
    /// 新しいModifiedMoveSequenceCollectionを作成
    pub fn new(original_collection: Vec<Sequence>) -> Self {
        Self {
            original_collection,
            modifiers: Vec::new(),
        }
    }

    /// 変更を追加
    pub fn add_modifier(&mut self, modifier: CollectionModifier) {
        self.modifiers.push(modifier);
    }

    /// 実際のMoveSequenceCollectionを取得（変更を適用済み）
    pub fn get_collection(&self) -> Vec<Sequence> {
        let mut result = Vec::new();

        for (i, original_seq) in self.original_collection.iter().enumerate() {
            // この位置に変更があるか確認
            if let Some(modifier) = self.modifiers.iter().find(|m| m.sequence_index() == i) {
                // 変更がある場合
                result.push(modifier.modified_sequence().get_sequence());
            } else {
                // 変更がない場合は元のSequenceを追加
                result.push(original_seq.clone());
            }
        }

        result
    }

    /// 元のCollectionの長さを取得
    pub fn len(&self) -> usize {
        self.original_collection.len()
    }

    /// 空かどうかを判定
    pub fn is_empty(&self) -> bool {
        self.original_collection.is_empty()
    }

    /// 変更されたCollectionを状態に適用
    pub fn apply_to_state(&self, initial_state: &State) -> State {
        let collection = self.get_collection();
        let mut state = initial_state.clone();
        
        for sequence in collection {
            for mv in sequence {
                state = crate::cube::operations::apply_notation_move(&state, &mv);
            }
        }
        
        state
    }

    /// 指定したSequenceインデックスが変更されているか確認
    pub fn is_modified(&self, sequence_index: usize) -> bool {
        self.modifiers.iter().any(|m| m.sequence_index() == sequence_index)
    }
}

impl fmt::Display for ModifiedMoveSequenceCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ModifiedMoveSequenceCollection:")?;
        
        for (i, original_seq) in self.original_collection.iter().enumerate() {
            write!(f, "  Sequence #{}: ", i + 1)?;
            
            if let Some(modifier) = self.modifiers.iter().find(|m| m.sequence_index() == i) {
                // 変更されたSequence
                writeln!(f, "{}", modifier.modified_sequence())?;
            } else {
                // 元のSequence
                write!(f, "{{")?;
                for (j, mv) in original_seq.iter().enumerate() {
                    if j > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", mv.to_string())?;
                }
                writeln!(f, "}}")?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::move_parser::NotationMove;
    use crate::explorer::MoveModifier;

    #[test]
    fn test_collection_modifier_basic() {
        let original_seq = vec![NotationMove::R, NotationMove::U];
        let mut modified_seq = ModifiedMoveSequence::new(original_seq);
        modified_seq.add_modifier(MoveModifier::new(0, NotationMove::R2));

        let modifier = CollectionModifier::new(0, modified_seq);
        
        assert_eq!(modifier.sequence_index(), 0);
    }

    #[test]
    fn test_modified_collection_basic() {
        let original_collection = vec![
            vec![NotationMove::R, NotationMove::U],
            vec![NotationMove::D, NotationMove::L],
        ];

        let mut modified_collection = ModifiedMoveSequenceCollection::new(original_collection.clone());

        // 最初のSequenceを変更
        let mut modified_seq = ModifiedMoveSequence::new(original_collection[0].clone());
        modified_seq.add_modifier(MoveModifier::new(0, NotationMove::R2));
        modified_collection.add_modifier(CollectionModifier::new(0, modified_seq));

        // 変更が反映されているか確認
        let collection = modified_collection.get_collection();
        assert_eq!(collection.len(), 2);
        assert_eq!(collection[0][0], NotationMove::R2);
        assert_eq!(collection[0][1], NotationMove::U);
        assert_eq!(collection[1][0], NotationMove::D);
        assert_eq!(collection[1][1], NotationMove::L);
        
        assert!(modified_collection.is_modified(0));
        assert!(!modified_collection.is_modified(1));
    }

    #[test]
    fn test_apply_to_state() {
        let original_collection = vec![
            vec![NotationMove::R],
            vec![NotationMove::U],
        ];

        let modified_collection = ModifiedMoveSequenceCollection::new(original_collection);

        let initial_state = State::solved();
        let final_state = modified_collection.apply_to_state(&initial_state);

        // R U を適用した状態になっているはず
        assert_ne!(final_state, initial_state);
    }

    #[test]
    fn test_display_formatting() {
        let original_collection = vec![
            vec![NotationMove::R, NotationMove::U],
            vec![NotationMove::D, NotationMove::L],
        ];

        let mut modified_collection = ModifiedMoveSequenceCollection::new(original_collection.clone());

        let mut modified_seq = ModifiedMoveSequence::new(original_collection[0].clone());
        modified_seq.add_modifier(MoveModifier::new(0, NotationMove::R2));
        modified_collection.add_modifier(CollectionModifier::new(0, modified_seq));

        let display = format!("{}", modified_collection);
        println!("{}", display);

        assert!(display.contains("**R2**"));
        assert!(display.contains("Sequence #1"));
        assert!(display.contains("Sequence #2"));
    }
}
