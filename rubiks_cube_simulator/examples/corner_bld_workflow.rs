use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::inspection::{CornerInspection, CornerOperation, OperationsToTurns};
use rubiks_cube_simulator::parser::sequence_to_string;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Corner BLD Workflow Example ===\n");

    // ステップ1: 特定のStateを定義
    // ここでは適当なスクランブル状態を想定
    // 後で変更可能なように、わかりやすい例を使用
    let state = State::new(
        [1, 0, 6, 2, 5, 4, 3, 7],
        [2, 1, 0, 2, 2, 2, 2, 1],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );

    println!("Scramble: U' F U2 D2 F' U B U F U D2 F' B' D2 F' U2 D2 B");
    println!("Initial State:");
    println!("  cp (corner permutation): {:?}", state.cp);
    println!("  co (corner orientation): {:?}", state.co);
    println!();

    // ステップ2: CornerInspectionを使って解くための操作列を生成
    // println!("=== Step 1: Solve Corner Permutation and Orientation ===");
    let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);

    println!("Generated {} operations:", operations.len());
    for (i, op) in operations.iter().enumerate() {
        println!("  Operation {}: {}", i + 1, op);
    }
    println!();

    // ステップ2.5: 操作列を適用して完成状態になることを確認
    // println!("=== Verification: Apply operations to state ===");
    let mut current_state = state.clone();
    for op in &operations {
        current_state = op.apply(&current_state);
    }
    // println!("Final state after applying operations:");
    // println!("  cp: {:?}", current_state.cp);
    // println!("  co: {:?}", current_state.co);
    // println!("  Is solved: {}", current_state.is_solved());
    // println!();

    // ステップ3: JSONファイルを読み込む
    // println!("=== Step 2: Load 3-Style Algorithm Database ===");
    let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")?;
    let ufr_parity = fs::read_to_string("resources/ufr_parity.json")?;
    let ufr_twist = fs::read_to_string("resources/ufr_twist.json")?;
    // println!("Loaded algorithm database from JSON files");
    // println!();

    // ステップ4: OperationsToTurnsを初期化
    // println!("=== Step 3: Initialize Operations-to-Turns Converter ===");
    let converter = OperationsToTurns::new(&ufr_expanded, &ufr_parity, &ufr_twist)?;
    // println!("Converter initialized successfully");
    // println!();

    // ステップ5: 操作列を手順列に変換
    // println!("=== Step 4: Convert Operations to Move Sequences ===");
    let sequences = converter.convert(&operations)?;

    // println!("Generated {} move sequences:", sequences.len());
    // println!();

    // ステップ6: 結果を見やすく出力
    // println!("=== Step 5: Output Move Sequences ===");
    for (i, seq) in sequences.iter().enumerate() {
        println!("Sequence {}: {}", i + 1, seq.description);
        println!("  Moves: {}", sequence_to_string(&seq.moves));
        // println!("  Move count: {}", seq.moves.len());
        // println!();
    }

    // ステップ7: 全体のサマリー
    // println!("=== Summary ===");
    let total_moves: usize = sequences.iter().map(|s| s.moves.len()).sum();
    // println!("Total sequences: {}", sequences.len());
    // println!("Total moves: {}", total_moves);
    // println!();

    // ボーナス: すべての手順を1行で出力（実際に実行する用）
    println!("=== Complete Algorithm (for execution) ===");
    let all_moves: Vec<String> = sequences
        .iter()
        .flat_map(|s| s.moves.iter().map(|m| m.to_string()))
        .collect();
    println!("{}", all_moves.join(" "));

    Ok(())
}
