use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::workflow::{BldWorkflow, MixedNearbySearchWorkflow};
use std::fs;

/// Mixed Nearby Search を使用した BLD ワークフローの例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSON リソースファイルを読み込み
    let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")?;
    let ufr_parity = fs::read_to_string("resources/ufr_parity.json")?;
    let ufr_twist = fs::read_to_string("resources/ufr_twist.json")?;
    let uf_expanded = fs::read_to_string("resources/uf_expanded.json")?;
    let uf_flip = fs::read_to_string("resources/uf_flip.json")?;

    // BLD ワークフローを初期化（エラーハンドリング付き）
    let bld_workflow = match BldWorkflow::new(
        &ufr_expanded,
        &ufr_parity,
        &ufr_twist,
        &uf_expanded,
        &uf_flip,
    ) {
        Ok(workflow) => workflow,
        Err(e) => {
            eprintln!("Warning: Failed to create BLD workflow: {}", e);
            eprintln!("This may be due to complex move notation in JSON files that the current parser doesn't support.");
            return Ok(());
        }
    };

    // Mixed Nearby Search ワークフローを作成
    let mixed_workflow = MixedNearbySearchWorkflow::new(bld_workflow);

    println!("=== Mixed Nearby Search Workflow Example ===\n");

    // 1: スクランブルに対する正しい操作列の出力
    let scrambled_state = State::new(
        [0, 1, 7, 3, 4, 5, 2, 6],               // cp
        [0, 0, 1, 0, 0, 0, 2, 0],               // co
        [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11], // ep
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // eo
    );

    println!("Step 1: スクランブル状態の正しい操作列");
    println!("Initial state:");
    println!("  cp: {:?}", scrambled_state.cp);
    println!("  co: {:?}", scrambled_state.co);
    println!("  ep: {:?}", scrambled_state.ep);
    println!("  eo: {:?}", scrambled_state.eo);
    println!();

    match mixed_workflow.get_correct_solution(&scrambled_state) {
        Ok(solution) => {
            println!("Correct solution found:");
            let mixed_operations = mixed_workflow.solution_to_mixed_operations(&solution);

            println!("Mixed operations ({} steps):", mixed_operations.len());
            for (i, op) in mixed_operations.iter().enumerate() {
                println!("  Step {}: {}", i + 1, op);
            }
            println!();

            // 2. 正しい操作列からの分岐のうち特定の状態にたどり着くものの出力
            let target_state = State::new(
                [0, 1, 2, 3, 4, 5, 6, 7],               // cp - solved corners
                [0, 0, 0, 0, 0, 0, 0, 0],               // co - solved orientation
                [0, 1, 2, 3, 4, 5, 11, 6, 8, 9, 10, 7], // ep - specific edge permutation
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // eo - solved orientation
            );

            println!("Step 2: 特定の状態にたどり着く分岐の探索");
            println!("Target state:");
            println!("  cp: {:?}", target_state.cp);
            println!("  co: {:?}", target_state.co);
            println!("  ep: {:?}", target_state.ep);
            println!("  eo: {:?}", target_state.eo);
            println!();

            match mixed_workflow.find_variants_reaching_target(&scrambled_state, &target_state) {
                Ok(matching_variants) => {
                    if matching_variants.is_empty() {
                        println!("No variants found that reach the target state.");

                        // すべてのバリエーションを探索して表示
                        match mixed_workflow.explore_all_variants(&scrambled_state) {
                            Ok(all_variants) => {
                                println!(
                                    "\nAll available variants ({} total):",
                                    all_variants.len()
                                );
                                for (i, (modified_seq, final_state)) in
                                    all_variants.iter().take(10).enumerate()
                                {
                                    println!("Variant {}:", i + 1);
                                    println!("  Modifications: {}", modified_seq.get_description());
                                    println!("  Modifications: {:?}", modified_seq.get_sequence());
                                    println!("  Final state:");
                                    println!("    cp: {:?}", final_state.cp);
                                    println!("    co: {:?}", final_state.co);
                                    println!("    ep: {:?}", final_state.ep);
                                    println!("    eo: {:?}", final_state.eo);
                                    println!();
                                }

                                if all_variants.len() > 10 {
                                    println!("... and {} more variants", all_variants.len() - 10);
                                }
                            }
                            Err(e) => eprintln!("Failed to explore all variants: {}", e),
                        }
                    } else {
                        println!(
                            "Found {} variants that reach the target state:",
                            matching_variants.len()
                        );

                        for (i, (modified_seq, final_state)) in matching_variants.iter().enumerate()
                        {
                            println!("Match {}:", i + 1);
                            // println!("  Modifications: {}", modified_seq.get_description());
                            println!("  Operations:");
                            println!("{}", modified_seq);
                            println!("  Final state verification:");
                            println!("    cp: {:?}", final_state.cp);
                            println!("    co: {:?}", final_state.co);
                            println!("    ep: {:?}", final_state.ep);
                            println!("    eo: {:?}", final_state.eo);
                            println!();
                        }
                    }
                }
                Err(e) => eprintln!("Failed to find variants reaching target: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to get correct solution: {}", e),
    }

    println!("=== Mixed Nearby Search completed ===");
    Ok(())
}
