use crate::cube::State;
use crate::explorer::{
    CollectionModifier, ModifiedMoveSequenceCollection, NearbySequenceSearch,
    SameGroupAlternativeGenerator,
};
use crate::parser::move_parser::Sequence;

/// MoveSequenceCollectionの代替探索ワークフロー
pub struct NearbySequenceSearchWorkflow {
    sequences: Vec<Sequence>,
}

impl NearbySequenceSearchWorkflow {
    /// 新しいワークフローを作成
    pub fn new(sequences: Vec<Sequence>) -> Self {
        Self { sequences }
    }

    /// Before → After を満たす代替手順を探索
    /// Collection全体で1つのNotationMoveを変更したバリエーションを探索
    pub fn find_alternatives(
        &self,
        before_state: &State,
        after_state: &State,
    ) -> Vec<ModifiedMoveSequenceCollection> {
        let mut results = Vec::new();

        // 各Sequenceについて探索
        for (seq_index, sequence) in self.sequences.iter().enumerate() {
            let generator = Box::new(SameGroupAlternativeGenerator::new());
            let search = NearbySequenceSearch::new(sequence.clone(), generator);

            let variants = search.explore_variants_one_change(before_state);

            // このSequenceだけを変更したCollectionを作成
            for (modified_seq, _) in variants {
                let mut modified_collection =
                    ModifiedMoveSequenceCollection::new(self.sequences.clone());
                modified_collection.add_modifier(CollectionModifier::new(seq_index, modified_seq));

                // Collection全体を適用して、Before → After を満たすかチェック
                let final_state = modified_collection.apply_to_state(before_state);
                if &final_state == after_state {
                    results.push(modified_collection);
                }
            }
        }

        results
    }

    /// Before → After を満たす代替手順を探索し、詳細情報を返す
    pub fn find_alternatives_with_details(
        &self,
        before_state: &State,
        after_state: &State,
    ) -> Vec<AlternativeResult> {
        let mut results = Vec::new();

        for (seq_index, sequence) in self.sequences.iter().enumerate() {
            let generator = Box::new(SameGroupAlternativeGenerator::new());
            let search = NearbySequenceSearch::new(sequence.clone(), generator);

            let variants = search.explore_variants_one_change(before_state);

            for (modified_seq, _) in variants {
                let mut modified_collection =
                    ModifiedMoveSequenceCollection::new(self.sequences.clone());
                modified_collection.add_modifier(CollectionModifier::new(seq_index, modified_seq));

                let final_state = modified_collection.apply_to_state(before_state);
                if &final_state == after_state {
                    results.push(AlternativeResult {
                        modified_collection,
                        final_state,
                    });
                }
            }
        }

        results
    }
}

/// 代替手順の探索結果
#[derive(Debug)]
pub struct AlternativeResult {
    /// 変更されたCollection
    pub modified_collection: ModifiedMoveSequenceCollection,
    /// 最終状態
    pub final_state: State,
}

