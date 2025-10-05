// src/lib.rs - ライブラリのエントリーポイント
pub mod cube;
pub mod display;

// 公開API
pub use cube::{State, RubiksCube};
pub use display::{CubeDisplay, CubeNetWidget, CubeColor};

//===============================================
// src/cube/mod.rs - キューブロジックモジュール
pub mod state;
pub mod operations;

pub use state::State;
pub use operations::RubiksCube;

//===============================================
// src/cube/state.rs - 元のState構造体
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

//===============================================
// src/cube/operations.rs - 操作ロジック
use super::state::State;
use std::collections::HashMap;

pub struct RubiksCube {
    moves: HashMap<String, State>,
}

impl RubiksCube {
    pub fn new() -> Self {
        // 既存の実装をここに移動
        // ... (既存のnew()実装)
        todo!("Move existing implementation here")
    }

    pub fn scramble_to_state(&self, scramble: &str) -> State {
        // 既存の実装をここに移動
        todo!("Move existing implementation here")
    }

    pub fn get_move_names(&self) -> Vec<String> {
        // 既存の実装をここに移動
        todo!("Move existing implementation here")
    }

    pub fn apply_move(&self, state: &State, move_name: &str) -> Option<State> {
        // 既存の実装をここに移動
        todo!("Move existing implementation here")
    }
}

//===============================================
// src/display/mod.rs - 表示モジュール
pub mod cube_net;
pub mod colors;
pub mod conversion;

pub use cube_net::CubeNetWidget;
pub use colors::{CubeColor, CubeFace, CubeDisplay};
pub use conversion::StateToDisplay;

//===============================================
// src/display/colors.rs - 色とフェイス定義
use ratatui::style::Color;

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
}

#[derive(Debug, Clone)]
pub struct CubeDisplay {
    pub faces: std::collections::HashMap<Face, CubeFace>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Face {
    Up, Down, Left, Right, Front, Back,
}

impl CubeDisplay {
    pub fn new_solved() -> Self {
        let mut faces = std::collections::HashMap::new();
        faces.insert(Face::Up, CubeFace::new(CubeColor::White));
        faces.insert(Face::Down, CubeFace::new(CubeColor::Yellow));
        faces.insert(Face::Left, CubeFace::new(CubeColor::Orange));
        faces.insert(Face::Right, CubeFace::new(CubeColor::Red));
        faces.insert(Face::Front, CubeFace::new(CubeColor::Green));
        faces.insert(Face::Back, CubeFace::new(CubeColor::Blue));
        
        Self { faces }
    }
}

//===============================================
// src/display/conversion.rs - State <-> CubeDisplay変換
use crate::cube::State;
use super::{CubeDisplay, CubeColor, CubeFace, Face};

pub struct StateToDisplay;

impl StateToDisplay {
    /// キューブの内部状態からCubeDisplayに変換
    pub fn convert(state: &State) -> CubeDisplay {
        // パーツの位置情報(cp, ep)と向き情報(co, eo)から
        // 実際の各面の色配置を計算する
        
        // この実装は複雑になるため、まずはシンプルな例から始める
        // TODO: 実際の変換アルゴリズムを実装
        CubeDisplay::new_solved() // 暫定実装
    }
}

//===============================================
// src/display/cube_net.rs - Ratataui表示ウィジェット
use ratatui::{prelude::*, widgets::*};
use super::{CubeDisplay, Face};

pub struct CubeNetWidget<'a> {
    cube: &'a CubeDisplay,
    title: Option<String>,
    show_borders: bool,
}

impl<'a> CubeNetWidget<'a> {
    pub fn new(cube: &'a CubeDisplay) -> Self {
        Self {
            cube,
            title: None,
            show_borders: true,
        }
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn borders(mut self, show: bool) -> Self {
        self.show_borders = show;
        self
    }
}

impl<'a> Widget for CubeNetWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ウィジェットの実装
        // TODO: 実際のレンダリングロジック
        let block = Block::default()
            .title(self.title.unwrap_or_else(|| "Cube".to_string()))
            .borders(Borders::ALL);
        block.render(area, buf);
    }
}

//===============================================
// src/main.rs - メイン関数
use rubiks_cube_simulator::{State, RubiksCube, CubeDisplay, StateToDisplay, CubeNetWidget};

fn main() {
    println!("🧩 ルービックキューブシミュレーター");
    
    // キューブロジック
    let cube = RubiksCube::new();
    let solved = State::solved();
    
    // 表示機能
    let display = StateToDisplay::convert(&solved);
    let widget = CubeNetWidget::new(&display)
        .title("Solved Cube")
        .borders(true);
    
    // ... TUI実装
}