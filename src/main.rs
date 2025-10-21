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
        .add_systems(Startup, setup_game)
        .add_systems(Update, render_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
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
}
