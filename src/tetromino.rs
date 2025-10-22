use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            _ => TetrominoType::L,
        }
    }

    pub fn shape(&self) -> Vec<(i32, i32)> {
        match self {
            TetrominoType::I => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            TetrominoType::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            TetrominoType::T => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
            TetrominoType::S => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
            TetrominoType::Z => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
            TetrominoType::J => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
            TetrominoType::L => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
        }
    }

    pub fn color(&self) -> Color {
        match self {
            TetrominoType::I => Color::srgb(0.0, 0.9, 0.9),
            TetrominoType::O => Color::srgb(0.9, 0.9, 0.0),
            TetrominoType::T => Color::srgb(0.7, 0.0, 0.9),
            TetrominoType::S => Color::srgb(0.0, 0.9, 0.0),
            TetrominoType::Z => Color::srgb(0.9, 0.0, 0.0),
            TetrominoType::J => Color::srgb(0.0, 0.0, 0.9),
            TetrominoType::L => Color::srgb(0.9, 0.5, 0.0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ActivePiece {
    pub tetromino_type: TetrominoType,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}

impl ActivePiece {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        Self {
            tetromino_type,
            x: (crate::constants::GRID_WIDTH as i32) / 2 - 2,
            y: 0,
            rotation: 0,
        }
    }

    pub fn blocks(&self) -> Vec<(i32, i32)> {
        let shape = self.tetromino_type.shape();
        shape
            .iter()
            .map(|&(dx, dy)| {
                let (rx, ry) = self.rotate_point(dx, dy);
                (self.x + rx, self.y + ry)
            })
            .collect()
    }

    pub fn check_collision(&self, dx: i32, dy: i32, board: &crate::resources::GameBoard) -> bool {
        use crate::constants::*;

        for (x, y) in self.blocks() {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_x >= GRID_WIDTH as i32 {
                return true;
            }
            if new_y >= GRID_HEIGHT as i32 {
                return true;
            }

            if new_y >= 0 && !board.is_empty(new_x as usize, new_y as usize) {
                return true;
            }
        }
        false
    }

    fn rotate_point(&self, x: i32, y: i32) -> (i32, i32) {
        match self.rotation % 4 {
            0 => (x, y),
            1 => (-y, x),
            2 => (-x, -y),
            3 => (y, -x),
            _ => unreachable!(),
        }
    }
}
