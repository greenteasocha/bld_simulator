use rubiks_cube_simulator::cube::{operations, State};
use rubiks_cube_simulator::inspection::{EdgeInspection, EdgeOperation};

fn main() {
    println!("=== Edge BLD Workflow Example ===\n");

    // 1. スクランブル済みのキューブ状態を作成
    let scramble = "R U R' U' R' F R2 U' R' U' R U R' F'";
    let scrambled_state = operations::apply_moves_notation(&State::solved(), scramble);

    println!("Scrambled state:");
    println!("  ep: {:?}", scrambled_state.ep);
    println!("  eo: {:?}", scrambled_state.eo);
    println!();

    // 2. BLD用の操作列を計算
    let operations = EdgeInspection::solve_edge_permutation_with_orientation(&scrambled_state);

    println!("Operations needed to solve edge pieces:");
    println!("{}", EdgeInspection::format_operations(&operations));
    println!();

    // 3. 操作列を適用して完成状態に到達することを確認
    let mut current_state = scrambled_state.clone();
    println!("Step-by-step verification:");
    
    for (i, op) in operations.iter().enumerate() {
        println!("\nStep {}: {}", i + 1, op);
        println!("  Before: ep: {:?}, eo: {:?}", current_state.ep, current_state.eo);
        
        current_state = op.apply(&current_state);
        
        println!("  After:  ep: {:?}, eo: {:?}", current_state.ep, current_state.eo);
    }

    println!("\n=== Verification ===");
    println!("Final state:");
    println!("  ep: {:?}", current_state.ep);
    println!("  eo: {:?}", current_state.eo);
    
    if current_state.ep == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] 
        && current_state.eo == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
        println!("✓ Edge pieces are solved!");
    } else {
        println!("✗ Edge pieces are NOT solved");
    }

    // 4. 統計情報
    let swap_count = operations.iter().filter(|op| matches!(op, EdgeOperation::Swap(_))).count();
    let flip_count = operations.iter().filter(|op| matches!(op, EdgeOperation::Flip(_))).count();
    
    println!("\n=== Statistics ===");
    println!("Total operations: {}", operations.len());
    println!("  Swap operations: {}", swap_count);
    println!("  Flip operations: {}", flip_count);
}
