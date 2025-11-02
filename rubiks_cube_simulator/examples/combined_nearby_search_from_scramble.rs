use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::workflow::CombinedNearbySearchWorkflow;
use std::fs;

/// スクランブル文字列から探索する例
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
            return Ok(());
        }
    };

    println!("=== Combined Nearby Search from Scramble ===\n");

    // スクランブル文字列
    let scramble = "R U R' D R U' R' D'";
    println!("Scramble: {}\n", scramble);

    // 目標状態（コーナーは完成、エッジは特定の配置）
    let target_state = State::new(
        [0, 1, 2, 3, 4, 5, 6, 7],               // cp - solved corners
        [0, 0, 0, 0, 0, 0, 0, 0],               // co - solved orientation
        [0, 1, 2, 3, 4, 5, 11, 6, 8, 9, 10, 7], // ep - specific edge permutation
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // eo - solved orientation
    );

    // スクランブルから探索
    match workflow.search_from_scramble(scramble, &target_state) {
        Ok(result) => {
            println!("{}", result.display_detailed(3));
        }
        Err(e) => {
            eprintln!("Failed to search: {}", e);
        }
    }

    println!("=== Search completed ===");
    Ok(())
}
