// 玩家输入处理系统
// 处理键盘输入，实现方块的移动、旋转和游戏控制

use crate::resources::{GameBoard, GameState};
use bevy::prelude::*;

/// 玩家输入处理系统
/// 处理所有键盘输入，包括：
/// - 方块移动（方向键/WASD）
/// - 方块旋转（上箭头/W）
/// - 快速下落（下箭头/S）
/// - 直接落地（空格）
/// - 暂停/继续（ESC）
/// - 游戏结束后重新开始（空格）
/// - 退出游戏（Q）
pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut board: ResMut<GameBoard>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    // 处理暂停/继续（ESC 键）
    if keyboard.just_pressed(KeyCode::Escape) && !game_state.game_over {
        game_state.paused = !game_state.paused;
        return;
    }

    // 处理退出游戏（Q 键）
    if keyboard.just_pressed(KeyCode::KeyQ) {
        app_exit_events.write(AppExit::Success);
        return;
    }

    // 游戏结束时的处理
    if game_state.game_over {
        // 按空格重新开始游戏
        if keyboard.just_pressed(KeyCode::Space) {
            game_state.reset();
            *board = GameBoard::default();
        }
        return;
    }

    // 游戏暂停时不处理方块操作
    if game_state.paused {
        return;
    }

    // 处理活跃方块的操作
    if let Some(piece) = game_state.current_piece {
        let mut new_piece = piece;

        // 判断按下了哪个移动/操作键
        if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
            // 左移：X 坐标减 1
            new_piece.x -= 1;
        } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD)
        {
            // 右移：X 坐标加 1
            new_piece.x += 1;
        } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS)
        {
            // 快速下落：Y 坐标加 1
            new_piece.y += 1;
        } else if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
            // 旋转：旋转角度加 1（0-3 循环）
            new_piece.rotation = (new_piece.rotation + 1) % 4;
        } else if keyboard.just_pressed(KeyCode::Space) {
            // 直接落地：不断下移直到碰撞
            while !new_piece.check_collision(0, 1, &board) {
                new_piece.y += 1;
            }
        } else {
            // 没有按下任何操作键，直接返回
            return;
        }

        // 检查新位置是否有效（不与已固定方块碰撞）
        if !new_piece.check_collision(0, 0, &board) {
            // 有效，更新当前方块
            game_state.current_piece = Some(new_piece);
        }
        // 如果碰撞，则不更新方块位置（玩家操作被拒绝）
    }
}
