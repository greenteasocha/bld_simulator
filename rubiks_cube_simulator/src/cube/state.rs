#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub cp: [u8; 8],  // Corner Permutation
    pub co: [u8; 8],  // Corner Orientation
    pub ep: [u8; 12], // Edge Permutation
    pub eo: [u8; 12], // Edge Orientation
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
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Corner Permutation: {:?}", self.cp)?;
        writeln!(f, "Corner Orientation: {:?}", self.co)?;
        writeln!(f, "Edge Permutation:   {:?}", self.ep)?;
        write!(f, "Edge Orientation:   {:?}", self.eo)
    }
}