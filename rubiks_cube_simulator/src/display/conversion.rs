use super::{CubeColor, CubeDisplay, Face};
use crate::cube::State;

pub struct StateToDisplay;

impl StateToDisplay {
    /// キューブの内部状態からCubeDisplayに変換
    pub fn convert(state: &State) -> CubeDisplay {
        let mut display = CubeDisplay::new_solved();

        // 各面の色を実際の状態から計算
        let face_colors = Self::calculate_face_colors_from_state(state);

        // CubeDisplayの各面を更新
        for (face, colors) in face_colors {
            if let Some(cube_face) = display.get_face_mut(&face) {
                for (row, row_colors) in colors.iter().enumerate() {
                    for (col, &color) in row_colors.iter().enumerate() {
                        cube_face.set_cell(row, col, color);
                    }
                }
            }
        }

        display
    }

    /// 実際の状態から各面の色配置を計算
    fn calculate_face_colors_from_state(
        state: &State,
    ) -> std::collections::HashMap<Face, [[CubeColor; 3]; 3]> {
        let mut face_colors = std::collections::HashMap::new();

        // 各面を計算
        for face in [
            Face::Up,
            Face::Down,
            Face::Left,
            Face::Right,
            Face::Front,
            Face::Back,
        ] {
            face_colors.insert(face.clone(), Self::calculate_single_face(face, state));
        }

        face_colors
    }

    /// 単一面の色配置を計算
    fn calculate_single_face(face: Face, state: &State) -> [[CubeColor; 3]; 3] {
        let mut colors = [[CubeColor::White; 3]; 3];

        // センター色（固定）
        let center_color = match face {
            Face::Up => CubeColor::White,
            Face::Down => CubeColor::Yellow,
            Face::Left => CubeColor::Orange,
            Face::Right => CubeColor::Red,
            Face::Front => CubeColor::Green,
            Face::Back => CubeColor::Blue,
        };
        colors[1][1] = center_color;

        // コーナーピースの計算
        let corners = Self::get_face_corners(&face);
        for (i, &corner_idx) in corners.iter().enumerate() {
            let actual_corner = state.cp[corner_idx] as usize;
            let orientation = state.co[corner_idx];
            let corner_colors = Self::get_corner_colors(actual_corner, orientation);
            let face_color = Self::get_color_on_face(&face, corner_colors);

            let (row, col) = match i {
                0 => (0, 0), // 左上
                1 => (0, 2), // 右上
                2 => (2, 2), // 右下
                3 => (2, 0), // 左下
                _ => unreachable!(),
            };
            colors[row][col] = face_color;
        }

        // エッジピースの計算
        let edges = Self::get_face_edges(&face);
        for (i, &edge_idx) in edges.iter().enumerate() {
            let actual_edge = state.ep[edge_idx] as usize;
            let orientation = state.eo[edge_idx];
            let edge_colors = Self::get_edge_colors(actual_edge, orientation);
            let face_color = Self::get_edge_color_on_face(&face, edge_colors);

            let (row, col) = match i {
                0 => (0, 1), // 上
                1 => (1, 2), // 右
                2 => (2, 1), // 下
                3 => (1, 0), // 左
                _ => unreachable!(),
            };
            colors[row][col] = face_color;
        }

        colors
    }

    /// 面のコーナー位置を取得
    fn get_face_corners(face: &Face) -> [usize; 4] {
        match face {
            Face::Up => [0, 1, 2, 3],    // UBL, UBR, UFR, UFL
            Face::Down => [4, 7, 6, 5],  // DBL, DFL, DFR, DBR
            Face::Left => [0, 3, 7, 4],  // UBL, UFL, DFL, DBL
            Face::Right => [2, 1, 5, 6], // UFR, UBR, DBR, DFR
            Face::Front => [3, 2, 6, 7], // UFL, UFR, DFR, DFL
            Face::Back => [1, 0, 4, 5],  // UBR, UBL, DBL, DBR
        }
    }

