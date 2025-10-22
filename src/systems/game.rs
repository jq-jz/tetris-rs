// 游戏核心逻辑系统
// 处理方块下落、碰撞、锁定、消行等核心游戏逻辑

use crate::constants::GRID_HEIGHT;
use crate::resources::{FallTimer, GameBoard, GameState};
use crate::tetromino::{ActivePiece, TetrominoType};
use bevy::prelude::*;

/// 游戏逻辑更新系统
/// 每帧调用，按顺序处理：方块下落 -> 方块锁定 -> 消行 -> 生成新方块
pub fn update_game_logic(
    time: Res<Time>,
    mut timer: ResMut<FallTimer>,
    mut game_state: ResMut<GameState>,
    mut board: ResMut<GameBoard>,
) {
    // 游戏结束或暂停时不更新逻辑
    if game_state.game_over || game_state.paused {
        return;
    }

    // 按顺序执行各个游戏逻辑阶段
    handle_fall(&time, &mut timer, &mut game_state, &board);
    handle_lock(&mut game_state, &mut board);
    handle_clear_lines(&mut game_state, &mut board);
    handle_spawn(&mut game_state, &board);
}

/// 处理方块下落
/// 根据计时器周期性地将当前方块下移一格
fn handle_fall(
    time: &Res<Time>,
    timer: &mut ResMut<FallTimer>,
    game_state: &mut ResMut<GameState>,
    board: &GameBoard,
) {
    // 更新计时器
    timer.timer.tick(time.delta());

    // 计时器触发时（到达掉落周期）
    if timer.timer.just_finished() {
        if let Some(ref mut piece) = game_state.current_piece {
            // 检查下移一格是否会碰撞
            if !piece.check_collision(0, 1, board) {
                // 没有碰撞，方块下移一格
                piece.y += 1;
            }
        }
    }
}

/// 处理方块锁定
/// 当方块无法继续下落时，将其固定在游戏板上
fn handle_lock(game_state: &mut ResMut<GameState>, board: &mut ResMut<GameBoard>) {
    // 检查当前方块是否会在下一步碰撞（无法继续下落）
    let should_lock = if let Some(ref piece) = game_state.current_piece {
        piece.check_collision(0, 1, board)
    } else {
        false
    };

    if should_lock {
        // 取出当前方块
        if let Some(piece) = game_state.current_piece.take() {
            let color = piece.tetromino_type.color();
            // 将方块的所有格子固定到游戏板上
            for (x, y) in piece.blocks() {
                // 如果方块的格子出现在屏幕顶部以上，游戏结束
                if y < 0 {
                    game_state.game_over = true;
                    return;
                }
                // 在游戏板上标记该格子已被占用
                board.set_cell(x as usize, y as usize, Some(color));
            }
        }
    }
}

/// 处理消行
/// 检查并清除满行，并将上面的行下移
fn handle_clear_lines(game_state: &mut ResMut<GameState>, board: &mut ResMut<GameBoard>) {
    let mut lines_cleared = 0;

    // 从下向上扫描所有行
    for y in (0..GRID_HEIGHT).rev() {
        // 检查该行是否已满
        if board.is_line_full(y) {
            // 清空该行
            board.clear_line(y);
            lines_cleared += 1;
            // 将上面的所有行下移一格
            board.shift_lines_down(y);
        }
    }

    // 如果有消行，根据消行数增加分数
    if lines_cleared > 0 {
        game_state.add_score(lines_cleared);
    }
}

/// 处理新方块生成
/// 当没有活跃方块时，生成下一个方块
fn handle_spawn(game_state: &mut ResMut<GameState>, board: &GameBoard) {
    // 当前没有活跃方块时
    if game_state.current_piece.is_none() {
        // 创建新方块（使用预先生成的下一个方块）
        let new_piece = ActivePiece::new(game_state.next_piece);

        // 检查新方块的生成位置是否已被占用（碎片堆积到顶部）
        if new_piece.check_collision(0, 0, board) {
            // 生成位置被占用，游戏结束
            game_state.game_over = true;
            return;
        }

        // 方块成为当前活跃方块
        game_state.current_piece = Some(new_piece);
        // 预先生成下一个随机方块
        game_state.next_piece = TetrominoType::random();
    }
}
