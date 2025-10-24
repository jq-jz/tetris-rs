use bevy::prelude::*;
use tetris::*;

/// 游戏入口函数
/// 初始化 Bevy 应用程序，配置窗口、资源和系统
fn main() {
    App::new()
        // 添加默认插件，并配置窗口参数
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                resolution: (800, 700).into(),
                ..default()
            }),
            ..default()
        }))
        // 初始化游戏资源
        .init_resource::<GameBoard>() // 游戏板资源
        .init_resource::<GameState>() // 游戏状态资源
        .init_resource::<FallTimer>() // 方块掉落计时器
        // 添加启动系统（只执行一次）
        .add_systems(Startup, setup_game)
        // 添加更新系统
        .add_systems(
            Update,
            (
                handle_player_input, // 处理玩家输入（键盘）
                update_game_logic,   // 更新游戏逻辑（下落、碰撞、消行等）
                render_game,         // 渲染游戏（绘制方块、方块预览）
                update_ui,           // 更新 UI（分数、游戏状态提示）
            ),
        )
        .run();
}

/// 初始化游戏场景
/// 创建摄像头和 UI 文本元素（分数、游戏结束、暂停提示）
fn setup_game(mut commands: Commands) {
    // 创建 2D 摄像头
    commands.spawn(Camera2d);

    // 创建分数显示文本（左上角）
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

    // 创建游戏结束提示文本（中心）
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)), // 红色
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(250.0),
            ..default()
        },
        UiText::GameOver,
    ));

    // 创建暂停提示文本（中心上方）
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.5)), // 黄色
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(250.0),
            left: Val::Px(280.0),
            ..default()
        },
        UiText::Pause,
    ));
}
