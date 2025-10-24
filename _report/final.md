# 基于 Rust 的俄罗斯方块游戏开发项目报告

## 一、实验目的

-   **设计目标与待解决问题**
    本项目的主要目标是使用 Rust 编程语言及其生态流行的 Bevy 框架，从零开始完整实现一个功能齐全的经典俄罗斯方块游戏。要解决的核心问题包括：
    1.  学习并实践 Rust 语言的特性。
    2.  掌握 Bevy 框架的核心设计范式，实体组件系统（Entity-Component-System, ECS）。
    3.  实现俄罗斯方块的核心游戏逻辑，包括方块的生成、移动、旋转、下落、锁定、消行计分以及游戏结束判断。
    4.  构建一个完整的游戏循环，处理玩家输入、更新游戏状态并实时渲染游戏画面。

-   **用户体验与性能评估**
    项目旨在提供流畅、直观、无延迟的游戏体验。通过 Bevy 框架的数据驱动架构，确保了高效的渲染和逻辑处理，即使用“每帧重绘”的简单渲染策略，也能在目标硬件上实现高帧率运行，保证玩家操作的即时响应。

## 二、实验环境

-   **开发工具**：
    -   代码编辑器：Visual Studio Code / Zed
    -   构建工具与包管理器：Cargo 1.90.0
    -   编译器：rustc 1.90.0

-   **操作系统**：Windows 11 / macOS Tahoe 26.0.1

-   **设备与运行环境**：
    -   项目编译为本地可执行文件，直接运行。

-   **依赖与 SDK**：
    -   游戏框架：`bevy = "0.17.2"`
    -   随机数生成库：`rand = "0.9"`

## 三、实验原理

-   **核心概念与技术路线**
    本项目采用 Bevy 框架提供的 **实体组件系统 (ECS)** 作为核心架构。该架构将传统面向对象中的“对象”拆分为三个独立部分：
    1.  **实体 (Entity)**：一个唯一的ID，代表游戏中的一个“事物”，如一个方块格、一段UI文字。
    2.  **组件 (Component)**：附加在实体上的纯数据，用于描述其属性。例如 `Transform` (位置)、`Sprite` (外观)、`Block` (自定义标签)。
    3.  **系统 (System)**：处理带有特定组件集的实体的逻辑（函数）。例如，`rendering` 系统负责绘制所有带 `Sprite` 和 `Transform` 的实体。

    此外，项目还使用了 Bevy 的 **资源 (Resource)** 概念来管理全局唯一的游戏状态，如 `GameState` (游戏进程、分数) 和 `GameBoard` (棋盘数据)，实现了数据与逻辑的彻底分离。

-   **关键机制与实现思路**
    游戏的核心是**游戏循环 (Game Loop)**，由 Bevy 框架驱动。在 `main.rs` 中，我们注册了四个系统，它们在每帧按顺序执行：
    ```rust
    .add_systems(
        Update,
        (
            handle_player_input, // 处理玩家输入
            update_game_logic,   // 更新游戏逻辑
            render_game,         // 渲染游戏画面
            update_ui,           // 更新UI文本
        ),
    )
    ```
    这种顺序执行保证了：玩家输入先被处理 → 游戏逻辑基于最新输入更新 → 渲染反映最新状态。项目实现了经典的"Lock Delay"机制，方块触底后不会立即锁定，而是给予 0.2 秒的缓冲时间，允许玩家在最后一刻进行移动或旋转操作。

    **游戏主循环流程图:**

    ```mermaid
    graph TD
        A[开始] --> B{处理玩家输入};
        B --> C{更新游戏逻辑};
        C --> D[处理方块下落];
        D --> E[处理方块锁定];
        E --> F[处理消行与计分];
        F --> G[生成新方块];
        G --> H{渲染游戏画面};
        H --> I[绘制棋盘与已锁定方块];
        I --> J[绘制当前活动方块];
        J --> K[绘制UI文本];
        K --> B;
    ```
    此流程在每一帧重复执行，构成了游戏的实时交互体验。

## 四、实验步骤

### （一）需求分析与设计规划

-   **用户画像与场景**：面向所有年龄段的休闲游戏玩家。

-   **功能需求列表**：
    -   **P0 (必须)**：方块可左右移动、旋转、加速下落（软降）、瞬间下落（硬降）。
    -   **P0 (必须)**：方块在触底或接触其他方块后自动锁定。
    -   **P0 (必须)**：填满一行或多行后，行被消除并计分。
    -   **P0 (必须)**：游戏区域堆满后，游戏结束。
    -   **P1 (重要)**：显示当前分数、下一个方块预览。
    -   **P1 (重要)**：支持游戏暂停/继续、重新开始。
