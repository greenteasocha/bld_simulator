use super::state::State;
use crate::parser::move_parser::NotationMove;
use std::collections::HashMap;

pub struct RubiksCube {
    moves: HashMap<String, State>,
}

impl RubiksCube {
    pub fn new() -> Self {
        let mut moves = HashMap::new();

        // 6つの基本操作とその2層回し（90度時計回り）を定義
        moves.insert(
            "U".to_string(),
            State::new(
                [3, 0, 1, 2, 4, 5, 6, 7],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        moves.insert(
            "u".to_string(),
            State::new(
                [3, 0, 1, 2, 4, 5, 6, 7],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [3, 0, 1, 2, 7, 4, 5, 6, 8, 9, 10, 11],
                [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        moves.insert(
            "D".to_string(),
            State::new(
                [0, 1, 2, 3, 5, 6, 7, 4],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        moves.insert(
            "d".to_string(),
            State::new(
                [0, 1, 2, 3, 5, 6, 7, 4],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 3, 0, 4, 5, 6, 7, 9, 10, 11, 8],
                [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        moves.insert(
            "L".to_string(),
            State::new(
                [4, 1, 2, 0, 7, 5, 6, 3],
                [2, 0, 0, 1, 1, 0, 0, 2],
                [11, 1, 2, 7, 4, 5, 6, 0, 8, 9, 10, 3],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        moves.insert(
            "l".to_string(),
            State::new(
                [4, 1, 2, 0, 7, 5, 6, 3],
                [2, 0, 0, 1, 1, 0, 0, 2],
                [11, 1, 2, 7, 8, 5, 4, 0, 10, 9, 6, 3],
                [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            ),
        );

        moves.insert(
            "R".to_string(),
            State::new(
                [0, 2, 6, 3, 4, 1, 5, 7],
                [0, 1, 2, 0, 0, 2, 1, 0],
                [0, 5, 9, 3, 4, 2, 6, 7, 8, 1, 10, 11],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        moves.insert(
            "r".to_string(),
            State::new(
                [0, 2, 6, 3, 4, 1, 5, 7],
                [0, 1, 2, 0, 0, 2, 1, 0],
                [0, 5, 9, 3, 6, 2, 10, 7, 4, 1, 8, 11],
                [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            ),
        );

        moves.insert(
            "F".to_string(),
            State::new(
                [0, 1, 3, 7, 4, 5, 2, 6],
                [0, 0, 1, 2, 0, 0, 2, 1],
                [0, 1, 6, 10, 4, 5, 3, 7, 8, 9, 2, 11],
                [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0],
            ),
        );

        moves.insert(
            "f".to_string(),
            State::new(
                [0, 1, 3, 7, 4, 5, 2, 6],
                [0, 0, 1, 2, 0, 0, 2, 1],
                [0, 1, 6, 10, 4, 7, 3, 11, 8, 5, 2, 9],
                [0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1],
            ),
        );

        moves.insert(
            "B".to_string(),
            State::new(
                [1, 5, 2, 3, 0, 4, 6, 7],
                [1, 2, 0, 0, 2, 1, 0, 0],
                [4, 8, 2, 3, 1, 5, 6, 7, 0, 9, 10, 11],
                [1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            ),
        );

        moves.insert(
            "b".to_string(),
            State::new(
                [1, 5, 2, 3, 0, 4, 6, 7],
                [1, 2, 0, 0, 2, 1, 0, 0],
                [4, 8, 2, 3, 1, 9, 6, 5, 0, 11, 10, 7],
                [1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1],
            ),
        );

        moves.insert(
            "M".to_string(),
            State::new(
                [0, 1, 2, 3, 4, 5, 6, 7],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 2, 3, 8, 5, 4, 7, 10, 9, 6, 11],
                [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            ),
        );

        moves.insert(
            "S".to_string(),
            State::new(
                [0, 1, 2, 3, 4, 5, 6, 7],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 2, 9, 4, 7, 6, 3, 8, 5, 10, 11],
                [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0],
            ),
        );

        moves.insert(
            "E".to_string(),
            State::new(
                [0, 1, 2, 3, 4, 5, 6, 7],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
                [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );

        // 180度回転と反時計回り90度回転を生成
        let face_names = [
            "U", "u", "D", "d", "L", "l", "R", "r", "F", "f", "B", "b", "M", "S", "E",
        ];
        let mut cube = RubiksCube { moves };

        for face in &face_names {
            let base_move = cube.moves[*face].clone();

            // 180度回転 (2回適用)
            let double_move = base_move.apply_move(&base_move);
            cube.moves.insert(format!("{}2", face), double_move.clone());

            // 反時計回り90度回転 (3回適用)
            let counter_move = double_move.apply_move(&base_move);
            cube.moves.insert(format!("{}'", face), counter_move);
        }

        cube
    }

    pub fn scramble_to_state(&self, scramble: &str) -> State {
        let mut state = State::solved();

        for move_name in scramble.split_whitespace() {
            if let Some(move_state) = self.moves.get(move_name) {
                state = state.apply_move(move_state);
            } else {
                eprintln!("警告: 未知の操作 '{}'", move_name);
            }
        }

        state
    }

    pub fn get_move_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.moves.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn apply_move(&self, state: &State, move_name: &str) -> Option<State> {
        self.moves.get(move_name).map(|m| state.apply_move(m))
    }
}

impl Default for RubiksCube {
    fn default() -> Self {
        Self::new()
    }
}

/// NotationMoveを状態に適用する
pub fn apply_notation_move(state: &State, mv: &NotationMove) -> State {
    let cube = RubiksCube::new();
    let move_name = mv.to_string();
    cube.apply_move(state, &move_name).unwrap_or_else(|| {
        eprintln!("Warning: Unknown move '{}'", move_name);
        state.clone()
    })
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rubiks_cube_scramble() {
        let cube = RubiksCube::new();
        let scramble = "M' U M2 U M2 U M' U2 M2 U'";
        let desired_state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 5, 4, 7, 6, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let result_state = cube.scramble_to_state(scramble);
        assert_eq!(result_state, desired_state);
    }

    #[test]
    fn test_rubiks_cube_scramble_a() {
        let cube = RubiksCube::new();
        let scramble = "D R U R' D' R U' R' U2 M' U2 M M' U2 M U2 U R' D R U' R' D'";
        println!("state: {:?}", cube.scramble_to_state(scramble));
    }
}
