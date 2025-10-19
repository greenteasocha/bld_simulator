use rubiks_cube_simulator::cube::{operations, State};
use rubiks_cube_simulator::inspection::{EdgeInspection, EdgeOperation, EdgeSwapOperation, EdgeFlipOperation};

fn main() {
    println!("=== Edge BLD Workflow (Detailed) Example ===\n");

    // カスタムのスクランブル状態を作成
    let state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],              // cp (corners unchanged)
        [0, 0, 0, 0, 0, 0, 0, 0],              // co
        [3, 1, 2, 0, 5, 7, 4, 6, 9, 8, 10, 11], // ep (edges scrambled)
        [1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0],  // eo (some edges flipped)
    );

    println!("Initial edge state:");
    println!("  ep: {:?}", state.ep);
    println!("  eo: {:?}", state.eo);
    println!();

    // BLD用の操作列を計算
    let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);

    println!("=== Operation Sequence ===");
    println!("{}\n", EdgeInspection::format_operations(&operations));

    // 各操作の詳細を表示
    println!("=== Detailed Operation Analysis ===");
    let mut current_state = state.clone();
    
    for (i, op) in operations.iter().enumerate() {
        println!("\n--- Step {} ---", i + 1);
        
        match op {
            EdgeOperation::Swap(swap_op) => {
                println!("Operation: Swap");
                println!("  Target1 (buffer): {}", swap_op.target1);
                println!("  Target2: {}", swap_op.target2);
                println!("  Orientation: {}", swap_op.orientation);
                println!("  Display: {}", swap_op);
            }
            EdgeOperation::Flip(flip_op) => {
                println!("Operation: Flip");
                println!("  Target: {}", flip_op.target);
                println!("  Display: {}", flip_op);
            }
        }
        
        println!("  State before:");
        println!("    ep: {:?}", current_state.ep);
        println!("    eo: {:?}", current_state.eo);
        
        current_state = op.apply(&current_state);
        
        println!("  State after:");
        println!("    ep: {:?}", current_state.ep);
        println!("    eo: {:?}", current_state.eo);
    }

    // 最終検証
    println!("\n=== Final Verification ===");
    println!("Final edge state:");
    println!("  ep: {:?}", current_state.ep);
    println!("  eo: {:?}", current_state.eo);
    
    let is_edge_solved = current_state.ep == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        && current_state.eo == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    
    if is_edge_solved {
        println!("✓ Edge pieces are completely solved!");
    } else {
        println!("✗ Edge pieces are NOT solved");
    }

    // 統計情報
    let swap_count = operations
        .iter()
        .filter(|op| matches!(op, EdgeOperation::Swap(_)))
        .count();
    let flip_count = operations
        .iter()
        .filter(|op| matches!(op, EdgeOperation::Flip(_)))
        .count();

    println!("\n=== Operation Statistics ===");
    println!("Total operations: {}", operations.len());
    println!("  Swap operations: {}", swap_count);
    println!("  Flip operations: {}", flip_count);

    // 各タイプの操作を分類して表示
    println!("\n=== Operation Breakdown ===");
    println!("Swap operations:");
    for (i, op) in operations.iter().enumerate() {
        if let EdgeOperation::Swap(swap_op) = op {
            println!("  Step {}: {}", i + 1, swap_op);
        }
    }

    println!("\nFlip operations:");
    for (i, op) in operations.iter().enumerate() {
        if let EdgeOperation::Flip(flip_op) = op {
            println!("  Step {}: {}", i + 1, flip_op);
        }
    }
}