-   **交互与信息架构**：
    游戏主要在一个单页面内完成所有交互。信息结构简单，主要包括游戏主区域、分数显示区和下一个方块预览区。

-   **原型与线框**：
    设计基于经典的俄罗斯方块布局。游戏区为 10x20 网格，位于屏幕中央；分数在左上角；下一个方块预览在右上角；暂停/游戏结束提示居中显示。

### （二）项目创建与环境配置

-   **项目创建**：使用 Cargo 命令 `cargo new tetris-rs --bin` 创建一个新的二进制 Rust 项目。
-   **依赖配置**：在项目根目录的 `Cargo.toml` 文件中，添加 `[dependencies]` 部分，引入 `bevy` 和 `rand` 库。
    ```toml
    [dependencies]
    bevy = "0.17.2"
    rand = "0.9"
    ```
    执行 `cargo build` 即可自动下载并编译所有依赖。

-   **项目结构**：采用模块化设计，代码组织清晰：
    ```
    src/
    ├── main.rs           # 程序入口，配置 Bevy App
    ├── lib.rs            # 库根模块，导出所有公共接口
    ├── components.rs     # ECS 组件定义 (Block, UiText)
    ├── resources.rs      # 全局资源 (GameState, GameBoard, FallTimer)
    ├── tetromino.rs      # 方块类型与逻辑 (TetrominoType, ActivePiece)
    ├── constants.rs      # 游戏常量 (LOCK_DELAY, FALL_SPEED) 与工具函数
    └── systems/
        ├── mod.rs        # 系统模块导出
        ├── player_input.rs   # 玩家输入处理系统
        ├── rendering.rs      # 渲染与 UI 更新系统
        └── game.rs           # 核心游戏逻辑系统
    ```

### （三）界面布局设计

-   **整体布局思路**：
    游戏窗口设置为 800x700 像素 (`main.rs:12`)，采用 Bevy 的 2D 正交摄像头 `Camera2d`。游戏世界使用世界坐标系（原点在屏幕中心），通过自定义的 `grid_to_world` 函数将逻辑层面的 10x20 网格坐标精确转换为渲染层面的世界坐标。每个单元格大小为 30x30 像素。

-   **核心布局实现 (`constants.rs`)**：
    `grid_to_world(x, y)` 函数是布局的核心，它将网格坐标转换为 Bevy 的世界坐标系。网格坐标以左上角为原点 (0,0)，世界坐标以屏幕中心为原点。
    ```rust
    pub fn grid_to_world(x: i32, y: i32) -> (f32, f32) {
        // 计算游戏板左上角在世界坐标中的位置
        let offset_x = -(GRID_WIDTH as f32) * CELL_SIZE / 2.0;
        let offset_y = GRID_HEIGHT as f32 * CELL_SIZE / 2.0;
        // 转换为单元格中心点的世界坐标
        let world_x = offset_x + x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let world_y = offset_y - y as f32 * CELL_SIZE - CELL_SIZE / 2.0;
        (world_x, world_y)
    }
    ```

-   **组件摆放与层级 (`rendering.rs`)**：
    渲染系统采用分层绘制策略，通过 Z 坐标控制渲染顺序：
    ```rust
    // 背景网格 (z=0)
    Transform::from_xyz(world_x, world_y, 0.0)

    // 锁定的方块 (z=1)
    Transform::from_xyz(world_x, world_y, 1.0)

    // 当前下落方块 (z=2，最前面)
    Transform::from_xyz(world_x, world_y, 2.0)
    ```

    -   **UI 文本**：使用 Bevy 0.17 的新 UI 系统，通过 `Node` 组件实现绝对定位：
    ```rust
    commands.spawn((
        Text::new("Score: 0"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        UiText::Score,
    ));
    ```

### （四）界面样式设计

-   **样式与资源**：
    -   **颜色**：方块的颜色与 `TetrominoType` 枚举绑定，每种方块有独特的颜色标识 (`tetromino.rs:64-74`)：
    ```rust
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
    ```
    -   **大小**：所有游戏方块的大小由常量 `CELL_SIZE = 30.0` 统一控制。
    -   **字体**：UI 文本的字体大小和颜色在 `setup_game` 函数中创建实体时直接指定（分数：24px 白色，游戏结束：48px 红色，暂停：36px 黄色）。

