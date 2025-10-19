use rubiks_cube_simulator::inspection::{
    CornerInspection, CornerOperation, CornerSwapOperation, CornerTwistOperation,
    MoveSequence, OperationsToTurns,
};
use rubiks_cube_simulator::parser::move_parser::sequence_to_string;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSONファイルを読み込む
    let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")?;
    let ufr_parity = fs::read_to_string("resources/ufr_parity.json")?;
    let ufr_twist = fs::read_to_string("resources/ufr_twist.json")?;

    // OperationsToTurns を初期化
    let converter = OperationsToTurns::new(&ufr_expanded, &ufr_parity, &ufr_twist)?;

    // テストケース1: 2つのSwap
    println!("=== Test Case 1: Two Swaps ===");
    let swap1 = CornerSwapOperation::new(2, 5, 2); // UFR ↔ DBR (ori: 2) → BDR
    let swap2 = CornerSwapOperation::new(2, 6, 1); // UFR ↔ DFR (ori: 1) → RDF

    let operations1 = vec![
        CornerOperation::Swap(swap1),
        CornerOperation::Swap(swap2),
    ];

    let sequences1 = converter.convert(&operations1)?;
    for (i, seq) in sequences1.iter().enumerate() {
        println!("Sequence {}: {}", i + 1, seq.description);
        println!("  Moves: {}", sequence_to_string(&seq.moves));
    }

    // テストケース2: 1つのSwap (Parity)
    println!("\n=== Test Case 2: Single Swap (Parity) ===");
    let swap = CornerSwapOperation::new(2, 5, 2); // UFR ↔ DBR (ori: 2) → BDR
    let operations2 = vec![CornerOperation::Swap(swap)];

    let sequences2 = converter.convert(&operations2)?;
    for (i, seq) in sequences2.iter().enumerate() {
        println!("Sequence {}: {}", i + 1, seq.description);
        println!("  Moves: {}", sequence_to_string(&seq.moves));
    }

    // テストケース3: 1つのTwist
    println!("\n=== Test Case 3: Single Twist ===");
    let twist = CornerTwistOperation::new(3, 1); // UFL (counter-clockwise) → FUL
    let operations3 = vec![CornerOperation::Twist(twist)];

    let sequences3 = converter.convert(&operations3)?;
    for (i, seq) in sequences3.iter().enumerate() {
        println!("Sequence {}: {}", i + 1, seq.description);
        println!("  Moves: {}", sequence_to_string(&seq.moves));
    }

    // テストケース4: 混合
    println!("\n=== Test Case 4: Mixed Operations ===");
    let swap1 = CornerSwapOperation::new(2, 5, 2); // BDR
    let swap2 = CornerSwapOperation::new(2, 6, 1); // RDF
    let swap3 = CornerSwapOperation::new(2, 0, 1); // BUL
    let twist = CornerTwistOperation::new(3, 1); // FUL

    let operations4 = vec![
        CornerOperation::Swap(swap1),
        CornerOperation::Swap(swap2),
        CornerOperation::Swap(swap3),
        CornerOperation::Twist(twist),
    ];

    let sequences4 = converter.convert(&operations4)?;
    for (i, seq) in sequences4.iter().enumerate() {
        println!("Sequence {}: {}", i + 1, seq.description);
        println!("  Moves: {}", sequence_to_string(&seq.moves));
    }

    Ok(())
}
