use crate::parser::move_parser::NotationMove;

/// NotationMoveの代替案を生成するトレイト
pub trait NotationAlternativeGenerator {
    fn generate_alternatives(&self, mv: &NotationMove) -> Vec<NotationMove>;
}

/// 同一グループ内のNotationMoveの代替案を生成する
pub struct SameGroupAlternativeGenerator;

impl SameGroupAlternativeGenerator {
    pub fn new() -> Self {
        Self
    }

    /// NotationMoveが属するグループの全ての候補を返す
    fn get_group_alternatives(&self, mv: &NotationMove) -> Vec<NotationMove> {
        match mv {
            NotationMove::U | NotationMove::U2 | NotationMove::UPrime 
            | NotationMove::UWide | NotationMove::UWide2 | NotationMove::UWidePrime => {
                vec![
                    NotationMove::U,
                    NotationMove::UPrime,
                    NotationMove::U2,
                    NotationMove::UWide,
                    NotationMove::UWidePrime,
                    NotationMove::UWide2,
                ]
            }
            NotationMove::D | NotationMove::D2 | NotationMove::DPrime
            | NotationMove::DWide | NotationMove::DWide2 | NotationMove::DWidePrime => {
                vec![
                    NotationMove::D,
                    NotationMove::DPrime,
                    NotationMove::D2,
                    NotationMove::DWide,
                    NotationMove::DWidePrime,
                    NotationMove::DWide2,
                ]
            }
            NotationMove::L | NotationMove::L2 | NotationMove::LPrime
            | NotationMove::LWide | NotationMove::LWide2 | NotationMove::LWidePrime => {
                vec![
                    NotationMove::L,
                    NotationMove::LPrime,
                    NotationMove::L2,
                    NotationMove::LWide,
                    NotationMove::LWidePrime,
                    NotationMove::LWide2,
                ]
            }
            NotationMove::R | NotationMove::R2 | NotationMove::RPrime
            | NotationMove::RWide | NotationMove::RWide2 | NotationMove::RWidePrime => {
                vec![
                    NotationMove::R,
                    NotationMove::RPrime,
                    NotationMove::R2,
                    NotationMove::RWide,
                    NotationMove::RWidePrime,
                    NotationMove::RWide2,
                ]
            }
            NotationMove::F | NotationMove::F2 | NotationMove::FPrime
            | NotationMove::FWide | NotationMove::FWide2 | NotationMove::FWidePrime => {
                vec![
                    NotationMove::F,
                    NotationMove::FPrime,
                    NotationMove::F2,
                    NotationMove::FWide,
                    NotationMove::FWidePrime,
                    NotationMove::FWide2,
                ]
            }
            NotationMove::B | NotationMove::B2 | NotationMove::BPrime
            | NotationMove::BWide | NotationMove::BWide2 | NotationMove::BWidePrime => {
                vec![
                    NotationMove::B,
                    NotationMove::BPrime,
                    NotationMove::B2,
                    NotationMove::BWide,
                    NotationMove::BWidePrime,
                    NotationMove::BWide2,
                ]
            }
            NotationMove::M | NotationMove::M2 | NotationMove::MPrime => {
                vec![
                    NotationMove::M,
                    NotationMove::MPrime,
                    NotationMove::M2,
                ]
            }
            NotationMove::S | NotationMove::S2 | NotationMove::SPrime => {
                vec![
                    NotationMove::S,
                    NotationMove::SPrime,
                    NotationMove::S2,
                ]
            }
            NotationMove::E | NotationMove::E2 | NotationMove::EPrime => {
                vec![
                    NotationMove::E,
                    NotationMove::EPrime,
                    NotationMove::E2,
                ]
            }
        }
    }
}

impl NotationAlternativeGenerator for SameGroupAlternativeGenerator {
    fn generate_alternatives(&self, mv: &NotationMove) -> Vec<NotationMove> {
        self.get_group_alternatives(mv)
            .into_iter()
            .filter(|alt| alt != mv) // 元のムーブ自身は除外
            .collect()
    }
}

impl Default for SameGroupAlternativeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u_group_alternatives() {
        let generator = SameGroupAlternativeGenerator::new();
        let alternatives = generator.generate_alternatives(&NotationMove::U);
        
        assert_eq!(alternatives.len(), 5);
        assert!(alternatives.contains(&NotationMove::UPrime));
        assert!(alternatives.contains(&NotationMove::U2));
        assert!(alternatives.contains(&NotationMove::UWide));
        assert!(alternatives.contains(&NotationMove::UWidePrime));
        assert!(alternatives.contains(&NotationMove::UWide2));
        assert!(!alternatives.contains(&NotationMove::U)); // 元のムーブは含まれない
    }

    #[test]
    fn test_u2_alternatives() {
        let generator = SameGroupAlternativeGenerator::new();
        let alternatives = generator.generate_alternatives(&NotationMove::U2);
        
        assert_eq!(alternatives.len(), 5);
        assert!(alternatives.contains(&NotationMove::U));
        assert!(alternatives.contains(&NotationMove::UPrime));
        assert!(!alternatives.contains(&NotationMove::U2)); // 元のムーブは含まれない
    }

    #[test]
    fn test_m_group_alternatives() {
        let generator = SameGroupAlternativeGenerator::new();
        let alternatives = generator.generate_alternatives(&NotationMove::M);
        
        assert_eq!(alternatives.len(), 2);
        assert!(alternatives.contains(&NotationMove::MPrime));
        assert!(alternatives.contains(&NotationMove::M2));
        assert!(!alternatives.contains(&NotationMove::M));
    }

    #[test]
    fn test_all_groups_have_alternatives() {
        let generator = SameGroupAlternativeGenerator::new();
        
        // 各グループから1つずつテスト
        let test_moves = vec![
            NotationMove::U,
            NotationMove::D,
            NotationMove::L,
            NotationMove::R,
            NotationMove::F,
            NotationMove::B,
            NotationMove::M,
            NotationMove::S,
            NotationMove::E,
        ];
        
        for mv in test_moves {
            let alternatives = generator.generate_alternatives(&mv);
            assert!(!alternatives.is_empty(), "Move {:?} should have alternatives", mv);
        }
    }
}
