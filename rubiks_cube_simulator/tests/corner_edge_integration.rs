use rubiks_cube_simulator::cube::{RubiksCube, State};
use rubiks_cube_simulator::inspection::{CornerInspection, EdgeInspection};

#[test]
fn test_corner_and_edge_solver_together() {
    println!("\n=== Corner and Edge Solver Together ===");

    // スクランブルを適用
    let scramble = "R U R' U' R' F R2 U' R' U' R U R' F'";
    let cube = RubiksCube::new();
    let scrambled_state = cube.scramble_to_state(scramble);

    println!("Scramble: {}", scramble);
    println!("\nScrambled state:");
    println!("  cp: {:?}", scrambled_state.cp);
    println!("  co: {:?}", scrambled_state.co);
    println!("  ep: {:?}", scrambled_state.ep);
    println!("  eo: {:?}", scrambled_state.eo);

    // コーナーの解法
    let corner_ops = CornerInspection::solve_corner_permutation_with_orientation(&scrambled_state);
    println!("\n--- Corner Operations ---");
    println!("{}", CornerInspection::format_operations(&corner_ops));

    // エッジの解法
    let edge_ops = EdgeInspection::solve_edge_permutation_with_orientation(&scrambled_state);
    println!("\n--- Edge Operations ---");
    println!("{}", EdgeInspection::format_operations(&edge_ops));

    // コーナー操作を適用
    let mut state_after_corners = scrambled_state.clone();
    for op in &corner_ops {
        state_after_corners = op.apply(&state_after_corners);
    }

    println!("\nAfter corner operations:");
    println!("  Corner solved: cp={:?}, co={:?}", 
        state_after_corners.cp == [0, 1, 2, 3, 4, 5, 6, 7],
        state_after_corners.co == [0, 0, 0, 0, 0, 0, 0, 0]);

    // エッジ操作を適用
    let mut final_state = state_after_corners.clone();
    for op in &edge_ops {
        final_state = op.apply(&final_state);
    }

    println!("\nAfter edge operations:");
    println!("  Edge solved: ep={:?}, eo={:?}", 
        final_state.ep == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        final_state.eo == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    // 完全に解けたか確認
    assert!(final_state.is_solved(), "Cube should be completely solved");

    println!("\n✓ Complete BLD solve successful!");
}

#[test]
fn test_complex_scramble() {
    println!("\n=== Complex Scramble Test ===");

    // より複雑なスクランブル
    let scramble = "F R U' R' U' R U R' F' R U R' U' R' F R F'";
    let cube = RubiksCube::new();
    let scrambled_state = cube.scramble_to_state(scramble);

    println!("Scramble: {}", scramble);

    // コーナーとエッジの解法を計算
    let corner_ops = CornerInspection::solve_corner_permutation_with_orientation(&scrambled_state);
    let edge_ops = EdgeInspection::solve_edge_permutation_with_orientation(&scrambled_state);

    println!("\nCorner operations: {}", corner_ops.len());
    println!("Edge operations: {}", edge_ops.len());

    // 完全に解けることを確認
    let mut state = scrambled_state.clone();
    for op in &corner_ops {
        state = op.apply(&state);
    }
    for op in &edge_ops {
        state = op.apply(&state);
    }

    assert!(state.is_solved(), "Complex scramble should be solvable");

    println!("✓ Complex scramble solved!");
}

#[test]
fn test_corner_only_scramble() {
    println!("\n=== Corner Only Scramble Test ===");

    // コーナーのみが崩れた状態を作成
    let state = State::new(
        [1, 0, 2, 3, 4, 5, 6, 7],  // コーナー置換のみ
        [2, 1, 0, 0, 0, 0, 0, 0],  // コーナー向き
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],  // エッジ完成
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // エッジ向き完成
    );

    let corner_ops = CornerInspection::solve_corner_permutation_with_orientation(&state);
    let edge_ops = EdgeInspection::solve_edge_permutation_with_orientation(&state);

    println!("Corner operations: {}", corner_ops.len());
    println!("Edge operations: {}", edge_ops.len());

    // エッジは完成しているので操作なし
    assert_eq!(edge_ops.len(), 0);

    // コーナー操作を適用
    let mut solved_state = state.clone();
    for op in &corner_ops {
        solved_state = op.apply(&solved_state);
    }

    assert!(solved_state.is_solved());

    println!("✓ Corner-only scramble solved!");
}

#[test]
fn test_edge_only_scramble() {
    println!("\n=== Edge Only Scramble Test ===");

    // エッジのみが崩れた状態を作成
    let state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],              // コーナー完成
        [0, 0, 0, 0, 0, 0, 0, 0],              // コーナー向き完成
        [1, 0, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11], // エッジ置換
        [1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],  // エッジ向き
    );

    let corner_ops = CornerInspection::solve_corner_permutation_with_orientation(&state);
    let edge_ops = EdgeInspection::solve_edge_permutation_with_orientation(&state);

    println!("Corner operations: {}", corner_ops.len());
    println!("Edge operations: {}", edge_ops.len());

    // コーナーは完成しているので操作なし
    assert_eq!(corner_ops.len(), 0);

    // エッジ操作を適用
    let mut solved_state = state.clone();
    for op in &edge_ops {
        solved_state = op.apply(&solved_state);
    }

    assert!(solved_state.is_solved());

    println!("✓ Edge-only scramble solved!");
}
