// 游戏资源模块：定义游戏全局状态和数据

use crate::constants::*;
use crate::tetromino::{ActivePiece, TetrominoType};
use bevy::prelude::*;

/// 游戏板资源
/// 存储游戏板上已锁定方块的信息，使用二维数组表示网格
#[derive(Resource)]
pub struct GameBoard {
    /// 游戏板单元格数组，行x列的二维数组，每个单元格存储颜色（None表示空）
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
    /// 检查指定单元格是否为空
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.cells[y][x].is_none()
    }

    /// 设置指定单元格的颜色
    ///
    /// # 参数
    /// * `x` - 列坐标
    /// * `y` - 行坐标
    /// * `color` - 要设置的颜色（None表示清空）
    pub fn set_cell(&mut self, x: usize, y: usize, color: Option<Color>) {
        self.cells[y][x] = color;
    }

    /// 清空指定行（消行时使用）
    pub fn clear_line(&mut self, line: usize) {
        self.cells[line] = [None; GRID_WIDTH];
    }

    /// 将指定行及以上的所有行向下移动一行（消行后移动）
    ///
    /// # 参数
    /// * `from_line` - 要开始移动的行号
    pub fn shift_lines_down(&mut self, from_line: usize) {
        // 从上往下遍历，将每一行复制到下一行
        for y in (1..=from_line).rev() {
            self.cells[y] = self.cells[y - 1];
        }
        // 顶部行清空
        self.cells[0] = [None; GRID_WIDTH];
    }

    /// 检查指定行是否已满（所有单元格都有方块）
    pub fn is_line_full(&self, line: usize) -> bool {
        self.cells[line].iter().all(|cell| cell.is_some())
    }
}

/// 游戏状态资源
/// 管理当前游戏的运行状态和得分
#[derive(Resource)]
pub struct GameState {
    /// 当前下落的方块（None表示没有活跃方块）
    pub current_piece: Option<ActivePiece>,
    /// 下一个将要出现的方块类型
    pub next_piece: TetrominoType,
    /// 当前分数
    pub score: u32,
    /// 游戏是否结束
    pub game_over: bool,
    /// 游戏是否暂停
    pub paused: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_piece: None,
            next_piece: TetrominoType::random(),
            score: 0,
            game_over: false,
            paused: false,
        }
    }
}

impl GameState {
    /// 重置游戏状态（用于重新开始游戏）
    pub fn reset(&mut self) {
        self.score = 0;
        self.game_over = false;
        self.paused = false;
        self.current_piece = None;
        self.next_piece = TetrominoType::random();
    }

    /// 根据消行数量增加分数
    ///
    /// # 参数
    /// * `lines_cleared` - 消行的数量（1-4行）
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

/// 方块掉落计时器资源
/// 控制方块的定时下落
#[derive(Resource)]
pub struct FallTimer {
    /// Bevy 计时器，周期性触发
    pub timer: Timer,
}

impl Default for FallTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(FALL_SPEED, TimerMode::Repeating),
        }
    }
}