impl AlternativeResult {
    /// 結果を表示用の文字列に変換
    pub fn to_display_string(&self) -> String {
        format!("{}", self.modified_collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::move_parser::NotationMove;

    #[test]
    fn test_workflow_basic() {
        // テストケースのBefore State
        let before_state = State::new(
            [0, 1, 6, 3, 4, 5, 7, 2],
            [0, 0, 1, 0, 0, 0, 0, 2],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // テストケースのAfter State
        let after_state = State::new(
            [0, 6, 2, 3, 4, 1, 7, 5],
            [0, 2, 0, 0, 0, 2, 1, 1],
            [0, 5, 9, 3, 4, 2, 6, 7, 8, 1, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // Original Move Sequence: R U R' D R U' R' D'
        // 注意: ドキュメントでは {R U R' D R U' R' D'} となっているので、最後はR'
        let original_sequence = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
            NotationMove::D,
            NotationMove::R,
            NotationMove::UPrime,
            NotationMove::RPrime,
            NotationMove::DPrime,
        ];

        let sequences = vec![original_sequence.clone()];
        let workflow = NearbySequenceSearchWorkflow::new(sequences);

        let results = workflow.find_alternatives(&before_state, &after_state);

        // 結果を表示
        println!("Found {} alternatives", results.len());
        for modified_collection in &results {
            println!("{}", modified_collection);
        }

        // 少なくとも1つは見つかるはず
        assert!(!results.is_empty(), "No alternatives found");

        // R2 U R' D R U' R' D' が含まれているか確認
        let target_found = results.iter().any(|modified_collection| {
            let collection = modified_collection.get_collection();
            if collection.len() != 1 {
                return false;
            }
            let seq = &collection[0];
            seq.len() == 8
                && seq[0] == NotationMove::R2
                && seq[1] == NotationMove::U
                && seq[2] == NotationMove::RPrime
                && seq[3] == NotationMove::D
                && seq[4] == NotationMove::R
                && seq[5] == NotationMove::UPrime
                && seq[6] == NotationMove::RPrime
                && seq[7] == NotationMove::DPrime
        });

        assert!(
            target_found,
            "Target sequence R2 U R' D R U' R' D' not found"
        );
    }

    #[test]
    fn test_workflow_with_details() {
        let before_state = State::new(
            [0, 1, 6, 3, 4, 5, 7, 2],
            [0, 0, 1, 0, 0, 0, 0, 2],
            [0, 1, 2, 3, 6, 5, 10, 7, 8, 9, 4, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // テストケースのAfter State
        let after_state = State::new(
            [0, 6, 2, 3, 4, 1, 7, 5],
            [0, 2, 0, 0, 0, 2, 1, 1],
            [0, 5, 9, 3, 4, 2, 6, 7, 8, 1, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // Original Move Sequence: R U R' D R U' R D'
        let original_sequence1 = vec![
            NotationMove::MPrime,
            NotationMove::U2,
            NotationMove::M,
            NotationMove::U2,
        ];

        let original_sequence2 = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
            NotationMove::D,
            NotationMove::R,
            NotationMove::UPrime,
            NotationMove::RPrime,
            NotationMove::DPrime,
        ];

        let sequences = vec![original_sequence1, original_sequence2];
        let workflow = NearbySequenceSearchWorkflow::new(sequences);

        let results = workflow.find_alternatives(&before_state, &after_state);

        println!("Found {} alternatives with details", results.len());
        for result in &results {
            println!("{}", result);
        }

        assert!(!results.is_empty());
    }

    #[test]
    fn test_workflow_with_details_noop() {
        let before_state = State::new(
            [0, 1, 6, 3, 4, 5, 7, 2],
            [0, 0, 1, 0, 0, 0, 0, 2],
            [0, 1, 2, 3, 6, 5, 10, 7, 8, 9, 4, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // テストケースのAfter State
        let after_state = State::new(
            [0, 5, 2, 3, 4, 7, 1, 6],
            [0, 1, 0, 0, 0, 2, 1, 2],
            [0, 9, 5, 3, 4, 1, 6, 7, 8, 2, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // Original Move Sequence: R U R' D R U' R D'
        let original_sequence1 = vec![
            NotationMove::MPrime,
            NotationMove::U2,
            NotationMove::M,
            NotationMove::U2,
        ];

        let original_sequence2 = vec![
            NotationMove::R,
            NotationMove::U,
            NotationMove::RPrime,
            NotationMove::D,
            NotationMove::R,
            NotationMove::UPrime,
            NotationMove::RPrime,
            NotationMove::DPrime,
        ];

        let sequences = vec![original_sequence1, original_sequence2];
        let workflow = NearbySequenceSearchWorkflow::new(sequences);

        let results = workflow.find_alternatives(&before_state, &after_state);

        println!("Found {} alternatives with details", results.len());
        for result in &results {
            println!("{}", result);
        }

        assert!(!results.is_empty());
    }

    #[test]
    fn test_workflow_multiple_sequences() {
        // 複数のSequenceを持つCollectionのテスト
        let before_state = State::solved();

        // 簡単なテスト: R と U の2つのSequence
        let sequences = vec![vec![NotationMove::R], vec![NotationMove::U]];

        let workflow = NearbySequenceSearchWorkflow::new(sequences);

        // R2 U を適用した状態
        let mut after_state = State::solved();
        after_state = crate::cube::operations::apply_notation_move(&after_state, &NotationMove::R2);
        after_state = crate::cube::operations::apply_notation_move(&after_state, &NotationMove::U);

        let results = workflow.find_alternatives(&before_state, &after_state);

        println!(
            "Found {} alternatives for multiple sequences",
            results.len()
        );
        for modified_collection in &results {
            println!("{}", modified_collection);
        }

        // R→R2の変更が見つかるはず
        let target_found = results.iter().any(|modified_collection| {
            let collection = modified_collection.get_collection();
            collection.len() == 2
                && collection[0].len() == 1
                && collection[0][0] == NotationMove::R2
                && collection[1].len() == 1
                && collection[1][0] == NotationMove::U
        });

        assert!(target_found, "Expected R2 U combination not found");
    }
}
