#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub cp: [u8; 8],  // Corner Permutation
    pub co: [u8; 8],  // Corner Orientation
    pub ep: [u8; 12], // Edge Permutation
    pub eo: [u8; 12], // Edge Orientation
}

/// Represents a partial state pattern for comparison
/// Each array element: 0 = don't care, 1 = must match original position
#[derive(Debug, Clone, PartialEq)]
pub struct PartialStatePattern {
    pub desired_edge: [u8; 12],    // Which edge positions to check (0 or 1)
    pub desired_corner: [u8; 8],   // Which corner positions to check (0 or 1)
}

impl State {
    pub fn new(cp: [u8; 8], co: [u8; 8], ep: [u8; 12], eo: [u8; 12]) -> Self {
        State { cp, co, ep, eo }
    }

    pub fn solved() -> Self {
        State {
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            co: [0, 0, 0, 0, 0, 0, 0, 0],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn apply_move(&self, move_state: &State) -> State {
        let mut new_cp = [0u8; 8];
        let mut new_co = [0u8; 8];
        let mut new_ep = [0u8; 12];
        let mut new_eo = [0u8; 12];

        // Corner permutation と orientation の計算
        for (i, &p) in move_state.cp.iter().enumerate() {
            new_cp[i] = self.cp[p as usize];
            new_co[i] = (self.co[p as usize] + move_state.co[i]) % 3;
        }

        // Edge permutation と orientation の計算
        for (i, &p) in move_state.ep.iter().enumerate() {
            new_ep[i] = self.ep[p as usize];
            new_eo[i] = (self.eo[p as usize] + move_state.eo[i]) % 2;
        }

        State::new(new_cp, new_co, new_ep, new_eo)
    }

    pub fn is_solved(&self) -> bool {
        *self == State::solved()
    }
    
    /// Check if this state matches the given partial pattern
    /// For edges and corners marked with 1 in the pattern, they must be in their solved position
    pub fn matches_partial_pattern(&self, pattern: &PartialStatePattern) -> bool {
        let solved = State::solved();
        
        // Check edge positions and orientations where pattern specifies (desired_edge[i] == 1)
        for (i, &should_check) in pattern.desired_edge.iter().enumerate() {
            if should_check == 1 {
                // Edge position must match solved position
                if self.ep[i] != solved.ep[i] {
                    return false;
                }
                // Edge orientation must match solved orientation
                if self.eo[i] != solved.eo[i] {
                    return false;
                }
            }
        }
        
        // Check corner positions and orientations where pattern specifies (desired_corner[i] == 1)
        for (i, &should_check) in pattern.desired_corner.iter().enumerate() {
            if should_check == 1 {
                // Corner position must match solved position
                if self.cp[i] != solved.cp[i] {
                    return false;
                }
                // Corner orientation must match solved orientation
                if self.co[i] != solved.co[i] {
                    return false;
                }
            }
        }
        
        true
    }
}

impl PartialStatePattern {
    /// Create a new partial state pattern
    pub fn new(desired_edge: [u8; 12], desired_corner: [u8; 8]) -> Self {
        PartialStatePattern {
            desired_edge,
            desired_corner,
        }
    }
    
    /// Create a pattern that checks if the top face (U face) is solved
    /// Top face corners are positions 0,1,2,3 and top face edges are positions 0,1,2,3
    pub fn top_face_solved() -> Self {
        PartialStatePattern {
            desired_edge: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            desired_corner: [1, 1, 1, 1, 0, 0, 0, 0],
        }
    }
    
    /// Create a pattern that checks if the bottom face (D face) is solved
    /// Bottom face corners are positions 4,5,6,7 and bottom face edges are positions 4,5,6,7
    pub fn bottom_face_solved() -> Self {
        PartialStatePattern {
            desired_edge: [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
            desired_corner: [0, 0, 0, 0, 1, 1, 1, 1],
        }
    }
    
    /// Create a pattern for the first two layers (F2L) - top and middle layer
    pub fn f2l_solved() -> Self {
        PartialStatePattern {
            desired_edge: [1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1],
            desired_corner: [1, 1, 1, 1, 0, 0, 0, 0],
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Corner Permutation: {:?}", self.cp)?;
        writeln!(f, "Corner Orientation: {:?}", self.co)?;
        writeln!(f, "Edge Permutation:   {:?}", self.ep)?;
        write!(f, "Edge Orientation:   {:?}", self.eo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_state_matches_all_patterns() {
        let solved = State::solved();
        
        // Solved state should match all patterns
        assert!(solved.matches_partial_pattern(&PartialStatePattern::top_face_solved()));
        assert!(solved.matches_partial_pattern(&PartialStatePattern::bottom_face_solved()));
        assert!(solved.matches_partial_pattern(&PartialStatePattern::f2l_solved()));
    }

    #[test]
    fn test_custom_partial_pattern() {
        // Create a pattern that only checks first 4 edges and first 4 corners
        let pattern = PartialStatePattern::new(
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], // Check first 4 edges
            [1, 1, 1, 1, 0, 0, 0, 0],              // Check first 4 corners
        );
        
        let solved = State::solved();
        assert!(solved.matches_partial_pattern(&pattern));
        
        // Create a state where only the bottom layer is scrambled
        let partially_scrambled = State::new(
            [0, 1, 2, 3, 7, 6, 5, 4], // Bottom corners scrambled
            [0, 0, 0, 0, 2, 1, 2, 1], // Bottom corner orientations changed
            [0, 1, 2, 3, 7, 6, 5, 4, 8, 9, 10, 11], // Bottom edges scrambled
            [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],   // Bottom edge orientations changed
        );
        
        // Should match the pattern because top face is still solved
        assert!(partially_scrambled.matches_partial_pattern(&pattern));
        assert!(partially_scrambled.matches_partial_pattern(&PartialStatePattern::top_face_solved()));
        assert!(!partially_scrambled.matches_partial_pattern(&PartialStatePattern::bottom_face_solved()));
    }

    #[test]
    fn test_pattern_with_example_from_docs() {
        // Test the example from the documentation
        let pattern = PartialStatePattern::new(
            [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0], // desired_edge
            [1, 1, 1, 1, 0, 0, 0, 0],              // desired_corner
        );
        
        // Create a state that matches the example
        let test_state = State::new(
            [0, 1, 2, 3, 7, 6, 5, 4], // cp: first 4 corners in correct position
            [0, 0, 0, 0, 2, 1, 0, 1], // co: first 4 corners with correct orientation
            [10, 11, 8, 9, 4, 5, 6, 7, 0, 1, 2, 3], // ep: edges 4,5,6,7 in correct position
            [1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1],   // eo: edges 4,5,6,7 with correct orientation
        );
        
        assert!(test_state.matches_partial_pattern(&pattern));
    }

    #[test]
    fn test_non_matching_patterns() {
        
        // Create a state where top corners are scrambled
        let scrambled_top = State::new(
            [1, 0, 3, 2, 4, 5, 6, 7], // Top corners scrambled
            [1, 2, 0, 1, 0, 0, 0, 0], // Top corner orientations changed
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        
        // Should not match top face pattern
        assert!(!scrambled_top.matches_partial_pattern(&PartialStatePattern::top_face_solved()));
        // But should match bottom face pattern
        assert!(scrambled_top.matches_partial_pattern(&PartialStatePattern::bottom_face_solved()));
    }
}