    /// 面のエッジ位置を取得
    fn get_face_edges(face: &Face) -> [usize; 4] {
        match face {
            Face::Up => [0, 1, 2, 3],     // UB, UR, UF, UL
            Face::Down => [8, 11, 10, 9], // DB, DL, DF, DR
            Face::Left => [3, 7, 11, 4],  // UL, FL, DL, BL
            Face::Right => [1, 5, 9, 6],  // UR, FR, DR, BR
            Face::Front => [2, 5, 10, 7], // UF, FR, DF, FL
            Face::Back => [0, 4, 8, 6],   // UB, BL, DB, BR
        }
    }

    /// コーナーの色配置を取得（orientation適用済み）
    fn get_corner_colors(corner: usize, orientation: u8) -> [CubeColor; 3] {
        let base_colors = match corner {
            0 => [CubeColor::White, CubeColor::Blue, CubeColor::Orange], // UBL
            1 => [CubeColor::White, CubeColor::Red, CubeColor::Blue],    // UBR
            2 => [CubeColor::White, CubeColor::Green, CubeColor::Red],   // UFR
            3 => [CubeColor::White, CubeColor::Orange, CubeColor::Green], // UFL
            4 => [CubeColor::Yellow, CubeColor::Orange, CubeColor::Blue], // DBL
            5 => [CubeColor::Yellow, CubeColor::Blue, CubeColor::Red],   // DBR
            6 => [CubeColor::Yellow, CubeColor::Red, CubeColor::Green],  // DFR
            7 => [CubeColor::Yellow, CubeColor::Green, CubeColor::Orange], // DFL
            _ => [CubeColor::White, CubeColor::White, CubeColor::White],
        };

        // orientation に基づいて色を回転
        match orientation {
            0 => base_colors,
            1 => [base_colors[2], base_colors[0], base_colors[1]],
            2 => [base_colors[1], base_colors[2], base_colors[0]],
            _ => base_colors,
        }
    }

    /// エッジの色配置を取得（orientation適用済み）
    fn get_edge_colors(edge: usize, orientation: u8) -> [CubeColor; 2] {
        let base_colors = match edge {
            0 => [CubeColor::White, CubeColor::Blue],     // UB
            1 => [CubeColor::White, CubeColor::Red],      // UR
            2 => [CubeColor::White, CubeColor::Green],    // UF
            3 => [CubeColor::White, CubeColor::Orange],   // UL
            4 => [CubeColor::Orange, CubeColor::Blue],    // BL
            5 => [CubeColor::Green, CubeColor::Red],      // FR
            6 => [CubeColor::Red, CubeColor::Blue],       // BR
            7 => [CubeColor::Orange, CubeColor::Green],   // FL
            8 => [CubeColor::Yellow, CubeColor::Blue],    // DB
            9 => [CubeColor::Yellow, CubeColor::Red],     // DR
            10 => [CubeColor::Yellow, CubeColor::Green],  // DF
            11 => [CubeColor::Yellow, CubeColor::Orange], // DL
            _ => [CubeColor::White, CubeColor::White],
        };

        // orientation に基づいて色を反転
        if orientation == 1 {
            [base_colors[1], base_colors[0]]
        } else {
            base_colors
        }
    }

    /// コーナーの特定面での色を取得
    fn get_color_on_face(face: &Face, colors: [CubeColor; 3]) -> CubeColor {
        match face {
            Face::Up | Face::Down => colors[0],
            Face::Left | Face::Right => colors[2],
            Face::Front | Face::Back => colors[1],
        }
    }

    /// エッジの特定面での色を取得
    fn get_edge_color_on_face(face: &Face, colors: [CubeColor; 2]) -> CubeColor {
        match face {
            Face::Up | Face::Down => colors[0],
            Face::Left | Face::Right | Face::Front | Face::Back => colors[1],
        }
    }
}