-   **主题设置**：
    本项目未实现复杂的主题切换功能。游戏背景色和方块颜色是硬编码的，以还原经典游戏的视觉风格。背景网格使用深灰色 `Color::srgb(0.1, 0.1, 0.1)`，方块之间留有 2-4 像素的间隙以区分边界。

-   **方块形状定义** (`tetromino.rs:42-61`)：
    每种方块类型定义为相对坐标数组，描述四个格子相对于锚点的位置：
    ```rust
    TetrominoType::I => vec![(0, 0), (1, 0), (2, 0), (3, 0)],  // 一字形
    TetrominoType::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],  // 正方形
    TetrominoType::T => vec![(1, 0), (0, 1), (1, 1), (2, 1)],  // T字形
    // ... 其他形状
    ```
    这种设计使得旋转和碰撞检测的实现变得简洁统一。

### （五）交互设计实现

-   **事件绑定 (`player_input.rs`)**：
    在 `handle_player_input` 系统中，通过查询 `Res<ButtonInput<KeyCode>>` 资源来获取键盘输入，支持方向键和 WASD 双键位操作：
    ```rust
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        new_piece.x -= 1;  // 左移
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        new_piece.x += 1;  // 右移
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        new_piece.y += 1;  // 软降（加速下落）
    } else if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        new_piece.rotation = (new_piece.rotation + 1) % 4;  // 旋转
    } else if keyboard.just_pressed(KeyCode::Space) {
        // 硬降（直接落地）
        while !new_piece.check_collision(0, 1, &board) {
            new_piece.y += 1;
        }
    }
    // 碰撞检测：只有新位置有效时才更新
    if !new_piece.check_collision(0, 0, &board) {
        game_state.current_piece = Some(new_piece);
    }
    ```
    此外还支持：ESC 暂停/继续，Q 退出游戏，游戏结束后 Space 重新开始。

-   **页面导航（游戏状态转换）**：
    游戏没有多页面导航，而是通过改变 `GameState` 资源中的布尔标志 `paused` 和 `game_over` 来实现状态转换。
    -   按下 `ESC` 键会切换 `paused` 的值。
    -   当方块堆叠至顶部时，`game_over` 被设为 `true`。
    `update_ui` 和 `update_game_logic` 等系统会根据这些标志决定自身的行为（如暂停时逻辑停止，游戏结束时显示提示信息）。

-   **状态管理**：
    `GameState` 和 `GameBoard` 这两个全局资源是唯一的数据源 (Single Source of Truth)。所有系统都通过只读 (`Res<T>`) 或可写 (`ResMut<T>`) 的方式访问这些资源。

    项目实现了 **7-Bag 随机系统** (`resources.rs:108-131`)，确保每 7 个方块中包含所有 7 种类型各一个，避免完全随机可能导致的长时间不出现某种方块：
    ```rust
    pub fn next_from_bag(&mut self) -> TetrominoType {
        // 如果袋子空了，重新填充所有 7 种方块并打乱
        if self.bag.is_empty() {
            self.bag = vec![
                TetrominoType::I, TetrominoType::O, TetrominoType::T,
                TetrominoType::S, TetrominoType::Z, TetrominoType::J,
                TetrominoType::L,
            ];
            self.bag.shuffle(&mut rand::rng());
        }
        // 从袋子中取出一个方块
        self.bag.pop().unwrap()
    }
    ```

### （六）核心游戏逻辑实现

游戏逻辑系统 `update_game_logic` (`game.rs`) 在每帧按顺序执行四个关键阶段：

1. **方块下落** (`handle_fall`)：使用 `FallTimer` 计时器，周期性（每 0.8 秒）让当前方块下移一格。
    ```rust
    if timer.timer.just_finished() {
        if let Some(ref mut piece) = game_state.current_piece {
            if !piece.check_collision(0, 1, board) {
                piece.y += 1;  // 下移一格
            }
        }
    }
    // constants.rs 中定义：pub const FALL_SPEED: f32 = 0.8;
    ```

2. **方块锁定** (`handle_lock`)：实现了经典的落地延迟机制，方块触底后给予 0.2 秒的缓冲时间。
    ```rust
    // 检查方块是否触底
    let is_grounded = if let Some(ref piece) = game_state.current_piece {
        piece.check_collision(0, 1, board)
    } else { false };

    if is_grounded {
        // 累加锁定计时器
        let elapsed = game_state.lock_timer.get_or_insert(0.0);
        *elapsed += time.delta_secs();

        // 计时器超过延迟时间，执行锁定
        if *elapsed >= LOCK_DELAY {
            if let Some(piece) = game_state.current_piece.take() {
                for (x, y) in piece.blocks() {
                    if y < 0 { game_state.game_over = true; return; }
                    board.set_cell(x as usize, y as usize, Some(color));
                }
            }
            game_state.lock_timer = None;
        }
    } else {
        // 方块未触底，重置计时器
        game_state.lock_timer = None;
    }
    ```
    玩家在触底缓冲时间内移动方块，若移动后脱离触底状态，计时器会重置，允许继续操作。

