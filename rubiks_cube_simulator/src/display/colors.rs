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
    Void, // 未塗装状態
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
            CubeColor::Void => Color::DarkGray,
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
            CubeColor::Void => '?',
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CornerSticker {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeSticker {
    Upper,
    Right,
    Lower,
    Left,
}

#[derive(Debug, Clone)]
pub struct FaceStickers {
    pub corners: [CubeColor; 4], // UpperLeft, UpperRight, LowerRight, LowerLeft
    pub edges: [CubeColor; 4],   // Upper, Right, Lower, Left
    pub center: CubeColor,
}

impl FaceStickers {
    pub fn new_void() -> Self {
        Self {
            corners: [CubeColor::Void; 4],
            edges: [CubeColor::Void; 4],
            center: CubeColor::Void,
        }
    }
    
    pub fn set_corner(&mut self, position: CornerSticker, color: CubeColor) {
        let idx = match position {
            CornerSticker::UpperLeft => 0,
            CornerSticker::UpperRight => 1,
            CornerSticker::LowerRight => 2,
            CornerSticker::LowerLeft => 3,
        };
        self.corners[idx] = color;
    }
    
    pub fn set_edge(&mut self, position: EdgeSticker, color: CubeColor) {
        let idx = match position {
            EdgeSticker::Upper => 0,
            EdgeSticker::Right => 1,
            EdgeSticker::Lower => 2,
            EdgeSticker::Left => 3,
        };
        self.edges[idx] = color;
    }
    
    pub fn to_3x3_grid(&self) -> [[CubeColor; 3]; 3] {
        [
            [self.corners[0], self.edges[0], self.corners[1]], // Top row: UL, U, UR
            [self.edges[3], self.center, self.edges[1]],       // Mid row: L, C, R
            [self.corners[3], self.edges[2], self.corners[2]], // Bot row: LL, L, LR
        ]
    }
}

#[derive(Debug, Clone)]
pub struct CubeStickers {
    pub faces: HashMap<Face, FaceStickers>,
}

impl CubeStickers {
    pub fn new_void() -> Self {
        let mut faces = HashMap::new();
        for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
            faces.insert(face, FaceStickers::new_void());
        }
        Self { faces }
    }
    
    pub fn get_face_mut(&mut self, face: &Face) -> Option<&mut FaceStickers> {
        self.faces.get_mut(face)
    }
    
    pub fn to_debug_string(&self) -> String {
        let mut output = String::new();
        output.push_str("CubeStickers {\n");
        
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
            if let Some(face_stickers) = self.faces.get(&face) {
                // コーナーステッカー
                output.push_str("    Corners: [");
                for (i, &color) in face_stickers.corners.iter().enumerate() {
                    if i > 0 { output.push_str(", "); }
                    output.push(color.to_char());
                }
                output.push_str("]\n");
                
                // エッジステッカー
                output.push_str("    Edges:   [");
                for (i, &color) in face_stickers.edges.iter().enumerate() {
                    if i > 0 { output.push_str(", "); }
                    output.push(color.to_char());
                }
                output.push_str("]\n");
                
                // センターステッカー
                output.push_str(&format!("    Center:  {}\n", face_stickers.center.to_char()));
                
                // 3x3グリッド表示
                output.push_str("    Grid:\n");
                let grid = face_stickers.to_3x3_grid();
                for row in grid.iter() {
                    output.push_str("      ");
                    for &color in row.iter() {
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