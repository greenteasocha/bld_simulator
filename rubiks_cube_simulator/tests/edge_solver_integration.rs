use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::explorer::{NearbyEdgeOperationSearch, WrongEdgeOperationDetector};
use rubiks_cube_simulator::inspection::{EdgeInspection, EdgeOperation};

#[test]
fn test_edge_solver_integration() {
    // 複雑なケース
    let state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 6, 2, 5, 4, 3, 7, 9, 10, 11, 8],
        [1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0],
    );

    println!("\n=== Edge Solver Integration Test ===");
    println!("Initial state:");
    println!("  ep: {:?}", state.ep);
    println!("  eo: {:?}", state.eo);

    // 解法を計算
    let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

    println!("\nOperations:");
    println!("{}", EdgeInspection::format_operations(&operations));

    // 操作列を適用
    let mut current_state = state.clone();
    for op in &operations {
        current_state = op.apply(&current_state);
    }

    println!("\nFinal state:");
    println!("  ep: {:?}", current_state.ep);
    println!("  eo: {:?}", current_state.eo);

    // 検証
    assert_eq!(current_state.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(current_state.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    println!("\n✓ Edge pieces are solved!");
}

#[test]
fn test_edge_nearby_search_integration() {
    let state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    );

    println!("\n=== Edge Nearby Search Integration Test ===");

    let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);
    let searcher = NearbyEdgeOperationSearch::new(operations.clone());
    let variants = searcher.explore_variants(&state);

    println!("Generated {} variants", variants.len());

    let swap_count = operations
        .iter()
        .filter(|op| matches!(op, EdgeOperation::Swap(_)))
        .count();

    // 各Swap操作に対して24個の代替操作 (12 targets × 2 orientations)
    assert_eq!(variants.len(), swap_count * 24);

    println!("✓ Variant generation successful!");
}

#[test]
fn test_edge_wrong_operation_detector_integration() {
    let initial_state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );

    println!("\n=== Edge Wrong Operation Detector Integration Test ===");

    let detector = WrongEdgeOperationDetector::new(initial_state.clone());

    // 誤った状態を作成（意図的に間違った操作を適用した結果）
    let wrong_state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );

    let detected = detector.detect_wrong_operation(&wrong_state);

    println!("Detected {} possible wrong operations", detected.len());
    println!("\n{}", detector.format_detection_result(&wrong_state));

    println!("✓ Wrong operation detection successful!");
}

#[test]
fn test_edge_swap_orientation_logic() {
    // eoの加算ロジックをテスト
    use rubiks_cube_simulator::inspection::EdgeSwapOperation;

    let initial_state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    );

    println!("\n=== Edge Swap Orientation Logic Test ===");
    println!(
        "Initial: ep[8]={}, eo[8]={}",
        initial_state.ep[8], initial_state.eo[8]
    );
    println!(
        "Initial: ep[9]={}, eo[9]={}",
        initial_state.ep[9], initial_state.eo[9]
    );

    // orientation=1でswap
    let swap_op = EdgeSwapOperation::new(8, 9, 1);
    let result_state = swap_op.apply(&initial_state);

    println!("\nAfter swap with orientation=1:");
    println!(
        "Result: ep[8]={}, eo[8]={}",
        result_state.ep[8], result_state.eo[8]
    );
    println!(
        "Result: ep[9]={}, eo[9]={}",
        result_state.ep[9], result_state.eo[9]
    );

    // epは交換される: 9, 8
    assert_eq!(result_state.ep[8], 8);
    assert_eq!(result_state.ep[9], 9);

    // eoは両方にorientationが加算される
    // eo[8] = (eo[9] + orientation) % 2 = (0 + 1) % 2 = 1
    // eo[9] = (eo[8] + orientation) % 2 = (1 + 1) % 2 = 0
    assert_eq!(result_state.eo[8], 1);
    assert_eq!(result_state.eo[9], 0);

    println!("\n✓ Edge swap orientation logic verified!");
}

#[test]
fn test_edge_swap_orientation_edge_cases() {
    use rubiks_cube_simulator::inspection::EdgeSwapOperation;

    println!("\n=== Edge Swap Orientation Edge Cases ===");

    // ケース1: orientation=0のswap（向きは変わらない）
    let state1 = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 0, 9, 10, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    );

    let swap_op1 = EdgeSwapOperation::new(8, 0, 0);
    let result1 = swap_op1.apply(&state1);

    println!("\nCase 1: orientation=0");
    println!(
        "Before: ep[8]={}, eo[8]={}, ep[0]={}, eo[0]={}",
        state1.ep[8], state1.eo[8], state1.ep[0], state1.eo[0]
    );
    println!(
        "After:  ep[8]={}, eo[8]={}, ep[0]={}, eo[0]={}",
        result1.ep[8], result1.eo[8], result1.ep[0], result1.eo[0]
    );

    // ep交換: [8]←0, [0]←0 → [8]=0, [0]=0
    assert_eq!(result1.ep[8], 0);
    assert_eq!(result1.ep[0], 0);
    // eo: [8] = (eo[0] + 0) % 2 = 0, [0] = (eo[8] + 0) % 2 = 1
    assert_eq!(result1.eo[8], 0);
    assert_eq!(result1.eo[0], 1);

    // ケース2: 両方eo=1の状態でorientation=1
    let state2 = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 9, 8, 10, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
    );

    let swap_op2 = EdgeSwapOperation::new(8, 9, 1);
    let result2 = swap_op2.apply(&state2);

    println!("\nCase 2: both eo=1, orientation=1");
    println!(
        "Before: ep[8]={}, eo[8]={}, ep[9]={}, eo[9]={}",
        state2.ep[8], state2.eo[8], state2.ep[9], state2.eo[9]
    );
    println!(
        "After:  ep[8]={}, eo[8]={}, ep[9]={}, eo[9]={}",
        result2.ep[8], result2.eo[8], result2.ep[9], result2.eo[9]
    );

    assert_eq!(result2.ep[8], 8);
    assert_eq!(result2.ep[9], 9);
    // eo: [8] = (1 + 1) % 2 = 0, [9] = (1 + 1) % 2 = 0
    assert_eq!(result2.eo[8], 0);
    assert_eq!(result2.eo[9], 0);

    println!("\n✓ Edge cases verified!");
}

#[test]
fn test_edge_flip_operation() {
    use rubiks_cube_simulator::inspection::EdgeFlipOperation;

    println!("\n=== Edge Flip Operation Test ===");

    let state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        [0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    );

    println!("Initial eo: {:?}", state.eo);

    // Flip操作を適用
    let flip_op1 = EdgeFlipOperation::new(1);
    let state1 = flip_op1.apply(&state);
    println!("After flipping edge[1]: eo: {:?}", state1.eo);
    assert_eq!(state1.eo[1], 0); // 1 → 0

    let flip_op2 = EdgeFlipOperation::new(0);
    let state2 = flip_op2.apply(&state);
    println!("After flipping edge[0]: eo: {:?}", state2.eo);
    assert_eq!(state2.eo[0], 1); // 0 → 1

    println!("\n✓ Flip operation verified!");
}
