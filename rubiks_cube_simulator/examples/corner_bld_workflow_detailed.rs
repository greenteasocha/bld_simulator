use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::inspection::{
    CornerInspection, CornerOperation, OperationsToTurns,
};
use rubiks_cube_simulator::parser::move_parser::sequence_to_string;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Corner BLD Workflow: Detailed Example ===\n");

    // ========================================
    // ステップ1: 初期状態の定義
    // ========================================
    // この状態は後で変更可能です。
    // 現在の例は以下のような状態を表しています：
    //   - コーナーの位置が複数のサイクルを含む
    //   - コーナーの向きも変更されている
    //
    // コーナーインデックスの対応:
    //   0: UBL, 1: UBR, 2: UFR (buffer), 3: UFL
    //   4: DBL, 5: DBR, 6: DFR, 7: DFL
    let state = State::new(
        [3, 1, 6, 2, 5, 7, 4, 0], // cp: UFR位置にUFLがある、など
        [2, 1, 0, 1, 0, 2, 1, 0], // co: 各コーナーの向き（0, 1, 2）
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], // ep: エッジは完成状態
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // eo: エッジは完成状態
    );

    print_state_info(&state);

    // ========================================
    // ステップ2: Corner Inspection（解析）
    // ========================================
    // CornerInspectionは、与えられた状態を完成状態に戻すための
    // 抽象的な操作列（Swap/Twist）を生成します
    println!("\n=== Step 1: Analyze and Generate Corner Operations ===");
    println!("Using CornerInspection::solve_corner_permutation_with_orientation()");
    
    let operations = CornerInspection::solve_corner_permutation_with_orientation(&state);
    
    println!("\nGenerated {} corner operations:", operations.len());
    println!("{}", CornerInspection::format_operations(&operations));

    // ========================================
    // ステップ2.5: 検証（操作列を適用）
    // ========================================
    println!("\n=== Verification: Simulate Operations ===");
    let mut verification_state = state.clone();
    
    for (i, op) in operations.iter().enumerate() {
        println!("\nApplying operation {}: {}", i + 1, op);
        println!("  Before: cp={:?}, co={:?}", 
                 verification_state.cp, verification_state.co);
        
        verification_state = op.apply(&verification_state);
        
        println!("  After:  cp={:?}, co={:?}", 
                 verification_state.cp, verification_state.co);
    }

    let is_solved = verification_state.is_solved();
    println!("\n✓ Final state is solved: {}", is_solved);
    assert!(is_solved, "Operations should solve the cube!");

    // ========================================
    // ステップ3: 3-Styleアルゴリズムデータベースの読み込み
    // ========================================
    println!("\n=== Step 2: Load 3-Style Algorithm Database ===");
    println!("Loading JSON files...");
    
    let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")
        .expect("Failed to read ufr_expanded.json");
    let ufr_parity = fs::read_to_string("resources/ufr_parity.json")
        .expect("Failed to read ufr_parity.json");
    let ufr_twist = fs::read_to_string("resources/ufr_twist.json")
        .expect("Failed to read ufr_twist.json");
    
    println!("  ✓ ufr_expanded.json (3-style algorithms)");
    println!("  ✓ ufr_parity.json (parity algorithms)");
    println!("  ✓ ufr_twist.json (twist algorithms)");

    // ========================================
    // ステップ4: OperationsToTurnsの初期化
    // ========================================
    println!("\n=== Step 3: Initialize Operations-to-Turns Converter ===");
    let converter = OperationsToTurns::new(&ufr_expanded, &ufr_parity, &ufr_twist)?;
    println!("✓ Converter initialized and ready");

    // ========================================
    // ステップ5: 操作列を実際の手順に変換
    // ========================================
    println!("\n=== Step 4: Convert Abstract Operations to Concrete Moves ===");
    println!("Converting {} operations to move sequences...", operations.len());
    
    let sequences = converter.convert(&operations)?;
    
    println!("✓ Generated {} move sequences", sequences.len());

    // ========================================
    // ステップ6: 手順の詳細表示
    // ========================================
    println!("\n=== Step 5: Display Move Sequences ===");
    print_sequences_detailed(&sequences);

    // ========================================
    // ステップ7: サマリーと実行用フォーマット
    // ========================================
    print_summary(&sequences);

    Ok(())
}

/// 状態の情報を見やすく表示
fn print_state_info(state: &State) {
    println!("Initial Cube State:");
    println!("┌─────────────────────────────────────────┐");
    println!("│ Corner Permutation (cp):                │");
    println!("│   {:?}          │", state.cp);
    println!("│                                         │");
    println!("│ Corner Orientation (co):                │");
    println!("│   {:?}          │", state.co);
    println!("└─────────────────────────────────────────┘");
    
    println!("\nCorner Index Mapping:");
    println!("  0:UBL  1:UBR  2:UFR(buffer)  3:UFL");
    println!("  4:DBL  5:DBR  6:DFR          7:DFL");
}

/// 手順を詳細に表示
fn print_sequences_detailed(sequences: &[rubiks_cube_simulator::inspection::MoveSequence]) {
    for (i, seq) in sequences.iter().enumerate() {
        println!("\n┌────────────────────────────────────────");
        println!("│ Sequence {}: {}", i + 1, seq.description);
        println!("├────────────────────────────────────────");
        println!("│ Moves: {}", sequence_to_string(&seq.moves));
        println!("│ Count: {} moves", seq.moves.len());
        println!("└────────────────────────────────────────");
    }
}

/// サマリー情報を表示
fn print_summary(sequences: &[rubiks_cube_simulator::inspection::MoveSequence]) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║              SUMMARY                      ║");
    println!("╠═══════════════════════════════════════════╣");
    
    let total_moves: usize = sequences.iter().map(|s| s.moves.len()).sum();
    
    println!("║ Total Sequences: {:>24} ║", sequences.len());
    println!("║ Total Moves:     {:>24} ║", total_moves);
    println!("╚═══════════════════════════════════════════╝");

    // すべての手順を1行で出力（キューブ操作ツールなどで実行する用）
    println!("\n=== Complete Algorithm (Ready to Execute) ===");
    let all_moves: Vec<String> = sequences
        .iter()
        .flat_map(|s| s.moves.iter().map(|m| m.to_string()))
        .collect();
    println!("{}", all_moves.join(" "));
    
    println!("\n=== Sequence Breakdown ===");
    for (i, seq) in sequences.iter().enumerate() {
        println!("{}. [{}] {}", 
                 i + 1, 
                 seq.moves.len(),
                 sequence_to_string(&seq.moves));
    }
}
