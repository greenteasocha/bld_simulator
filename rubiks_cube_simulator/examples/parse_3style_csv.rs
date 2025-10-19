use rubiks_cube_simulator::parse_3style_csv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コマンドライン引数からCSVファイルのパスを取得
    let args: Vec<String> = env::args().collect();
    let csv_path = if args.len() > 1 {
        &args[1]
    } else {
        "resources/original.csv"
    };

    println!("Reading CSV file: {}", csv_path);

    // CSVをパース
    let data = parse_3style_csv(csv_path)?;

    // JSONとして出力
    let json = serde_json::to_string_pretty(&data)?;
    println!("{}", json);

    Ok(())
}
