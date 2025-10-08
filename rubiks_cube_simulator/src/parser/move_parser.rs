/// Moveの定義
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationMove {
    U,
    U2,
    UPrime,
    D,
    D2,
    DPrime,
    R,
    R2,
    RPrime,
    L,
    L2,
    LPrime,
    F,
    F2,
    FPrime,
    B,
    B2,
    BPrime,
}

impl NotationMove {
    /// 文字列からNotationMoveをパース
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "U" => Ok(NotationMove::U),
            "U2" => Ok(NotationMove::U2),
            "U'" => Ok(NotationMove::UPrime),
            "D" => Ok(NotationMove::D),
            "D2" => Ok(NotationMove::D2),
            "D'" => Ok(NotationMove::DPrime),
            "R" => Ok(NotationMove::R),
            "R2" => Ok(NotationMove::R2),
            "R'" => Ok(NotationMove::RPrime),
            "L" => Ok(NotationMove::L),
            "L2" => Ok(NotationMove::L2),
            "L'" => Ok(NotationMove::LPrime),
            "F" => Ok(NotationMove::F),
            "F2" => Ok(NotationMove::F2),
            "F'" => Ok(NotationMove::FPrime),
            "B" => Ok(NotationMove::B),
            "B2" => Ok(NotationMove::B2),
            "B'" => Ok(NotationMove::BPrime),
            _ => Err(format!("Unknown move: {}", s)),
        }
    }

    /// NotationMoveを文字列に変換
    pub fn to_string(&self) -> String {
        match self {
            NotationMove::U => "U".to_string(),
            NotationMove::U2 => "U2".to_string(),
            NotationMove::UPrime => "U'".to_string(),
            NotationMove::D => "D".to_string(),
            NotationMove::D2 => "D2".to_string(),
            NotationMove::DPrime => "D'".to_string(),
            NotationMove::R => "R".to_string(),
            NotationMove::R2 => "R2".to_string(),
            NotationMove::RPrime => "R'".to_string(),
            NotationMove::L => "L".to_string(),
            NotationMove::L2 => "L2".to_string(),
            NotationMove::LPrime => "L'".to_string(),
            NotationMove::F => "F".to_string(),
            NotationMove::F2 => "F2".to_string(),
            NotationMove::FPrime => "F'".to_string(),
            NotationMove::B => "B".to_string(),
            NotationMove::B2 => "B2".to_string(),
            NotationMove::BPrime => "B'".to_string(),
        }
    }

    /// NotationMoveの逆操作を返す
    pub fn reversed(&self) -> Self {
        match self {
            NotationMove::U => NotationMove::UPrime,
            NotationMove::UPrime => NotationMove::U,
            NotationMove::U2 => NotationMove::U2,
            NotationMove::D => NotationMove::DPrime,
            NotationMove::DPrime => NotationMove::D,
            NotationMove::D2 => NotationMove::D2,
            NotationMove::R => NotationMove::RPrime,
            NotationMove::RPrime => NotationMove::R,
            NotationMove::R2 => NotationMove::R2,
            NotationMove::L => NotationMove::LPrime,
            NotationMove::LPrime => NotationMove::L,
            NotationMove::L2 => NotationMove::L2,
            NotationMove::F => NotationMove::FPrime,
            NotationMove::FPrime => NotationMove::F,
            NotationMove::F2 => NotationMove::F2,
            NotationMove::B => NotationMove::BPrime,
            NotationMove::BPrime => NotationMove::B,
            NotationMove::B2 => NotationMove::B2,
        }
    }

    /// NotationMoveを2倍にする（180度回転にする）
    pub fn doubled(&self) -> Self {
        match self {
            NotationMove::U | NotationMove::UPrime => NotationMove::U2,
            NotationMove::D | NotationMove::DPrime => NotationMove::D2,
            NotationMove::R | NotationMove::RPrime => NotationMove::R2,
            NotationMove::L | NotationMove::LPrime => NotationMove::L2,
            NotationMove::F | NotationMove::FPrime => NotationMove::F2,
            NotationMove::B | NotationMove::BPrime => NotationMove::B2,
            // すでに2回転の場合はそのまま
            m @ (NotationMove::U2 | NotationMove::D2 | NotationMove::R2 | NotationMove::L2 | NotationMove::F2 | NotationMove::B2) => m.clone(),
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
        assert_eq!(seq, vec![NotationMove::U, NotationMove::R, NotationMove::D, NotationMove::RPrime]);
    }

    #[test]
    fn test_reversed_sequence() {
        let seq = vec![NotationMove::U, NotationMove::R, NotationMove::D, NotationMove::RPrime];
        let reversed = reversed_sequence(&seq);
        assert_eq!(
            reversed,
            vec![NotationMove::R, NotationMove::DPrime, NotationMove::RPrime, NotationMove::UPrime]
        );
    }

    #[test]
    fn test_sequence_to_string() {
        let seq = vec![NotationMove::U, NotationMove::R, NotationMove::D, NotationMove::RPrime];
        assert_eq!(sequence_to_string(&seq), "U R D R'");
    }
}
