use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CubeColor {
    White,
    Yellow,
    Orange,
    Red,
    Green,
    Blue,
}

impl CubeColor {
    pub fn to_ratatui_color(&self) -> Color {
        match self {
            CubeColor::White => Color::White,
            CubeColor::Yellow => Color::Yellow,
            CubeColor::Orange => Color::Rgb(255, 165, 0),
            CubeColor::Red => Color::Red,
            CubeColor::Green => Color::Green,
            CubeColor::Blue => Color::Blue,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            CubeColor::White => 'W',
            CubeColor::Yellow => 'Y',
            CubeColor::Orange => 'O',
            CubeColor::Red => 'R',
            CubeColor::Green => 'G',
            CubeColor::Blue => 'B',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Face {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Debug, Clone)]
pub struct CubeFace {
    pub cells: [[CubeColor; 3]; 3],
}

impl CubeFace {
    pub fn new(color: CubeColor) -> Self {
        Self {
            cells: [[color; 3]; 3],
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> CubeColor {
        self.cells[row][col]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, color: CubeColor) {
        self.cells[row][col] = color;
    }
}

#[derive(Debug, Clone)]
pub struct CubeDisplay {
    pub faces: HashMap<Face, CubeFace>,
}

impl CubeDisplay {
    pub fn new_solved() -> Self {
        let mut faces = HashMap::new();
        faces.insert(Face::Up, CubeFace::new(CubeColor::White));
        faces.insert(Face::Down, CubeFace::new(CubeColor::Yellow));
        faces.insert(Face::Left, CubeFace::new(CubeColor::Orange));
        faces.insert(Face::Right, CubeFace::new(CubeColor::Red));
        faces.insert(Face::Front, CubeFace::new(CubeColor::Green));
        faces.insert(Face::Back, CubeFace::new(CubeColor::Blue));

        Self { faces }
    }

    pub fn get_face(&self, face: &Face) -> Option<&CubeFace> {
        self.faces.get(face)
    }

    pub fn get_face_mut(&mut self, face: &Face) -> Option<&mut CubeFace> {
        self.faces.get_mut(face)
    }

    pub fn to_debug_string(&self) -> String {
        let mut output = String::new();
        output.push_str("CubeDisplay {\n");
        
        let faces_order = [
            (Face::Up, "Up"),
            (Face::Front, "Front"), 
            (Face::Right, "Right"),
            (Face::Back, "Back"),
            (Face::Left, "Left"),
            (Face::Down, "Down"),
        ];
        
        for (face, name) in faces_order {
            output.push_str(&format!("  {} Face:\n", name));
            if let Some(cube_face) = self.get_face(&face) {
                for row in 0..3 {
                    output.push_str("    ");
                    for col in 0..3 {
                        let color = cube_face.get_cell(row, col);
                        output.push(color.to_char());
                        output.push(' ');
                    }
                    output.push('\n');
                }
            }
            output.push('\n');
        }
        
        output.push('}');
        output
    }
}