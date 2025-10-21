use crate::constants::*;
use crate::tetromino::{ActivePiece, TetrominoType};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameBoard {
    pub cells: [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT],
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            cells: [[None; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
}

impl GameBoard {
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.cells[y][x].is_none()
    }

    pub fn set_cell(&mut self, x: usize, y: usize, color: Option<Color>) {
        self.cells[y][x] = color;
    }
}

#[derive(Resource)]
pub struct GameState {
    pub current_piece: Option<ActivePiece>,
    pub next_piece: TetrominoType,
}

impl Default for GameState {
    fn default() -> Self {
        let next = TetrominoType::random();
        Self {
            current_piece: Some(ActivePiece::new(next)),
            next_piece: TetrominoType::random(),
        }
    }
}
