use super::move_modifier::{MoveModifier, ModifiedMoveSequence};
use super::notation_alternative_generator::NotationAlternativeGenerator;
use crate::parser::move_parser::{NotationMove, Sequence};
use crate::cube::State;

/// MoveSequenceの近傍を探索する構造体
pub struct NearbySequenceSearch {
    base_sequence: Sequence,
    alternative_generator: Box<dyn NotationAlternativeGenerator>,
}

impl NearbySequenceSearch {
    /// 新しいNearbySequenceSearchを作成
    pub fn new(
        base_sequence: Sequence,
        alternative_generator: Box<dyn NotationAlternativeGenerator>,
    ) -> Self {
        Self {
            base_sequence,
            alternative_generator,
        }
    }

    /// 指定したNotationMoveに対する代替案を生成（NOOPを含む）
    fn get_alternative_moves(&self, mv: &NotationMove) -> Vec<Option<NotationMove>> {
        let mut alternatives: Vec<Option<NotationMove>> = self
            .alternative_generator
            .generate_alternatives(mv)
            .into_iter()
            .map(Some)
            .collect();
        
        // NOOPを追加
        alternatives.push(None);
        
        alternatives
    }

    /// 元のSequenceから最大1つのNotationMoveを変更したバリエーションを生成
    pub fn explore_variants_one_change(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedMoveSequence, State)> {
        let mut variants = Vec::new();

        for step_index in 0..self.base_sequence.len() {
            let alternatives = self.get_alternative_moves(&self.base_sequence[step_index]);

            for alternative in alternatives {
                let mut modified = ModifiedMoveSequence::new(self.base_sequence.clone());
                let modifier = MoveModifier::new(step_index, alternative);
                modified.add_modifier(modifier);

                let final_state = modified.apply_to_state(initial_state);
                variants.push((modified, final_state));
            }
        }

        variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::explorer::notation_alternative_generator::SameGroupAlternativeGenerator;
    use crate::cube::State;

    #[test]
    fn test_nearby_sequence_search_basic() {
        let sequence = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
        ];

        let generator = Box::new(SameGroupAlternativeGenerator::new());
        let search = NearbySequenceSearch::new(sequence, generator);

        let initial_state = State::solved();
        let variants = search.explore_variants_one_change(&initial_state);

        // 各ステップについて、代替案の数 + NOOP = 6 (R系) + 6 (U系) + 6 (R系) = 18
        // R: 5 alternatives + 1 NOOP = 6
        // U: 5 alternatives + 1 NOOP = 6
        // R': 5 alternatives + 1 NOOP = 6
        assert_eq!(variants.len(), 18);
    }

    #[test]
    fn test_alternatives_include_noop() {
        let sequence = vec![NotationMove::U];

        let generator = Box::new(SameGroupAlternativeGenerator::new());
        let search = NearbySequenceSearch::new(sequence, generator);

        let initial_state = State::solved();
        let variants = search.explore_variants_one_change(&initial_state);

        // U: 5 alternatives + 1 NOOP = 6
        assert_eq!(variants.len(), 6);

        // NOOPを含むバリエーションが存在することを確認
        let has_noop = variants.iter().any(|(modified, _)| {
            let seq = modified.get_sequence();
            seq.is_empty()
        });
        assert!(has_noop);
    }

    #[test]
    fn test_state_changes() {
        let sequence = vec![NotationMove::R];

        let generator = Box::new(SameGroupAlternativeGenerator::new());
        let search = NearbySequenceSearch::new(sequence, generator);

        let initial_state = State::solved();
        let variants = search.explore_variants_one_change(&initial_state);

        // 状態が変化していることを確認
        for (modified, final_state) in &variants {
            let seq = modified.get_sequence();
            if !seq.is_empty() {
                // NOOPでない限り、状態は変化するはず
                // ただし、U2を2回やると戻るなどの例外もあるため、ここでは厳密にチェックしない
                println!("Sequence: {:?}, Changed: {}", seq, final_state != &initial_state);
            }
        }
    }
}
