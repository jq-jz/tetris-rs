// 游戏常量定义

/// 游戏板宽度（列数）
pub const GRID_WIDTH: usize = 10;
/// 游戏板高度（行数）
pub const GRID_HEIGHT: usize = 20;
/// 单个方块的像素大小
pub const CELL_SIZE: f32 = 30.0;
/// 方块掉落速度（秒）
pub const FALL_SPEED: f32 = 0.8;

// 消行计分表
/// 消一行的分数
pub const SCORE_1_LINE: u32 = 100;
/// 消两行的分数
pub const SCORE_2_LINES: u32 = 300;
/// 消三行的分数
pub const SCORE_3_LINES: u32 = 500;
/// 消四行（俄罗斯方块）的分数
pub const SCORE_4_LINES: u32 = 800;

/// 将网格坐标转换为世界坐标
///
/// # 参数
/// * `x` - 网格列坐标（0 到 GRID_WIDTH-1）
/// * `y` - 网格行坐标（0 到 GRID_HEIGHT-1）
///
/// # 返回
/// (世界坐标 x, 世界坐标 y)
///
/// 网格坐标以左上角为原点，世界坐标以屏幕中心为原点
pub fn grid_to_world(x: i32, y: i32) -> (f32, f32) {
    // 计算网格显示的左边界和上边界偏移
    let offset_x = -(GRID_WIDTH as f32) * CELL_SIZE / 2.0;
    let offset_y = GRID_HEIGHT as f32 * CELL_SIZE / 2.0;
    // 将网格坐标转换为世界坐标（加上单元格尺寸的一半以居中显示）
    let world_x = offset_x + x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
    let world_y = offset_y - y as f32 * CELL_SIZE - CELL_SIZE / 2.0;
    (world_x, world_y)
}