3. **消行与计分** (`handle_clear_lines`)：检查并清除满行，上方方块下移。
    ```rust
    for y in (0..GRID_HEIGHT).rev() {
        if board.is_line_full(y) {
            board.clear_line(y);
            lines_cleared += 1;
            board.shift_lines_down(y);  // 上方所有行下移
        }
    }
    if lines_cleared > 0 {
        game_state.add_score(lines_cleared);  // 消1-4行分别得 100/300/500/800 分
    }
    ```

4. **生成新方块** (`handle_spawn`)：当没有活跃方块时，从 7-Bag 系统取出下一个方块。

**碰撞检测** (`tetromino.rs:118-149`)：检查方块的每个格子是否超出边界或与已锁定方块重叠。

**旋转系统** (`tetromino.rs:153-161`)：使用旋转矩阵实现顺时针旋转。
```rust
fn rotate_point(&self, x: i32, y: i32) -> (i32, i32) {
    match self.rotation % 4 {
        0 => (x, y),    // 0°
        1 => (-y, x),   // 90°
        2 => (-x, -y),  // 180°
        3 => (y, -x),   // 270°
        _ => unreachable!(),
    }
}
```

### （七）测试与调试

-   **功能测试**：通过手动操作进行测试，确保所有功能符合预期。
    | 测试用例 | 操作步骤 | 预期结果 |
    | :--- | :--- | :--- |
    | 方块移动 | 按下左右方向键 | 方块在边界内正常左右移动一格 |
    | 方块旋转 | 按下上方向键 | 方块顺时针旋转90度，且不会穿墙 |
    | 硬降 | 按下空格键 | 方块立即下落并锁定在最底部 |
    | 软降 | 按下下方向键/S | 方块加速下落一格 |
    | 落地延迟 | 方块触底后立即左右移动 | 方块不会立即锁定，可以在0.2秒内调整位置 |
    | 消行与计分 | 填满一行 | 该行被消除，上方方块下落，分数增加100分 |
    | 同时消多行 | 填满2/3/4行 | 分别得到300/500/800分 |
    | 游戏结束 | 方块堆叠到顶部 | 屏幕显示 "GAME OVER"，游戏逻辑停止 |
    | 暂停/继续 | 按下ESC键 | 游戏画面静止，显示暂停菜单/恢复游戏 |
    | 退出游戏 | 按下Q键 | 游戏窗口关闭 |
    | 重新开始 | 游戏结束后按空格 | 游戏重置，分数归零，重新开始 |

## 五、实验结果

-   **目标对照**：
    项目成功完成了预设的全部 P0 和 P1 级功能需求：

    | 需求等级 | 功能 | 实现状态 | 代码位置 |
    | :---: | :--- | :---: | :--- |
    | P0 | 方块移动、旋转、软降、硬降 | ✓ | `player_input.rs:54-73` |
    | P0 | 方块自动下落与锁定 | ✓ | `game.rs:54-85` |
    | P0 | 落地延迟（Lock Delay） | ✓ | `game.rs:54-85, resources.rs:80` |
    | P0 | 消行计分 | ✓ | `game.rs:87-101` |
    | P0 | 游戏结束判断 | ✓ | `game.rs:72-75, 112-115` |
    | P1 | 分数显示 | ✓ | `main.rs:43-57` |
    | P1 | 下一个方块预览 | ✓ | `rendering.rs:79-106` |
    | P1 | 暂停/继续 | ✓ | `player_input.rs:23-25` |
    | P1 | 重新开始 | ✓ | `player_input.rs:37-39` |

-   **界面与功能展示**：
    -   **游戏进行中**：10x20 游戏区位于中心，左上角显示当前分数，右上角预览下一个方块（缩小至 70% 显示）。
    -   **暂停界面**：显示 "PAUSED" 提示及操作说明（ESC 继续，Q 退出）。
    -   **游戏结束**：显示 "GAME OVER" 和 "Press SPACE" 提示。

