// 俄罗斯方块类型和逻辑模块

use bevy::prelude::*;
use rand::Rng;

/// 俄罗斯方块类型枚举
///
/// 包含7种标准俄罗斯方块类型：
/// - I: 直线（青色，4格连成一线）
/// - O: 正方形（黄色，2x2方形）
/// - T: T形（紫色）
/// - S: S形（绿色）
/// - Z: Z形（红色）
/// - J: J形（蓝色）
/// - L: L形（橙色）
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
    /// 随机生成一个俄罗斯方块类型
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

    /// 获取方块的形状（相对坐标）
    /// 返回方块相对于中心点的四个格子位置
    pub fn shape(&self) -> Vec<(i32, i32)> {
        match self {
            // I 形：四格成一条线
            TetrominoType::I => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            // O 形：2x2 正方形
            TetrominoType::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            // T 形：T 字形
            TetrominoType::T => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
            // S 形：Z 字向左倾
            TetrominoType::S => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
            // Z 形：Z 字向右倾
            TetrominoType::Z => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
            // J 形：J 字形
            TetrominoType::J => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
            // L 形：L 字形
            TetrominoType::L => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
        }
    }

    /// 获取方块的显示颜色
    pub fn color(&self) -> Color {
        match self {
            TetrominoType::I => Color::srgb(0.0, 0.9, 0.9), // 青色
            TetrominoType::O => Color::srgb(0.9, 0.9, 0.0), // 黄色
            TetrominoType::T => Color::srgb(0.7, 0.0, 0.9), // 紫色
            TetrominoType::S => Color::srgb(0.0, 0.9, 0.0), // 绿色
            TetrominoType::Z => Color::srgb(0.9, 0.0, 0.0), // 红色
            TetrominoType::J => Color::srgb(0.0, 0.0, 0.9), // 蓝色
            TetrominoType::L => Color::srgb(0.9, 0.5, 0.0), // 橙色
        }
    }
}

/// 活跃方块（当前下落的方块）
/// 存储方块的类型、位置和旋转状态
#[derive(Clone, Copy)]
pub struct ActivePiece {
    /// 方块类型
    pub tetromino_type: TetrominoType,
    /// 方块中心的 X 坐标（网格坐标）
    pub x: i32,
    /// 方块中心的 Y 坐标（网格坐标）
    pub y: i32,
    /// 旋转角度（0-3，表示 0°、90°、180°、270°）
    pub rotation: u8,
}

impl ActivePiece {
    /// 创建新的活跃方块
    /// 方块从顶部中心位置生成
    pub fn new(tetromino_type: TetrominoType) -> Self {
        Self {
            tetromino_type,
            x: (crate::constants::GRID_WIDTH as i32) / 2 - 2,
            y: 0,
            rotation: 0,
        }
    }

    /// 获取方块当前的所有格子位置（考虑旋转）
    /// 返回包含方块占据的所有格子的绝对坐标
    pub fn blocks(&self) -> Vec<(i32, i32)> {
        let shape = self.tetromino_type.shape();
        shape
            .iter()
            .map(|&(dx, dy)| {
                // 根据旋转角度变换相对坐标
                let (rx, ry) = self.rotate_point(dx, dy);
                // 转换为绝对坐标
                (self.x + rx, self.y + ry)
            })
            .collect()
    }

    /// 检查方块移动后是否会发生碰撞
    ///
    /// # 参数
    /// * `dx` - X 方向的移动偏移量
    /// * `dy` - Y 方向的移动偏移量
    /// * `board` - 游戏板引用
    ///
    /// # 返回
    /// true 表示会碰撞（方块不能移动到目标位置），false 表示可以移动
    pub fn check_collision(&self, dx: i32, dy: i32, board: &crate::resources::GameBoard) -> bool {
        use crate::constants::*;

        for (x, y) in self.blocks() {
            let new_x = x + dx;
            let new_y = y + dy;

            // 检查左右边界
            if new_x < 0 || new_x >= GRID_WIDTH as i32 {
                return true;
            }
            // 检查下边界
            if new_y >= GRID_HEIGHT as i32 {
                return true;
            }

            // 检查是否与已锁定的方块冲突
            if new_y >= 0 && !board.is_empty(new_x as usize, new_y as usize) {
                return true;
            }
        }
        false
    }

    /// 将相对坐标按旋转角度进行变换
    /// 使用旋转矩阵实现方块的旋转效果
    fn rotate_point(&self, x: i32, y: i32) -> (i32, i32) {
        match self.rotation % 4 {
            0 => (x, y),   // 0° 不旋转
            1 => (-y, x),  // 90° 顺时针旋转
            2 => (-x, -y), // 180° 旋转
            3 => (y, -x),  // 270° 顺时针旋转
            _ => unreachable!(),
        }
    }
}