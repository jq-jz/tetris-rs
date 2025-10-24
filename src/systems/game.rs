// 游戏核心逻辑系统
// 处理方块下落、碰撞、锁定、消行等核心游戏逻辑

use crate::constants::{GRID_HEIGHT, LOCK_DELAY};
use crate::resources::{FallTimer, GameBoard, GameState};
use crate::tetromino::ActivePiece;
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
    handle_lock(&time, &mut game_state, &mut board);
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
/// 当方块无法继续下落时，启动延迟计时器，计时结束后将其固定在游戏板上
fn handle_lock(time: &Res<Time>, game_state: &mut ResMut<GameState>, board: &mut ResMut<GameBoard>) {
    // 检查当前方块是否触底（下一步会碰撞）
    let is_grounded = if let Some(ref piece) = game_state.current_piece {
        piece.check_collision(0, 1, board)
    } else {
        false
    };

    if is_grounded {
        // 方块触底，累加锁定计时器
        let elapsed = game_state.lock_timer.get_or_insert(0.0);
        *elapsed += time.delta_secs();

        // 计时器超过延迟时间，执行锁定
        if *elapsed >= LOCK_DELAY {
            if let Some(piece) = game_state.current_piece.take() {
                let color = piece.tetromino_type.color();
                for (x, y) in piece.blocks() {
                    if y < 0 {
                        game_state.game_over = true;
                        return;
                    }
                    board.set_cell(x as usize, y as usize, Some(color));
                }
            }
            game_state.lock_timer = None;
        }
    } else {
        // 方块未触底，重置锁定计时器
        game_state.lock_timer = None;
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
        // 使用 7-Bag 系统生成下一个方块
        game_state.next_piece = game_state.next_from_bag();
    }
}
