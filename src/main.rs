use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod tetromino;

use components::*;
use constants::*;
use resources::*;
use tetromino::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                resolution: (800, 700).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameBoard>()
        .init_resource::<GameState>()
        .init_resource::<FallTimer>()
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                handle_player_input,
                update_game_logic,
                render_game,
                update_ui,
            )
                .chain(),
        )
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        UiText::Score,
    ));
}

fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    board: Res<GameBoard>,
) {
    if let Some(piece) = game_state.current_piece {
        let mut new_piece = piece;

        if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
            new_piece.x -= 1;
        } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD)
        {
            new_piece.x += 1;
        } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS)
        {
            new_piece.y += 1;
        } else if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
            new_piece.rotation = (new_piece.rotation + 1) % 4;
        } else if keyboard.just_pressed(KeyCode::Space) {
            while !new_piece.check_collision(0, 1, &board) {
                new_piece.y += 1;
            }
        } else {
            return;
        }

        if !new_piece.check_collision(0, 0, &board) {
            game_state.current_piece = Some(new_piece);
        }
    }
}

fn update_game_logic(
    time: Res<Time>,
    mut timer: ResMut<FallTimer>,
    mut game_state: ResMut<GameState>,
    mut board: ResMut<GameBoard>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        if let Some(ref mut piece) = game_state.current_piece {
            if !piece.check_collision(0, 1, &board) {
                piece.y += 1;
            }
        }
    }

    let should_lock = if let Some(ref piece) = game_state.current_piece {
        piece.check_collision(0, 1, &board)
    } else {
        false
    };

    if should_lock {
        if let Some(piece) = game_state.current_piece.take() {
            let color = piece.tetromino_type.color();
            for (x, y) in piece.blocks() {
                if y >= 0 && y < GRID_HEIGHT as i32 && x >= 0 && x < GRID_WIDTH as i32 {
                    board.set_cell(x as usize, y as usize, Some(color));
                }
            }
        }
    }

    let mut lines_cleared = 0;
    for y in (0..GRID_HEIGHT).rev() {
        if board.is_line_full(y) {
            board.clear_line(y);
            lines_cleared += 1;
            board.shift_lines_down(y);
        }
    }

    if lines_cleared > 0 {
        game_state.add_score(lines_cleared);
    }

    if game_state.current_piece.is_none() {
        let new_piece = ActivePiece::new(game_state.next_piece);
        game_state.current_piece = Some(new_piece);
        game_state.next_piece = TetrominoType::random();
    }
}

fn render_game(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
    game_state: Res<GameState>,
    board: Res<GameBoard>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let (world_x, world_y) = grid_to_world(x as i32, y as i32);

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.1, 0.1, 0.1),
                    custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                Block,
            ));

            if let Some(color) = board.cells[y][x] {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 1.0),
                    Block,
                ));
            }
        }
    }

    if let Some(ref piece) = game_state.current_piece {
        let color = piece.tetromino_type.color();
        for (x, y) in piece.blocks() {
            if y >= 0 && y < GRID_HEIGHT as i32 && x >= 0 && x < GRID_WIDTH as i32 {
                let (world_x, world_y) = grid_to_world(x, y);

                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 2.0),
                    Block,
                ));
            }
        }
    }

    let offset_x = -(GRID_WIDTH as f32) * CELL_SIZE / 2.0;
    let offset_y = GRID_HEIGHT as f32 * CELL_SIZE / 2.0;
    let preview_offset_x = offset_x + GRID_WIDTH as f32 * CELL_SIZE + 60.0;
    let preview_offset_y = offset_y - 100.0;

    let next_shape = game_state.next_piece.shape();
    let next_color = game_state.next_piece.color();

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

fn update_ui(game_state: Res<GameState>, mut query: Query<(&mut Text, &UiText)>) {
    for (mut text, ui_type) in query.iter_mut() {
        **text = match ui_type {
            UiText::Score => format!("Score: {}", game_state.score),
        };
    }
}
