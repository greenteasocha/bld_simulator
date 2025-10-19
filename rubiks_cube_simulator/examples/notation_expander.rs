use rubiks_cube_simulator::parser::{parse_and_expand, sequence_to_string};

fn main() {
    println!("=== 3style Notation Expander ===\n");

    // テストケース
    let test_cases = vec![
        ("U, R D R'", "カンマ記法"),
        ("D/R' U' R", "スラッシュ記法"),
        ("R' D': U/R D R'", "ネスト: コロン + スラッシュ"),
        ("U R U': D, R' U' R", "ネスト: コロン + カンマ"),
    ];

    for (input, description) in test_cases {
        println!("【{}】", description);
        println!("入力: {}", input);
        
        match parse_and_expand(input) {
            Ok(expanded) => {
                let output = sequence_to_string(&expanded);
                println!("出力: {}", output);
            }
            Err(e) => {
                println!("エラー: {}", e);
            }
        }
        println!();
    }

    // インタラクティブモード
    println!("=== インタラクティブモード ===");
    println!("3style記法を入力してください（終了は Ctrl+C）:\n");

    use std::io::{self, BufRead};
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = match line {
            Ok(s) => s,
            Err(_) => break,
        };

        if input.trim().is_empty() {
            continue;
        }

        match parse_and_expand(&input) {
            Ok(expanded) => {
                let output = sequence_to_string(&expanded);
                println!("→ {}", output);
            }
            Err(e) => {
                println!("エラー: {}", e);
            }
        }
        println!();
    }
}
