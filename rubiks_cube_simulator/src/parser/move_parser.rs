/// Moveの定義
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationMove {
    U,
    U2,
    UPrime,
    UWide,
    UWide2,
    UWidePrime,
    D,
    D2,
    DPrime,
    DWide,
    DWide2,
    DWidePrime,
    R,
    R2,
    RPrime,
    RWide,
    RWide2,
    RWidePrime,
    L,
    L2,
    LPrime,
    LWide,
    LWide2,
    LWidePrime,
    F,
    F2,
    FPrime,
    FWide,
    FWide2,
    FWidePrime,
    B,
    B2,
    BPrime,
    BWide,
    BWide2,
    BWidePrime,
    M,
    M2,
    MPrime,
    S,
    S2,
    SPrime,
    E,
    E2,
    EPrime,
    /// NOOP（何もしない操作、空文字列""に相当）
    Noop,
}

impl NotationMove {
    /// 文字列からNotationMoveをパース
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "" => Ok(NotationMove::Noop),
            "U" => Ok(NotationMove::U),
            "U2" => Ok(NotationMove::U2),
            "U2'" => Ok(NotationMove::U2),
            "U'" => Ok(NotationMove::UPrime),
            "u" => Ok(NotationMove::UWide),
            "u2" => Ok(NotationMove::UWide2),
            "u2'" => Ok(NotationMove::UWide2),
            "u'" => Ok(NotationMove::UWidePrime),
            "D" => Ok(NotationMove::D),
            "D2" => Ok(NotationMove::D2),
            "D2'" => Ok(NotationMove::D2),
            "D'" => Ok(NotationMove::DPrime),
            "d" => Ok(NotationMove::DWide),
            "d2" => Ok(NotationMove::DWide2),
            "d2'" => Ok(NotationMove::DWide2),
            "d'" => Ok(NotationMove::DWidePrime),
            "R" => Ok(NotationMove::R),
            "R2" => Ok(NotationMove::R2),
            "R2'" => Ok(NotationMove::R2),
            "R'" => Ok(NotationMove::RPrime),
            "r" => Ok(NotationMove::RWide),
            "r2" => Ok(NotationMove::RWide2),
            "r2'" => Ok(NotationMove::RWide2),
            "r'" => Ok(NotationMove::RWidePrime),
            "L" => Ok(NotationMove::L),
            "L2" => Ok(NotationMove::L2),
            "L2'" => Ok(NotationMove::L2),
            "L'" => Ok(NotationMove::LPrime),
            "l" => Ok(NotationMove::LWide),
            "l2" => Ok(NotationMove::LWide2),
            "l2'" => Ok(NotationMove::LWide2),
            "l'" => Ok(NotationMove::LWidePrime),
            "F" => Ok(NotationMove::F),
            "F2" => Ok(NotationMove::F2),
            "F2'" => Ok(NotationMove::F2),
            "F'" => Ok(NotationMove::FPrime),
            "f" => Ok(NotationMove::FWide),
            "f2" => Ok(NotationMove::FWide2),
            "f2'" => Ok(NotationMove::FWide2),
            "f'" => Ok(NotationMove::FWidePrime),
            "B" => Ok(NotationMove::B),
            "B2" => Ok(NotationMove::B2),
            "B2'" => Ok(NotationMove::B2),
            "B'" => Ok(NotationMove::BPrime),
            "b" => Ok(NotationMove::BWide),
            "b2" => Ok(NotationMove::BWide2),
            "b2'" => Ok(NotationMove::BWide2),
            "b'" => Ok(NotationMove::BWidePrime),
            "M" => Ok(NotationMove::M),
            "M2" => Ok(NotationMove::M2),
            "M2'" => Ok(NotationMove::M2),
            "M'" => Ok(NotationMove::MPrime),
            "S" => Ok(NotationMove::S),
            "S2" => Ok(NotationMove::S2),
            "S2'" => Ok(NotationMove::S2),
            "S'" => Ok(NotationMove::SPrime),
            "E" => Ok(NotationMove::E),
            "E2" => Ok(NotationMove::E2),
            "E2'" => Ok(NotationMove::E2),
            "E'" => Ok(NotationMove::EPrime),
            _ => Err(format!("Unknown move1: {}", s)),
        }
    }

    /// NotationMoveを文字列に変換
    pub fn to_string(&self) -> String {
        match self {
            NotationMove::Noop => "".to_string(),
            NotationMove::U => "U".to_string(),
            NotationMove::U2 => "U2".to_string(),
            NotationMove::UPrime => "U'".to_string(),
            NotationMove::UWide => "u".to_string(),
            NotationMove::UWide2 => "u2".to_string(),
            NotationMove::UWidePrime => "u'".to_string(),
            NotationMove::D => "D".to_string(),
            NotationMove::D2 => "D2".to_string(),
            NotationMove::DPrime => "D'".to_string(),
            NotationMove::DWide => "d".to_string(),
            NotationMove::DWide2 => "d2".to_string(),
            NotationMove::DWidePrime => "d'".to_string(),
            NotationMove::R => "R".to_string(),
            NotationMove::R2 => "R2".to_string(),
            NotationMove::RPrime => "R'".to_string(),
            NotationMove::RWide => "r".to_string(),
            NotationMove::RWide2 => "r2".to_string(),
            NotationMove::RWidePrime => "r'".to_string(),
            NotationMove::L => "L".to_string(),
            NotationMove::L2 => "L2".to_string(),
            NotationMove::LPrime => "L'".to_string(),
            NotationMove::LWide => "l".to_string(),
            NotationMove::LWide2 => "l2".to_string(),
            NotationMove::LWidePrime => "l'".to_string(),
            NotationMove::F => "F".to_string(),
            NotationMove::F2 => "F2".to_string(),
            NotationMove::FPrime => "F'".to_string(),
            NotationMove::FWide => "f".to_string(),
            NotationMove::FWide2 => "f2".to_string(),
            NotationMove::FWidePrime => "f'".to_string(),
            NotationMove::B => "B".to_string(),
            NotationMove::B2 => "B2".to_string(),
            NotationMove::BPrime => "B'".to_string(),
            NotationMove::BWide => "b".to_string(),
            NotationMove::BWide2 => "b2".to_string(),
            NotationMove::BWidePrime => "b'".to_string(),
            NotationMove::M => "M".to_string(),
            NotationMove::M2 => "M2".to_string(),
            NotationMove::MPrime => "M'".to_string(),
            NotationMove::S => "S".to_string(),
            NotationMove::S2 => "S2".to_string(),
            NotationMove::SPrime => "S'".to_string(),
            NotationMove::E => "E".to_string(),
            NotationMove::E2 => "E2".to_string(),
            NotationMove::EPrime => "E'".to_string(),
        }
    }

    /// NotationMoveの逆操作を返す
    pub fn reversed(&self) -> Self {
        match self {
            NotationMove::Noop => NotationMove::Noop,
            NotationMove::U => NotationMove::UPrime,
            NotationMove::UPrime => NotationMove::U,
            NotationMove::U2 => NotationMove::U2,
            NotationMove::UWide => NotationMove::UWidePrime,
            NotationMove::UWidePrime => NotationMove::UWide,
            NotationMove::UWide2 => NotationMove::UWide2,
            NotationMove::D => NotationMove::DPrime,
            NotationMove::DPrime => NotationMove::D,
            NotationMove::D2 => NotationMove::D2,
            NotationMove::DWide => NotationMove::DWidePrime,
            NotationMove::DWidePrime => NotationMove::DWide,
            NotationMove::DWide2 => NotationMove::DWide2,
            NotationMove::R => NotationMove::RPrime,
            NotationMove::RPrime => NotationMove::R,
            NotationMove::R2 => NotationMove::R2,
            NotationMove::RWide => NotationMove::RWidePrime,
            NotationMove::RWidePrime => NotationMove::RWide,
            NotationMove::RWide2 => NotationMove::RWide2,
            NotationMove::L => NotationMove::LPrime,
            NotationMove::LPrime => NotationMove::L,
            NotationMove::L2 => NotationMove::L2,
            NotationMove::LWide => NotationMove::LWidePrime,
            NotationMove::LWidePrime => NotationMove::LWide,
            NotationMove::LWide2 => NotationMove::LWide2,
            NotationMove::F => NotationMove::FPrime,
            NotationMove::FPrime => NotationMove::F,
            NotationMove::F2 => NotationMove::F2,
            NotationMove::FWide => NotationMove::FWidePrime,
            NotationMove::FWidePrime => NotationMove::FWide,
            NotationMove::FWide2 => NotationMove::FWide2,
            NotationMove::B => NotationMove::BPrime,
            NotationMove::BPrime => NotationMove::B,
            NotationMove::B2 => NotationMove::B2,
            NotationMove::BWide => NotationMove::BWidePrime,
            NotationMove::BWidePrime => NotationMove::BWide,
            NotationMove::BWide2 => NotationMove::BWide2,
            NotationMove::M => NotationMove::MPrime,
            NotationMove::MPrime => NotationMove::M,
            NotationMove::M2 => NotationMove::M2,
            NotationMove::S => NotationMove::SPrime,
            NotationMove::SPrime => NotationMove::S,
            NotationMove::S2 => NotationMove::S2,
            NotationMove::E => NotationMove::EPrime,
            NotationMove::EPrime => NotationMove::E,
            NotationMove::E2 => NotationMove::E2,
        }
    }

    /// NotationMoveを2倍にする（180度回転にする）
    pub fn doubled(&self) -> Self {
        match self {
            NotationMove::Noop => NotationMove::Noop,
            NotationMove::U | NotationMove::UPrime => NotationMove::U2,
            NotationMove::UWide | NotationMove::UWidePrime => NotationMove::UWide2,
            NotationMove::D | NotationMove::DPrime => NotationMove::D2,
            NotationMove::DWide | NotationMove::DWidePrime => NotationMove::DWide2,
            NotationMove::R | NotationMove::RPrime => NotationMove::R2,
            NotationMove::RWide | NotationMove::RWidePrime => NotationMove::RWide2,
            NotationMove::L | NotationMove::LPrime => NotationMove::L2,
            NotationMove::LWide | NotationMove::LWidePrime => NotationMove::LWide2,
            NotationMove::F | NotationMove::FPrime => NotationMove::F2,
            NotationMove::FWide | NotationMove::FWidePrime => NotationMove::FWide2,
            NotationMove::B | NotationMove::BPrime => NotationMove::B2,
            NotationMove::BWide | NotationMove::BWidePrime => NotationMove::BWide2,
            NotationMove::M | NotationMove::MPrime => NotationMove::M2,
            NotationMove::S | NotationMove::SPrime => NotationMove::S2,
            NotationMove::E | NotationMove::EPrime => NotationMove::E2,
            // すでに2回転の場合はそのまま
            m @ (NotationMove::U2
            | NotationMove::UWide2
            | NotationMove::D2
            | NotationMove::DWide2
            | NotationMove::R2
            | NotationMove::RWide2
            | NotationMove::L2
            | NotationMove::LWide2
            | NotationMove::F2
            | NotationMove::FWide2
            | NotationMove::B2
            | NotationMove::BWide2
            | NotationMove::M2
            | NotationMove::S2
            | NotationMove::E2) => m.clone(),
        }
    }
}

