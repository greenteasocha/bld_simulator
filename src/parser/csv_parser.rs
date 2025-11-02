use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// CSVファイルをパースして二次元のHashMapに変換する
/// 
/// 1行目のセル（列ヘッダー）を第1階層のキー、
/// 1列目のセル（行ヘッダー）を第2階層のキーとして、
/// 各セルの値を格納する。
/// 
/// # 例
/// ```text
/// ,UBR,FDR
/// UBR,,"D: R D R', U"
/// FDR,"D: U, R D R'",
/// ```
/// 
/// これは以下のような構造に変換される:
/// ```json
/// {
///     "UBR": {
///         "FDR": "D: R D R', U"
///     },
///     "FDR": {
///         "UBR": "D: U, R D R'"
///     }
/// }
/// ```
pub fn parse_3style_csv<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // 1行目（ヘッダー行）を読み込む
    let header_line = lines
        .next()
        .ok_or("CSVファイルが空です")??;
    let column_headers: Vec<String> = parse_csv_line(&header_line);

    // 結果を格納するHashMap
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();

    // 2行目以降を読み込む
    for line in lines {
        let line = line?;
        let cells: Vec<String> = parse_csv_line(&line);

        if cells.is_empty() {
            continue;
        }

        // 1列目をキーとする
        let row_key = cells[0].clone();
        if row_key.is_empty() {
            continue;
        }

        // この行のデータを格納するHashMap
        let mut row_data: HashMap<String, String> = HashMap::new();

        // 2列目以降のセルを処理
        for (i, cell_value) in cells.iter().enumerate().skip(1) {
            if !cell_value.is_empty() {
                // 対応する列ヘッダーを取得
                if let Some(column_key) = column_headers.get(i) {
                    if !column_key.is_empty() {
                        row_data.insert(column_key.clone(), cell_value.clone());
                    }
                }
            }
        }

        // 行データが空でなければ結果に追加
        if !row_data.is_empty() {
            result.insert(row_key, row_data);
        }
    }

    Ok(result)
}

/// CSV行を解析してセルのベクトルに変換する
/// 
/// ダブルクォートで囲まれたセル内のカンマは区切り文字として扱わない
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut current_cell = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                // 次の文字も"の場合はエスケープされた"
                if chars.peek() == Some(&'"') {
                    current_cell.push('"');
                    chars.next(); // 次の"を消費
                } else {
                    // クォートの開始/終了を切り替え
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                // カンマで区切られた場合、現在のセルを追加
                cells.push(current_cell.trim().to_string());
                current_cell.clear();
            }
            _ => {
                current_cell.push(c);
            }
        }
    }

    // 最後のセルを追加
    cells.push(current_cell.trim().to_string());

    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_line() {
        let line = r#",UBR,"D: R D R', U",FDR"#;
        let cells = parse_csv_line(line);
        assert_eq!(cells, vec!["", "UBR", "D: R D R', U", "FDR"]);
    }

    #[test]
    fn test_parse_csv_line_with_escaped_quotes() {
        let line = r#"A,"B ""quoted"" text",C"#;
        let cells = parse_csv_line(line);
        assert_eq!(cells, vec!["A", r#"B "quoted" text"#, "C"]);
    }

    #[test]
    fn test_parse_3style_csv_structure() {
        // テスト用の小さなCSVファイルを想定
        let test_csv = r#",UBR,FDR
UBR,,"D: R D R', U"
FDR,"D: U, R D R'","#;

        // テスト用の一時ファイルを作成
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_3style.csv");
        std::fs::write(&temp_file, test_csv).unwrap();

        let result = parse_3style_csv(&temp_file).unwrap();

        // UBRキーの確認
        assert!(result.contains_key("UBR"));
        let ubr_data = &result["UBR"];
        assert_eq!(ubr_data.get("FDR"), Some(&"D: R D R', U".to_string()));

        // FDRキーの確認
        assert!(result.contains_key("FDR"));
        let fdr_data = &result["FDR"];
        assert_eq!(fdr_data.get("UBR"), Some(&"D: U, R D R'".to_string()));

        // クリーンアップ
        std::fs::remove_file(&temp_file).ok();
    }
}
