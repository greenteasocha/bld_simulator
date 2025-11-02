use super::{CubeColor, CubeDisplay, Face, CubeStickers, CornerSticker, EdgeSticker};
use crate::cube::State;

pub struct StateToDisplay;

#[derive(Debug, Clone)]
struct StickerTarget {
    face: Face,
    corner_position: Option<CornerSticker>,
    edge_position: Option<EdgeSticker>,
}

impl StickerTarget {
    fn corner(face: Face, position: CornerSticker) -> Self {
        Self {
            face,
            corner_position: Some(position),
            edge_position: None,
        }
    }
    
    fn edge(face: Face, position: EdgeSticker) -> Self {
        Self {
            face,
            corner_position: None,
            edge_position: Some(position),
        }
    }
}

impl StateToDisplay {
    /// キューブの内部状態からCubeDisplayに変換（メモ仕様に基づく完全実装）
    pub fn convert(state: &State) -> CubeDisplay {
        // Phase 1: Initialize cube_stickers with all stickers set to void
        let mut cube_stickers = CubeStickers::new_void();
        
        // Phase 2: Paint center stickers
        Self::paint_centers(&mut cube_stickers);
        
        // Phase 3: Paint corner stickers
        Self::paint_corners(&mut cube_stickers, state);
        
        // Phase 4: Paint edge stickers
        Self::paint_edges(&mut cube_stickers, state);
        
        // Phase 6: Convert to CubeDisplay format
        Self::convert_to_display(&cube_stickers)
    }
    
    /// Phase 2: センターステッカーをペイント
    fn paint_centers(cube_stickers: &mut CubeStickers) {
        let center_colors = [
            (Face::Up, CubeColor::White),
            (Face::Down, CubeColor::Yellow),
            (Face::Left, CubeColor::Orange),
            (Face::Right, CubeColor::Red),
            (Face::Front, CubeColor::Green),
            (Face::Back, CubeColor::Blue),
        ];
        
        for (face, color) in center_colors {
            if let Some(face_stickers) = cube_stickers.get_face_mut(&face) {
                face_stickers.center = color;
            }
        }
    }
    
    /// Phase 3: コーナーステッカーをペイント
    fn paint_corners(cube_stickers: &mut CubeStickers, state: &State) {
        for i in 0..8 {
            // Step 3.1: Get original piece colors
            let original_corner = state.cp[i] as usize;
            let base_colors = Self::get_corner_base_colors(original_corner);
            
            // Step 3.2: Apply orientation rotation
            let orientation = state.co[i];
            let rotated_colors = Self::rotate_corner_colors(base_colors, orientation);
            
            // Step 3.3: Get target sticker positions
            let target_stickers = Self::get_corner_affection_mapping(i);
            
            // Step 3.4: Paint target stickers
            for (j, target) in target_stickers.iter().enumerate() {
                if let Some(face_stickers) = cube_stickers.get_face_mut(&target.face) {
                    if let Some(corner_pos) = target.corner_position {
                        face_stickers.set_corner(corner_pos, rotated_colors[j]);
                    }
                }
            }
        }
    }
    
    /// Phase 4: エッジステッカーをペイント
    fn paint_edges(cube_stickers: &mut CubeStickers, state: &State) {
        for i in 0..12 {
            // Step 4.1: Get original piece colors
            let original_edge = state.ep[i] as usize;
            let base_colors = Self::get_edge_base_colors(original_edge);
            
            // Step 4.2: Apply orientation flip
            let orientation = state.eo[i];
            let rotated_colors = if orientation == 1 {
                [base_colors[1], base_colors[0]]
            } else {
                base_colors
            };
            
            // Step 4.3: Get target sticker positions
            let target_stickers = Self::get_edge_affection_mapping(i);
            
            // Step 4.4: Paint target stickers
            for (j, target) in target_stickers.iter().enumerate() {
                if let Some(face_stickers) = cube_stickers.get_face_mut(&target.face) {
                    if let Some(edge_pos) = target.edge_position {
                        face_stickers.set_edge(edge_pos, rotated_colors[j]);
                    }
                }
            }
        }
    }
    