/// Sequence（NotationMoveの列）
pub type Sequence = Vec<NotationMove>;

/// Sequenceをパース
pub fn parse_sequence(s: &str) -> Result<Sequence, String> {
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }

    s.trim()
        .split_whitespace()
        .map(|token| NotationMove::from_str(token))
        .collect()
}

/// Sequenceを文字列に変換
pub fn sequence_to_string(seq: &Sequence) -> String {
    seq.iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Sequenceの逆操作を返す
pub fn reversed_sequence(seq: &Sequence) -> Sequence {
    seq.iter().rev().map(|m| m.reversed()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_from_str() {
        assert_eq!(NotationMove::from_str("U").unwrap(), NotationMove::U);
        assert_eq!(NotationMove::from_str("U2").unwrap(), NotationMove::U2);
        assert_eq!(NotationMove::from_str("U'").unwrap(), NotationMove::UPrime);
        assert!(NotationMove::from_str("X").is_err());
    }

    #[test]
    fn test_move_reversed() {
        assert_eq!(NotationMove::U.reversed(), NotationMove::UPrime);
        assert_eq!(NotationMove::UPrime.reversed(), NotationMove::U);
        assert_eq!(NotationMove::U2.reversed(), NotationMove::U2);
    }

    #[test]
    fn test_move_doubled() {
        assert_eq!(NotationMove::U.doubled(), NotationMove::U2);
        assert_eq!(NotationMove::UPrime.doubled(), NotationMove::U2);
        assert_eq!(NotationMove::U2.doubled(), NotationMove::U2);
    }

    #[test]
    fn test_parse_sequence() {
        let seq = parse_sequence("U R D R'").unwrap();
        assert_eq!(
            seq,
            vec![
                NotationMove::U,
                NotationMove::R,
                NotationMove::D,
                NotationMove::RPrime
            ]
        );
    }

    #[test]
    fn test_reversed_sequence() {
        let seq = vec![
            NotationMove::U,
            NotationMove::R,
            NotationMove::D,
            NotationMove::RPrime,
        ];
        let reversed = reversed_sequence(&seq);
        assert_eq!(
            reversed,
            vec![
                NotationMove::R,
                NotationMove::DPrime,
                NotationMove::RPrime,
                NotationMove::UPrime
            ]
        );
    }

    #[test]
    fn test_sequence_to_string() {
        let seq = vec![
            NotationMove::U,
            NotationMove::R,
            NotationMove::D,
            NotationMove::RPrime,
        ];
        assert_eq!(sequence_to_string(&seq), "U R D R'");
    }
}
