use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cube::{RubiksCube, State, SolutionSearcher, Move};
use crate::parser::{parse_sequence, sequence_to_string, NotationMove};
use crate::workflow::BldWorkflow;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize)]
pub struct ParsedScramble {
    pub success: bool,
    pub error: Option<String>,
    pub moves: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ScrambleResult {
    pub success: bool,
    pub error: Option<String>,
    pub state: Option<StateData>,
}

#[derive(Serialize, Deserialize)]
pub struct StateData {
    pub cp: [u8; 8],
    pub co: [u8; 8], 
    pub ep: [u8; 12],
    pub eo: [u8; 12],
}

#[derive(Serialize, Deserialize)]
pub struct SolverResult {
    pub success: bool,
    pub error: Option<String>,
    pub solutions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct BldSolutionResult {
    pub success: bool,
    pub error: Option<String>,
    pub solution: Option<BldSolutionData>,
}

#[derive(Serialize, Deserialize)]
pub struct BldSolutionData {
    pub corner_operations: Vec<String>,
    pub edge_operations: Vec<String>,
    pub all_operations: Vec<String>,
    pub move_sequences: Vec<MoveSequenceData>,
    pub formatted_solution: String,
}

#[derive(Serialize, Deserialize)]
pub struct MoveSequenceData {
    pub description: String,
    pub sequence: String,
}

#[wasm_bindgen]
pub fn parse_scramble(input: &str) -> JsValue {
    let result = match parse_sequence(input) {
        Ok(sequence) => ParsedScramble {
            success: true,
            error: None,
            moves: Some(sequence.iter().map(|m| m.to_string()).collect()),
        },
        Err(err) => ParsedScramble {
            success: false,
            error: Some(err),
            moves: None,
        },
    };
    
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn apply_scramble_to_state(scramble: &str) -> JsValue {
    let cube = RubiksCube::new();
    let mut state = State::solved();
    
    match parse_sequence(scramble) {
        Ok(sequence) => {
            // Apply each move to the state
            for notation_move in sequence {
                let move_str = notation_move.to_string();
                match cube.apply_move(&state, &move_str) {
                    Some(new_state) => state = new_state,
                    None => {
                        let error_result = ScrambleResult {
                            success: false,
                            error: Some(format!("Failed to apply move: {}", move_str)),
                            state: None,
                        };
                        return serde_wasm_bindgen::to_value(&error_result).unwrap();
                    }
                }
            }
            
            let success_result = ScrambleResult {
                success: true,
                error: None,
                state: Some(StateData {
                    cp: state.cp,
                    co: state.co,
                    ep: state.ep,
                    eo: state.eo,
                }),
            };
            serde_wasm_bindgen::to_value(&success_result).unwrap()
        },
        Err(err) => {
            let error_result = ScrambleResult {
                success: false,
                error: Some(err),
                state: None,
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn solve_from_state(cp: Vec<u8>, co: Vec<u8>, ep: Vec<u8>, eo: Vec<u8>) -> JsValue {
    // Validate array lengths
    if cp.len() != 8 || co.len() != 8 || ep.len() != 12 || eo.len() != 12 {
        let error_result = SolverResult {
            success: false,
            error: Some("Invalid state array lengths. Expected: cp(8), co(8), ep(12), eo(12)".to_string()),
            solutions: None,
        };
        return serde_wasm_bindgen::to_value(&error_result).unwrap();
    }

    // Convert vectors to arrays
    let cp_array: [u8; 8] = match cp.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = SolverResult {
                success: false,
                error: Some("Failed to convert cp to array".to_string()),
                solutions: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let co_array: [u8; 8] = match co.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = SolverResult {
                success: false,
                error: Some("Failed to convert co to array".to_string()),
                solutions: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let ep_array: [u8; 12] = match ep.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = SolverResult {
                success: false,
                error: Some("Failed to convert ep to array".to_string()),
                solutions: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let eo_array: [u8; 12] = match eo.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = SolverResult {
                success: false,
                error: Some("Failed to convert eo to array".to_string()),
                solutions: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    // Create state from arrays
    let state = State {
        cp: cp_array,
        co: co_array,
        ep: ep_array,
        eo: eo_array,
    };

    // Create solver and search for solutions
    let mut searcher = SolutionSearcher::with_bottom_layer_pattern(state);
    
    match searcher.search() {
        Some(solutions) => {
            let solution_strings: Vec<String> = solutions
                .iter()
                .map(|moves| SolutionSearcher::format_solution(moves))
                .collect();

            let success_result = SolverResult {
                success: true,
                error: None,
                solutions: Some(solution_strings),
            };
            serde_wasm_bindgen::to_value(&success_result).unwrap()
        },
        None => {
            let error_result = SolverResult {
                success: false,
                error: Some("No solution found within search depth limit".to_string()),
                solutions: None,
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn solve_bld(
    cp: Vec<u8>, co: Vec<u8>, ep: Vec<u8>, eo: Vec<u8>,
    ufr_expanded_json: &str,
    ufr_parity_json: &str,
    ufr_twist_json: &str, 
    uf_expanded_json: &str,
    uf_flip_json: &str
) -> JsValue {
    // Validate array lengths
    if cp.len() != 8 || co.len() != 8 || ep.len() != 12 || eo.len() != 12 {
        let error_result = BldSolutionResult {
            success: false,
            error: Some("Invalid state array lengths. Expected: cp(8), co(8), ep(12), eo(12)".to_string()),
            solution: None,
        };
        return serde_wasm_bindgen::to_value(&error_result).unwrap();
    }

    // Convert vectors to arrays
    let cp_array: [u8; 8] = match cp.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert cp to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let co_array: [u8; 8] = match co.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert co to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let ep_array: [u8; 12] = match ep.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert ep to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let eo_array: [u8; 12] = match eo.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert eo to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    // Create state from arrays
    let state = State {
        cp: cp_array,
        co: co_array,
        ep: ep_array,
        eo: eo_array,
    };

    // Create BldWorkflow
    let workflow = match BldWorkflow::new(
        ufr_expanded_json,
        ufr_parity_json,
        ufr_twist_json,
        uf_expanded_json,
        uf_flip_json
    ) {
        Ok(w) => w,
        Err(err) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some(format!("Failed to create BldWorkflow: {}", err)),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    // Solve using BldWorkflow
    match workflow.solve(&state) {
        Ok(solution) => {
            let corner_ops: Vec<String> = solution.corner_operations
                .iter()
                .map(|op| op.to_string())
                .collect();

            let edge_ops: Vec<String> = solution.edge_operations
                .iter()
                .map(|op| op.to_string())
                .collect();

            let all_ops: Vec<String> = solution.all_operations
                .operations()
                .iter()
                .map(|op| op.to_string())
                .collect();

            let move_seqs: Vec<MoveSequenceData> = solution.move_sequences
                .sequences()
                .iter()
                .map(|seq| MoveSequenceData {
                    description: seq.description.clone(),
                    sequence: seq.to_string(),
                })
                .collect();

            let formatted = BldWorkflow::format_solution(&solution);

            let success_result = BldSolutionResult {
                success: true,
                error: None,
                solution: Some(BldSolutionData {
                    corner_operations: corner_ops,
                    edge_operations: edge_ops,
                    all_operations: all_ops,
                    move_sequences: move_seqs,
                    formatted_solution: formatted,
                }),
            };
            serde_wasm_bindgen::to_value(&success_result).unwrap()
        },
        Err(err) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some(format!("Failed to solve: {}", err)),
                solution: None,
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn solve_bld_with_default_moveset(
    cp: Vec<u8>, co: Vec<u8>, ep: Vec<u8>, eo: Vec<u8>
) -> JsValue {
    // Validate array lengths
    if cp.len() != 8 || co.len() != 8 || ep.len() != 12 || eo.len() != 12 {
        let error_result = BldSolutionResult {
            success: false,
            error: Some("Invalid state array lengths. Expected: cp(8), co(8), ep(12), eo(12)".to_string()),
            solution: None,
        };
        return serde_wasm_bindgen::to_value(&error_result).unwrap();
    }

    // Convert vectors to arrays
    let cp_array: [u8; 8] = match cp.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert cp to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let co_array: [u8; 8] = match co.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert co to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let ep_array: [u8; 12] = match ep.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert ep to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    let eo_array: [u8; 12] = match eo.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some("Failed to convert eo to array".to_string()),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    // Create state from arrays
    let state = State {
        cp: cp_array,
        co: co_array,
        ep: ep_array,
        eo: eo_array,
    };

    // Create BldWorkflow with embedded movesets
    let workflow = match BldWorkflow::new(
        include_str!("../data/ufr_expanded.json"),
        include_str!("../data/ufr_parity.json"),
        include_str!("../data/ufr_twist.json"),
        include_str!("../data/uf_expanded.json"),
        include_str!("../data/uf_flip.json")
    ) {
        Ok(w) => w,
        Err(err) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some(format!("Failed to create BldWorkflow: {}", err)),
                solution: None,
            };
            return serde_wasm_bindgen::to_value(&error_result).unwrap();
        }
    };

    // Solve using BldWorkflow
    match workflow.solve(&state) {
        Ok(solution) => {
            let corner_ops: Vec<String> = solution.corner_operations
                .iter()
                .map(|op| op.to_string())
                .collect();

            let edge_ops: Vec<String> = solution.edge_operations
                .iter()
                .map(|op| op.to_string())
                .collect();

            let all_ops: Vec<String> = solution.all_operations
                .operations()
                .iter()
                .map(|op| op.to_string())
                .collect();

            let move_seqs: Vec<MoveSequenceData> = solution.move_sequences
                .sequences()
                .iter()
                .map(|seq| MoveSequenceData {
                    description: seq.description.clone(),
                    sequence: seq.to_string(),
                })
                .collect();

            let formatted = BldWorkflow::format_solution(&solution);

            let success_result = BldSolutionResult {
                success: true,
                error: None,
                solution: Some(BldSolutionData {
                    corner_operations: corner_ops,
                    edge_operations: edge_ops,
                    all_operations: all_ops,
                    move_sequences: move_seqs,
                    formatted_solution: formatted,
                }),
            };
            serde_wasm_bindgen::to_value(&success_result).unwrap()
        },
        Err(err) => {
            let error_result = BldSolutionResult {
                success: false,
                error: Some(format!("Failed to solve: {}", err)),
                solution: None,
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap()
        }
    }
}