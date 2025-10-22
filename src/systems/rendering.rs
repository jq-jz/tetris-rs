// 游戏渲染和 UI 更新系统
// 处理游戏画面的绘制和文本 UI 的更新

use crate::components::{Block, UiText};
use crate::constants::*;
use crate::resources::{GameBoard, GameState};
use bevy::prelude::*;

/// 游戏渲染系统
/// 每帧清除上一帧的所有方块实体，然后重新绘制：
/// 1. 游戏板背景网格
/// 2. 已锁定的方块
/// 3. 当前下落的方块
/// 4. 下一个方块的预览
pub fn render_game(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
    game_state: Res<GameState>,
    board: Res<GameBoard>,
) {
    // 清除所有上一帧的 Block 实体
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // 绘制游戏板（网格和已锁定的方块）
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            // 将网格坐标转换为世界坐标
            let (world_x, world_y) = grid_to_world(x as i32, y as i32);

            // 绘制背景网格单元格（深灰色）
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.1, 0.1, 0.1),
                    custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0), // z=0 在最后面
                Block,
            ));

            // 如果该格子有已锁定的方块，绘制该方块
            if let Some(color) = board.cells[y][x] {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 1.0), // z=1 在网格上方
                    Block,
                ));
            }
        }
    }

    // 绘制当前下落的活跃方块
    if let Some(ref piece) = game_state.current_piece {
        let color = piece.tetromino_type.color();
        for (x, y) in piece.blocks() {
            // 只绘制在游戏板可见范围内的方块
            if y >= 0 && y < GRID_HEIGHT as i32 && x >= 0 && x < GRID_WIDTH as i32 {
                let (world_x, world_y) = grid_to_world(x, y);

                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 2.0), // z=2 在最前面
                    Block,
                ));
            }
        }
    }

    // 绘制下一个方块的预览（游戏运行中显示）
    if !game_state.game_over {
        // 计算预览区域的位置（在游戏板右侧）
        let offset_x = -(GRID_WIDTH as f32) * CELL_SIZE / 2.0;
        let offset_y = GRID_HEIGHT as f32 * CELL_SIZE / 2.0;
        let preview_offset_x = offset_x + GRID_WIDTH as f32 * CELL_SIZE + 60.0;
        let preview_offset_y = offset_y - 100.0;

        // 获取下一个方块的形状和颜色
        let next_shape = game_state.next_piece.shape();
        let next_color = game_state.next_piece.color();

        // 绘制下一个方块的每个格子（缩小显示）
        for (dx, dy) in next_shape {
            let world_x = preview_offset_x + dx as f32 * CELL_SIZE * 0.7;
            let world_y = preview_offset_y - dy as f32 * CELL_SIZE * 0.7;

            commands.spawn((
                Sprite {
                    color: next_color,
                    custom_size: Some(Vec2::new(CELL_SIZE * 0.7 - 4.0, CELL_SIZE * 0.7 - 4.0)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 1.0),
                Block,
            ));
        }
    }
}

/// UI 文本更新系统
/// 更新分数显示、游戏结束提示和暂停提示的文本内容
pub fn update_ui(game_state: Res<GameState>, mut query: Query<(&mut Text, &UiText)>) {
    for (mut text, ui_type) in query.iter_mut() {
        // 根据 UI 文本类型更新对应的文本内容
        **text = match ui_type {
            // 分数显示：当前分数
            UiText::Score => format!("Score: {}", game_state.score),
            // 游戏结束提示：仅在游戏结束时显示
            UiText::GameOver => {
                if game_state.game_over {
                    "GAME OVER\nPress SPACE".to_string()
                } else {
                    String::new()
                }
            }
            // 暂停提示：仅在游戏暂停时显示，附带操作说明
            UiText::Pause => {
                if game_state.paused {
                    "PAUSED\nESC: Resume\nQ: Quit".to_string()
                } else {
                    String::new()
                }
            }
        };
    }
}
