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

    pub fn clear_line(&mut self, line: usize) {
        self.cells[line] = [None; GRID_WIDTH];
    }

    pub fn shift_lines_down(&mut self, from_line: usize) {
        for y in (1..=from_line).rev() {
            self.cells[y] = self.cells[y - 1];
        }
        self.cells[0] = [None; GRID_WIDTH];
    }

    pub fn is_line_full(&self, line: usize) -> bool {
        self.cells[line].iter().all(|cell| cell.is_some())
    }
}

#[derive(Resource)]
pub struct GameState {
    pub current_piece: Option<ActivePiece>,
    pub next_piece: TetrominoType,
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        let next = TetrominoType::random();
        Self {
            current_piece: Some(ActivePiece::new(next)),
            next_piece: TetrominoType::random(),
            score: 0,
        }
    }
}

impl GameState {
    pub fn add_score(&mut self, lines_cleared: u32) {
        self.score += match lines_cleared {
            1 => SCORE_1_LINE,
            2 => SCORE_2_LINES,
            3 => SCORE_3_LINES,
            4 => SCORE_4_LINES,
            _ => 0,
        };
    }
}

#[derive(Resource)]
pub struct FallTimer {
    pub timer: Timer,
}

impl Default for FallTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(FALL_SPEED, TimerMode::Repeating),
        }
    }
}