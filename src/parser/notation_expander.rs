use super::move_parser::{parse_sequence, reversed_sequence, NotationMove, Sequence};

/// 記法の種類
#[derive(Debug, Clone, PartialEq)]
pub enum Notation {
    /// カンマ記法: seq_a, seq_b
    Comma { seq_a: Sequence, seq_b: Sequence },
    /// スラッシュ記法: move/seq
    Slash { mov: NotationMove, seq: Sequence },
    /// コロン記法: seq_a: seq_b
    Colon { seq_a: Sequence, seq_b: Sequence },
    /// プレーンなシーケンス（記法なし）
    Plain(Sequence),
}

impl Notation {
    /// 記法を展開してシーケンスに変換
    pub fn expand(&self) -> Sequence {
        match self {
            // 規則1: seq_a, seq_b → seq_a seq_b reversed(seq_a) reversed(seq_b)
            Notation::Comma { seq_a, seq_b } => {
                let mut result = seq_a.clone();
                result.extend(seq_b.clone());
                result.extend(reversed_sequence(seq_a));
                result.extend(reversed_sequence(seq_b));
                result
            }

            // 規則2: m/seq → m seq doubled(m) reversed(seq) m
            Notation::Slash { mov, seq } => {
                let mut result = vec![mov.clone()];
                result.extend(seq.clone());
                result.push(mov.doubled());
                result.extend(reversed_sequence(seq));
                result.push(mov.clone());
                result
            }

            // 規則3: seq_a: seq_b → seq_a seq_b reversed(seq_a)
            Notation::Colon { seq_a, seq_b } => {
                let mut result = seq_a.clone();
                result.extend(seq_b.clone());
                result.extend(reversed_sequence(seq_a));
                result
            }

            // プレーンなシーケンスはそのまま返す
            Notation::Plain(seq) => seq.clone(),
        }
    }
}

/// 文字列から記法をパースする
///
/// パースの優先順位:
/// 1. スラッシュ記法 (move/seq)
/// 2. カンマ記法 (seq, seq)
/// 3. コロン記法 (seq: seq)
/// 4. プレーンなシーケンス
pub fn parse_notation(input: &str) -> Result<Notation, String> {
    let input = input.trim();

    // コロン記法をチェック
    if let Some(colon_pos) = find_top_level_delimiter(input, ':') {
        let left = &input[..colon_pos].trim();
        let right = &input[colon_pos + 1..].trim();

        // 左側が空の場合はエラー
        if left.is_empty() {
            return Err("Colon notation requires sequence before ':'".to_string());
        }

        // 左側をパースして展開
        let seq_a = parse_and_expand(left)?;
        let seq_b = parse_and_expand(right)?;

        return Ok(Notation::Colon { seq_a, seq_b });
    }

    // スラッシュ記法をチェック
    if let Some(slash_pos) = find_top_level_delimiter(input, '/') {
        let left = &input[..slash_pos].trim();
        let right = &input[slash_pos + 1..].trim();

        // 左側が単一のNotationMoveかチェック
        if !left.contains(char::is_whitespace) {
            if let Ok(mov) = NotationMove::from_str(left) {
                let seq = parse_and_expand(right)?;
                return Ok(Notation::Slash { mov, seq });
            }
        }
    }

    // カンマ記法をチェック
    if let Some(comma_pos) = find_top_level_delimiter(input, ',') {
        let left = &input[..comma_pos].trim();
        let right = &input[comma_pos + 1..].trim();

        let seq_a = parse_and_expand(left)?;
        let seq_b = parse_and_expand(right)?;

        return Ok(Notation::Comma { seq_a, seq_b });
    }

    // プレーンなシーケンスとしてパース
    let seq = parse_sequence(input)?;
    Ok(Notation::Plain(seq))
}

/// 文字列をパースして展開する（再帰的に処理）
pub fn parse_and_expand(input: &str) -> Result<Sequence, String> {
    let notation = parse_notation(input)?;
    Ok(notation.expand())
}

/// トップレベルのデリミタを見つける（ネストを考慮しない簡易版）
///
/// 注: 本来はコロン、スラッシュ、カンマのネストを正しく扱うべきだが、
/// 今回の仕様では左から右へのパースで十分と判断
fn find_top_level_delimiter(input: &str, delimiter: char) -> Option<usize> {
    input.find(delimiter)
}

#[cfg(test)]
mod tests {
    use super::super::move_parser::sequence_to_string;
    use super::*;

    #[test]
    fn test_comma_notation() {
        // U, R D R' → U R D R' U' R D' R'
        let notation = Notation::Comma {
            seq_a: vec![NotationMove::U],
            seq_b: vec![NotationMove::R, NotationMove::D, NotationMove::RPrime],
        };
        let expanded = notation.expand();
        assert_eq!(sequence_to_string(&expanded), "U R D R' U' R D' R'");
    }

    #[test]
    fn test_slash_notation() {
        // D/R' U' R → D R' U' R D2 R' U R D
        let notation = Notation::Slash {
            mov: NotationMove::D,
            seq: vec![NotationMove::RPrime, NotationMove::UPrime, NotationMove::R],
        };
        let expanded = notation.expand();
        assert_eq!(sequence_to_string(&expanded), "D R' U' R D2 R' U R D");
    }

    #[test]
    fn test_colon_notation() {
        // R' D': (some sequence)
        let notation = Notation::Colon {
            seq_a: vec![NotationMove::RPrime, NotationMove::DPrime],
            seq_b: vec![NotationMove::U],
        };
        let expanded = notation.expand();

        assert_eq!(sequence_to_string(&expanded), "R' D' U D R");
    }

    #[test]
    fn test_parse_comma_notation() {
        let result = parse_and_expand("U, R D R'").unwrap();
        assert_eq!(sequence_to_string(&result), "U R D R' U' R D' R'");
    }

    #[test]
    fn test_parse_slash_notation() {
        let result = parse_and_expand("D/R' U' R").unwrap();
        assert_eq!(sequence_to_string(&result), "D R' U' R D2 R' U R D");
    }

    #[test]
    fn test_nested_notation() {
        // R' D': U/R D R'
        // まず内側: U/R D R' → U R D R' U2 R D' R' U
        // 次に外側: R' D': (expanded) → R' D' (expanded) D R
        let result = parse_and_expand("R' D': U/R D R'").unwrap();
        assert_eq!(
            sequence_to_string(&result),
            "R' D' U R D R' U2 R D' R' U D R"
        );
    }

    #[test]
    fn test_complex_nested_notation() {
        // U R U': D, R' U' R
        // まず内側のカンマ: D, R' U' R → D R' U' R D' R' U R
        // 次に外側のコロン: U R U': (expanded) → U R U' (expanded) U R' U'
        let result = parse_and_expand("U R U': D, R' U' R").unwrap();
        assert_eq!(
            sequence_to_string(&result),
            "U R U' D R' U' R D' R' U R U R' U'"
        );
    }
}