    /// コーナーの基本色配置
    fn get_corner_base_colors(corner: usize) -> [CubeColor; 3] {
        match corner {
            0 => [CubeColor::White, CubeColor::Orange, CubeColor::Blue],   // UBL
            1 => [CubeColor::White, CubeColor::Blue, CubeColor::Red],      // UBR  
            2 => [CubeColor::White, CubeColor::Red, CubeColor::Green],     // UFR
            3 => [CubeColor::White, CubeColor::Green, CubeColor::Orange],  // UFL
            4 => [CubeColor::Yellow, CubeColor::Blue, CubeColor::Orange],  // DBL
            5 => [CubeColor::Yellow, CubeColor::Red, CubeColor::Blue],     // DBR
            6 => [CubeColor::Yellow, CubeColor::Green, CubeColor::Red],    // DFR
            7 => [CubeColor::Yellow, CubeColor::Orange, CubeColor::Green], // DFL
            _ => [CubeColor::Void, CubeColor::Void, CubeColor::Void],
        }
    }
    
    /// コーナーorientation回転適用（メモ仕様）
    fn rotate_corner_colors(base_colors: [CubeColor; 3], orientation: u8) -> [CubeColor; 3] {
        let o = orientation as usize;
        [
            base_colors[(3 - o) % 3],
            base_colors[(4 - o) % 3],
            base_colors[(2 - o) % 3],
        ]
    }
    
    /// コーナーaffection_target_stickerマッピング
    fn get_corner_affection_mapping(corner_idx: usize) -> [StickerTarget; 3] {
        match corner_idx {
            0 => [ // UBL
                StickerTarget::corner(Face::Up, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Left, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Back, CornerSticker::UpperRight),
            ],
            1 => [ // UBR
                StickerTarget::corner(Face::Up, CornerSticker::UpperRight),
                StickerTarget::corner(Face::Back, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Right, CornerSticker::UpperRight),
            ],
            2 => [ // UFR
                StickerTarget::corner(Face::Up, CornerSticker::LowerRight),
                StickerTarget::corner(Face::Right, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Front, CornerSticker::UpperRight),
            ],
            3 => [ // UFL
                StickerTarget::corner(Face::Up, CornerSticker::LowerLeft),
                StickerTarget::corner(Face::Front, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Left, CornerSticker::UpperRight),
            ],
            4 => [ // DBL
                StickerTarget::corner(Face::Down, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Back, CornerSticker::LowerRight),
                StickerTarget::corner(Face::Left, CornerSticker::LowerLeft),
            ],
            5 => [ // DBR
                StickerTarget::corner(Face::Down, CornerSticker::UpperRight),
                StickerTarget::corner(Face::Right, CornerSticker::LowerRight),
                StickerTarget::corner(Face::Back, CornerSticker::LowerLeft),
            ],
            6 => [ // DFR
                StickerTarget::corner(Face::Down, CornerSticker::LowerRight),
                StickerTarget::corner(Face::Front, CornerSticker::LowerRight),
                StickerTarget::corner(Face::Right, CornerSticker::LowerLeft),
            ],
            7 => [ // DFL
                StickerTarget::corner(Face::Down, CornerSticker::LowerLeft),
                StickerTarget::corner(Face::Left, CornerSticker::LowerRight),
                StickerTarget::corner(Face::Front, CornerSticker::LowerLeft),
            ],
            _ => [
                StickerTarget::corner(Face::Up, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Up, CornerSticker::UpperLeft),
                StickerTarget::corner(Face::Up, CornerSticker::UpperLeft),
            ],
        }
    }
    
    /// エッジの基本色配置
    fn get_edge_base_colors(edge: usize) -> [CubeColor; 2] {
        match edge {
            0 => [CubeColor::Blue, CubeColor::Orange],    // BL
            1 => [CubeColor::Blue, CubeColor::Red],       // BR
            2 => [CubeColor::Green, CubeColor::Red],      // FR
            3 => [CubeColor::Green, CubeColor::Orange],   // FL
            4 => [CubeColor::White, CubeColor::Blue],     // UB
            5 => [CubeColor::White, CubeColor::Red],      // UR
            6 => [CubeColor::White, CubeColor::Green],    // UF  
            7 => [CubeColor::White, CubeColor::Orange],   // UL
            8 => [CubeColor::Yellow, CubeColor::Blue],    // DB
            9 => [CubeColor::Yellow, CubeColor::Red],     // DR
            10 => [CubeColor::Yellow, CubeColor::Green],  // DF
            11 => [CubeColor::Yellow, CubeColor::Orange], // DL
            _ => [CubeColor::Void, CubeColor::Void],
        }
    }
    