-   **性能表现**：
    在目标硬件上运行流畅，帧率稳定在 60 FPS 以上。即使采用每帧重绘策略（约 200 个实体的创建与销毁），性能开销仍在可接受范围内。

## 六、问题与反思

-   **主要问题与成因分析**：
    1.  **渲染效率**：`render_game` 系统采用“每帧销毁全部方块再全部重绘”的策略。虽然对于此项目规模性能足够，但对于实体数量庞大的游戏，这将是严重的性能瓶颈。
    2.  **旋转系统过于简单**：`rotate_point` 函数只实现了简单的数学旋转，未实现官方俄罗斯方块规则中的“墙踢 (Wall Kick)”机制，导致方块在贴近墙壁或其它方块时旋转体验不佳。

-   **解决方案与权衡**：
    1.  **渲染优化**：更优的方案是在游戏启动时创建所有需要的实体，渲染系统每帧只更新它们的 `Transform` (位置) 和 `Sprite` (颜色/可见性) 组件，而非销毁和重建。本项目为简化逻辑，选择了实现更简单的重绘方案。
    ```rust
    // 当前实现：每帧清除并重建所有 Block 实体
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    ```

    2.  **高级旋转**：实现"墙踢 (Wall Kick)"需要引入一套复杂的偏移检测规则（如 SRS 旋转系统），在旋转发生碰撞时，按预定规则尝试几个偏移位置。考虑到项目复杂度，本次实现中省略了该高级功能。

    3.  **落地延迟实现**：项目实现了经典的 Lock Delay 机制，通过在 `GameState` 中添加 `lock_timer: Option<f32>` 字段来追踪方块触底时间。当方块触底时启动计时器累加时间，计时器超过设定阈值（0.2秒）才真正锁定方块。如果玩家在延迟期间移动方块使其脱离触底状态，计时器会重置，允许继续操作。这种设计符合现代俄罗斯方块的游戏设计理念，提供了更好的容错空间，使得玩家能够在最后一刻调整方块位置。

-   **待优化项与后续计划**：
    -   增加音效和背景音乐。
    -   引入关卡系统，随分数增加，方块下落速度加快。
    -   实现一个更完善的主菜单和设置界面。

## 七、总结与展望

-   **收获与经验**：
    通过本项目，深刻理解了以下技术要点：

    1. **ECS 架构**：数据（组件）和逻辑（系统）彻底分离。例如，`Block` 组件仅作标记，实际渲染逻辑在 `render_game` 系统中实现。

    2. **Rust 所有权系统**：通过 `Res<T>` 和 `ResMut<T>` 在编译期保证资源访问安全，避免数据竞争：
    ```rust
    pub fn handle_player_input(
        keyboard: Res<ButtonInput<KeyCode>>,    // 只读
        mut game_state: ResMut<GameState>,      // 可写
        mut board: ResMut<GameBoard>,           // 可写
    )
    ```

    3. **游戏设计模式**：实现了 7-Bag 随机系统、分层渲染、碰撞检测等经典游戏机制。

    4. **Bevy 0.17 新特性**：掌握了新版 UI 系统（`Text::new`、`Node`）和资源管理模式。

-   **可扩展方向**：
    当前架构具有良好的可扩展性。可以方便地添加新的组件和系统来实现新功能，例如，可以添加一个 `Level` 资源和一个 `update_speed` 系统来轻松实现关卡难度递增功能。代码逻辑清晰，也为后续重构和性能优化打下了良好基础。

## 八、参考文献

-   [1] The Bevy Book. https://bevy.org/learn/quick-start/introduction/
-   [2] The Rust Programming Language. https://doc.rust-lang.org/book/

## 九、附录

-   **源码仓库**：`https://github.com/jq-jz/tetris-rs`

-   **运行步骤**：
    1.  安装 Rust 工具链（建议版本 1.90.0 或更高）。
    2.  克隆源码仓库：`git clone https://github.com/jq-jz/tetris-rs`
    3.  进入项目目录：`cd tetris-rs`
    4.  运行游戏：`cargo run`（首次运行会自动下载并编译依赖）
    5.  发布构建：`cargo build --release`（优化编译，可执行文件位于 `target/release/tetris`）

-   **控制键位汇总**：
    | 操作 | 按键 |
    | :--- | :--- |
    | 左移 | ← / A |
    | 右移 | → / D |
    | 旋转 | ↑ / W |
    | 软降（加速下落） | ↓ / S |
    | 硬降（直接落地） | Space |
    | 暂停/继续 | ESC |
    | 退出游戏 | Q |
    | 重新开始（游戏结束后） | Space |
