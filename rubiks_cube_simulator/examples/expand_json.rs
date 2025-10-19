use rubiks_cube_simulator::parser::{parse_and_expand, sequence_to_string};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSONファイルを読み込む
    let input_path = "resources/ufr.json";
    let output_path = "resources/ufr_expanded.json";

    println!("Reading: {}", input_path);
    let content = fs::read_to_string(input_path)?;
    let data: HashMap<String, HashMap<String, String>> = serde_json::from_str(&content)?;

    // 展開結果を格納
    let mut expanded_data: HashMap<String, HashMap<String, String>> = HashMap::new();

    // 各エントリを展開
    for (outer_key, inner_map) in &data {
        let mut expanded_inner: HashMap<String, String> = HashMap::new();

        for (inner_key, notation) in inner_map {
            print!("Expanding {}.{}: {} -> ", outer_key, inner_key, notation);

            match parse_and_expand(notation) {
                Ok(sequence) => {
                    let expanded = sequence_to_string(&sequence);
                    println!("{}", expanded);
                    expanded_inner.insert(inner_key.clone(), expanded);
                }
                Err(e) => {
                    eprintln!("Error expanding {}.{}: {}", outer_key, inner_key, e);
                    // エラーの場合は元の文字列をそのまま使用
                    expanded_inner.insert(inner_key.clone(), notation.clone());
                }
            }
        }

        expanded_data.insert(outer_key.clone(), expanded_inner);
    }

    // JSONに変換して出力
    let json_output = serde_json::to_string_pretty(&expanded_data)?;
    fs::write(output_path, json_output)?;

    println!("\nExpanded JSON saved to: {}", output_path);

    Ok(())
}