    /// エッジaffection_target_stickerマッピング
    fn get_edge_affection_mapping(edge_idx: usize) -> [StickerTarget; 2] {
        match edge_idx {
            0 => [ // BL
                StickerTarget::edge(Face::Back, EdgeSticker::Right),
                StickerTarget::edge(Face::Left, EdgeSticker::Left),
            ],
            1 => [ // BR
                StickerTarget::edge(Face::Back, EdgeSticker::Left),
                StickerTarget::edge(Face::Right, EdgeSticker::Right),
            ],
            2 => [ // FR
                StickerTarget::edge(Face::Front, EdgeSticker::Right),
                StickerTarget::edge(Face::Right, EdgeSticker::Left),
            ],
            3 => [ // FL
                StickerTarget::edge(Face::Front, EdgeSticker::Left),
                StickerTarget::edge(Face::Left, EdgeSticker::Right),
            ],
            4 => [ // UB
                StickerTarget::edge(Face::Up, EdgeSticker::Upper),
                StickerTarget::edge(Face::Back, EdgeSticker::Upper),
            ],
            5 => [ // UR
                StickerTarget::edge(Face::Up, EdgeSticker::Right),
                StickerTarget::edge(Face::Right, EdgeSticker::Upper),
            ],
            6 => [ // UF
                StickerTarget::edge(Face::Up, EdgeSticker::Lower),
                StickerTarget::edge(Face::Front, EdgeSticker::Upper),
            ],
            7 => [ // UL
                StickerTarget::edge(Face::Up, EdgeSticker::Left),
                StickerTarget::edge(Face::Left, EdgeSticker::Upper),
            ],
            8 => [ // DB
                StickerTarget::edge(Face::Down, EdgeSticker::Upper),
                StickerTarget::edge(Face::Back, EdgeSticker::Lower),
            ],
            9 => [ // DR
                StickerTarget::edge(Face::Down, EdgeSticker::Right),
                StickerTarget::edge(Face::Right, EdgeSticker::Lower),
            ],
            10 => [ // DF
                StickerTarget::edge(Face::Down, EdgeSticker::Lower),
                StickerTarget::edge(Face::Front, EdgeSticker::Lower),
            ],
            11 => [ // DL
                StickerTarget::edge(Face::Down, EdgeSticker::Left),
                StickerTarget::edge(Face::Left, EdgeSticker::Lower),
            ],
            _ => [
                StickerTarget::edge(Face::Up, EdgeSticker::Upper),
                StickerTarget::edge(Face::Up, EdgeSticker::Upper),
            ],
        }
    }
    
    /// CubeStickers → CubeDisplay変換（デバッグ用にCubeStickersも返す）
    pub fn convert_with_stickers(state: &State) -> (CubeDisplay, CubeStickers) {
        // Phase 1: Initialize cube_stickers with all stickers set to void
        let mut cube_stickers = CubeStickers::new_void();
        
        // Phase 2: Paint center stickers
        Self::paint_centers(&mut cube_stickers);
        
        // Phase 3: Paint corner stickers
        Self::paint_corners(&mut cube_stickers, state);
        
        // Phase 4: Paint edge stickers
        Self::paint_edges(&mut cube_stickers, state);
        
        // Phase 6: Convert to CubeDisplay format
        let display = Self::convert_to_display(&cube_stickers);
        
        (display, cube_stickers)
    }
    
    /// Phase 6: CubeStickers → CubeDisplay変換
    fn convert_to_display(cube_stickers: &CubeStickers) -> CubeDisplay {
        let mut display = CubeDisplay::new_solved();
        
        for (face, face_stickers) in &cube_stickers.faces {
            if let Some(cube_face) = display.get_face_mut(face) {
                let grid = face_stickers.to_3x3_grid();
                for (row, row_colors) in grid.iter().enumerate() {
                    for (col, &color) in row_colors.iter().enumerate() {
                        cube_face.set_cell(row, col, color);
                    }
                }
            }
        }
        
        display
    }
}