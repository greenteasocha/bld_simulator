//! 統合テスト: operations_to_turns の機能をテストする

use rubiks_cube_simulator::inspection::{
    CornerOperation, CornerSwapOperation, CornerTwistOperation, OperationsToTurns,
};
use rubiks_cube_simulator::parser::move_parser::sequence_to_string;

#[test]
fn test_operations_to_turns_with_real_data() {
    // JSONファイルを読み込む
    let ufr_expanded = include_str!("../resources/ufr_expanded.json");
    let ufr_parity = include_str!("../resources/ufr_parity.json");
    let ufr_twist = include_str!("../resources/ufr_twist.json");

    // OperationsToTurns を初期化
    let converter = OperationsToTurns::new(ufr_expanded, ufr_parity, ufr_twist)
        .expect("Failed to create converter");

    // テストケース1: 2つのSwap → 1つのシーケンスに変換される
    let swap1 = CornerSwapOperation::new(2, 5, 2); // UFR ↔ DBR (ori: 2) → BDR
    let swap2 = CornerSwapOperation::new(2, 6, 1); // UFR ↔ DFR (ori: 1) → RDF

    let operations = vec![
        CornerOperation::Swap(swap1),
        CornerOperation::Swap(swap2),
    ];

    let sequences = converter.convert(&operations).expect("Failed to convert");

    assert_eq!(sequences.len(), 1);
    assert_eq!(sequences[0].description, "BDR → RDF");
    assert_eq!(
        sequence_to_string(&sequences[0].moves),
        "D' R U R' D R U' R'"
    );
}

#[test]
fn test_single_swap_parity() {
    let ufr_expanded = include_str!("../resources/ufr_expanded.json");
    let ufr_parity = include_str!("../resources/ufr_parity.json");
    let ufr_twist = include_str!("../resources/ufr_twist.json");

    let converter = OperationsToTurns::new(ufr_expanded, ufr_parity, ufr_twist)
        .expect("Failed to create converter");

    // 1つのSwap → Parity
    let swap = CornerSwapOperation::new(2, 5, 2); // UFR ↔ DBR (ori: 2) → BDR
    let operations = vec![CornerOperation::Swap(swap)];

    let sequences = converter.convert(&operations).expect("Failed to convert");

    assert_eq!(sequences.len(), 1);
    assert_eq!(sequences[0].description, "Parity: BDR");
    // Parity手順は長いので文字列の一部だけチェック
    assert!(sequence_to_string(&sequences[0].moves).contains("U2"));
}

#[test]
fn test_single_twist() {
    let ufr_expanded = include_str!("../resources/ufr_expanded.json");
    let ufr_parity = include_str!("../resources/ufr_parity.json");
    let ufr_twist = include_str!("../resources/ufr_twist.json");

    let converter = OperationsToTurns::new(ufr_expanded, ufr_parity, ufr_twist)
        .expect("Failed to create converter");

    // Twist: UFL (counter-clockwise) → target=3, orientation=1 → "FUL"
    let twist = CornerTwistOperation::new(3, 1);
    let operations = vec![CornerOperation::Twist(twist)];

    let sequences = converter.convert(&operations).expect("Failed to convert");

    assert_eq!(sequences.len(), 1);
    assert_eq!(sequences[0].description, "Twist: FUL");
    assert_eq!(
        sequence_to_string(&sequences[0].moves),
        "R' D R D' R' D R U' R' D' R D R' D' R U"
    );
}

#[test]
fn test_mixed_operations() {
    let ufr_expanded = include_str!("../resources/ufr_expanded.json");
    let ufr_parity = include_str!("../resources/ufr_parity.json");
    let ufr_twist = include_str!("../resources/ufr_twist.json");

    let converter = OperationsToTurns::new(ufr_expanded, ufr_parity, ufr_twist)
        .expect("Failed to create converter");

    // 混合: 2つのSwap + 1つのSwap + 1つのTwist
    let swap1 = CornerSwapOperation::new(2, 5, 2); // BDR
    let swap2 = CornerSwapOperation::new(2, 6, 1); // RDF
    let swap3 = CornerSwapOperation::new(2, 0, 1); // BUL (parity)
    let twist = CornerTwistOperation::new(3, 1); // FUL

    let operations = vec![
        CornerOperation::Swap(swap1),
        CornerOperation::Swap(swap2),
        CornerOperation::Swap(swap3),
        CornerOperation::Twist(twist),
    ];

    let sequences = converter.convert(&operations).expect("Failed to convert");

    // 2つのSwap → 1シーケンス、1つのSwap → 1シーケンス、1つのTwist → 1シーケンス
    assert_eq!(sequences.len(), 3);
    assert_eq!(sequences[0].description, "BDR → RDF");
    assert_eq!(sequences[1].description, "Parity: BUL");
    assert_eq!(sequences[2].description, "Twist: FUL");
}
