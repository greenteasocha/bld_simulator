use super::operations::RubiksCube;
use super::state::{PartialStatePattern, State};
use std::collections::VecDeque;

/// Represents a single move on the Rubik's cube
#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    U,
    D,
    R,
    L,
    F,
    B, // Clockwise moves
    UPrime,
    DPrime,
    RPrime,
    LPrime,
    FPrime,
    BPrime, // Counter-clockwise moves
}

impl Move {
    /// Get all possible moves
    pub fn all_moves() -> Vec<Move> {
        vec![
            Move::U,
            Move::D,
            Move::R,
            Move::L,
            Move::F,
            Move::B,
            Move::UPrime,
            Move::DPrime,
            Move::RPrime,
            Move::LPrime,
            Move::FPrime,
            Move::BPrime,
        ]
    }

    /// Convert move to string notation
    pub fn to_string(&self) -> String {
        match self {
            Move::U => "U".to_string(),
            Move::D => "D".to_string(),
            Move::R => "R".to_string(),
            Move::L => "L".to_string(),
            Move::F => "F".to_string(),
            Move::B => "B".to_string(),
            Move::UPrime => "U'".to_string(),
            Move::DPrime => "D'".to_string(),
            Move::RPrime => "R'".to_string(),
            Move::LPrime => "L'".to_string(),
            Move::FPrime => "F'".to_string(),
            Move::BPrime => "B'".to_string(),
        }
    }

    /// Get the move state for applying this move using RubiksCube
    pub fn apply_to_state(&self, state: &State, cube: &RubiksCube) -> Option<State> {
        let move_name = self.to_string();
        cube.apply_move(state, &move_name)
    }
}

/// Solution searcher that performs breadth-first search
pub struct SolutionSearcher {
    start_state: State,
    desired_pattern: PartialStatePattern,
    max_depth: usize,
    cube: RubiksCube,
    solutions_found: Vec<Vec<Move>>,
}

/// Represents a search node containing state, move sequence, and depth
#[derive(Debug, Clone)]
struct SearchNode {
    state: State,
    moves: Vec<Move>,
    depth: usize,
}

impl SolutionSearcher {
    /// Create a new solution searcher
    pub fn new(start_state: State, desired_pattern: PartialStatePattern, max_depth: usize) -> Self {
        SolutionSearcher {
            start_state,
            desired_pattern,
            max_depth,
            cube: RubiksCube::new(),
            solutions_found: Vec::new(),
        }
    }

    /// Create searcher with the fixed bottom layer pattern from the docs
    pub fn with_bottom_layer_pattern(start_state: State) -> Self {
        let desired_pattern = PartialStatePattern::new(
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1], // Bottom layer edges
            [0, 0, 0, 0, 0, 0, 0, 0],             // No corners required
        );
        SolutionSearcher::new(start_state, desired_pattern, 3)
    }

    /// Search for a solution using breadth-first search
    pub fn search(&mut self) -> Option<Vec<Vec<Move>>> {
        let mut queue = VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        // Start with the initial state
        let start_node = SearchNode {
            state: self.start_state.clone(),
            moves: Vec::new(),
            depth: 0,
        };

        // Check if start state already matches
        if self
            .start_state
            .matches_partial_pattern(&self.desired_pattern)
        {
            return Some(Vec::new()); // Already solved
        }

        queue.push_back(start_node);
        visited.insert(self.start_state.clone());

        while let Some(current) = queue.pop_front() {
            // Don't search beyond max depth
            if current.depth >= self.max_depth {
                continue;
            }

            // Try all possible moves
            for move_option in Move::all_moves() {
                if let Some(new_state) = move_option.apply_to_state(&current.state, &self.cube) {
                    // Skip if we've seen this state before
                    if visited.contains(&new_state) {
                        continue;
                    }

                    visited.insert(new_state.clone());

                    let mut new_moves = current.moves.clone();
                    new_moves.push(move_option);

                    // Check if this state matches our desired pattern
                    if new_state.matches_partial_pattern(&self.desired_pattern) {
                        self.solutions_found.push(new_moves);
                        continue;
                    }

                    // Add to queue for further exploration
                    let new_node = SearchNode {
                        state: new_state,
                        moves: new_moves,
                        depth: current.depth + 1,
                    };
                    queue.push_back(new_node);
                }
            }
        }

        // Return all of solution found, if any
        if !self.solutions_found.is_empty() {
            return Some(self.solutions_found.clone());
        }
        None // No solution found within max depth
    }

    /// Format solution as a string
    pub fn format_solution(moves: &[Move]) -> String {
        if moves.is_empty() {
            "Already solved!".to_string()
        } else {
            moves
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_string() {
        assert_eq!(Move::U.to_string(), "U");
        assert_eq!(Move::UPrime.to_string(), "U'");
        assert_eq!(Move::R.to_string(), "R");
        assert_eq!(Move::RPrime.to_string(), "R'");
    }

    #[test]
    fn test_all_moves_count() {
        assert_eq!(Move::all_moves().len(), 12);
    }

    #[test]
    fn test_solution_searcher_creation() {
        let state = State::solved();
        let searcher = SolutionSearcher::with_bottom_layer_pattern(state);
        assert_eq!(searcher.max_depth, 3);
    }

    #[test]
    fn test_format_solution() {
        let moves = vec![Move::U, Move::RPrime, Move::F];
        assert_eq!(SolutionSearcher::format_solution(&moves), "U R' F");

        let empty_moves = vec![];
        assert_eq!(
            SolutionSearcher::format_solution(&empty_moves),
            "Already solved!"
        );
    }

    // テスト追加。完成状態から U R F を施した State に対して Solution が F' R' になることを確認
    #[test]
    fn test_solution_search() {
        let cube = RubiksCube::new();
        let mut state = State::solved();
        state = cube.apply_move(&state, "U").unwrap();
        state = cube.apply_move(&state, "R").unwrap();
        state = cube.apply_move(&state, "F").unwrap();

        let mut searcher = SolutionSearcher::with_bottom_layer_pattern(state);
        let solution = searcher.search();

        // U は bottom layer に影響しないので、F' R' で解けるはず
        assert!(solution.is_some());
        let solutions = solution.unwrap();
        assert_eq!(SolutionSearcher::format_solution(&solutions[0]), "F' R'");

        // debug 用に all solution を表示
        println!("Input: U R F");
        println!("All cross solutions found:");

        for (i, sol) in solutions.iter().enumerate() {
            println!(
                "Solution {}: {}",
                i + 1,
                SolutionSearcher::format_solution(sol)
            );
        }

        // solution の数は7つであること
        assert_eq!(solutions.len(), 7);
    }
}
