use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::workflow::CombinedNearbySearchWorkflow;
use std::fs;

/// Combined Nearby Search Workflow を使用した簡潔な例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSON リソースファイルを読み込み
    let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")?;
    let ufr_parity = fs::read_to_string("resources/ufr_parity.json")?;
    let ufr_twist = fs::read_to_string("resources/ufr_twist.json")?;
    let uf_expanded = fs::read_to_string("resources/uf_expanded.json")?;
    let uf_flip = fs::read_to_string("resources/uf_flip.json")?;

    // Combined Nearby Search ワークフローを作成
    let workflow = match CombinedNearbySearchWorkflow::from_json(
        &ufr_expanded,
        &ufr_parity,
        &ufr_twist,
        &uf_expanded,
        &uf_flip,
    ) {
        Ok(workflow) => workflow,
        Err(e) => {
            eprintln!("Warning: Failed to create workflow: {}", e);
            eprintln!("This may be due to complex move notation in JSON files that the current parser doesn't support.");
            return Ok(());
        }
    };

    println!("=== Combined Nearby Search Workflow Example ===\n");

    // スクランブル状態
    let scrambled_state = State::new(
        [0, 1, 7, 3, 4, 5, 2, 6],               // cp
        [0, 0, 1, 0, 0, 0, 2, 0],               // co
        [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11], // ep
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // eo
    );

    // 目標状態
    let target_state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],               // cp - solved corners
        [0, 0, 0, 0, 0, 0, 0, 0],               // co - solved orientation
        [0, 1, 2, 3, 4, 5, 11, 6, 8, 9, 10, 7], // ep - specific edge permutation
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // eo - solved orientation
    );

    // 探索を実行
    match workflow.search(&scrambled_state, &target_state) {
        Ok(result) => {
            // 詳細な結果を表示（最大5件ずつ）
            println!("{}", result.display_detailed(5));
        }
        Err(e) => {
            eprintln!("Failed to search: {}", e);
        }
    }

    println!("=== Combined Nearby Search completed ===");
    Ok(())
}
