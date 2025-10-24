# tetris-rs

基于 [Bevy](https://bevyengine.org/) 游戏引擎实现的经典俄罗斯方块游戏。

## 快速开始

### 前置要求

- Rust 1.70+
- Cargo

### 安装与运行

```bash
# 克隆项目
git clone https://github.com/jq-jz/tetris-rs
cd tetris-rs

# 运行游戏
cargo run
```

## 操作说明

### 游戏控制

| 操作                   | 按键  |
| :--------------------- | :---- |
| 左移                   | ← / A |
| 右移                   | → / D |
| 旋转                   | ↑ / W |
| 软降（加速下落）       | ↓ / S |
| 硬降（直接落地）       | Space |
| 暂停/继续              | ESC   |
| 退出游戏               | Q     |
| 重新开始（游戏结束后） | Space |

## 项目结构

```
tetris-rs/
├── src/
│   ├── main.rs           # 游戏入口和初始化
│   ├── lib.rs            # 库模块导出
│   ├── components.rs     # ECS 组件定义
│   ├── resources.rs      # 游戏资源（游戏板、游戏状态等）
│   ├── tetromino.rs      # 俄罗斯方块类型和逻辑
│   ├── constants.rs      # 游戏常量配置
│   └── systems/
│       ├── mod.rs
│       ├── player_input.rs  # 玩家输入处理
│       ├── game.rs          # 游戏逻辑（下落、碰撞、消行）
│       └── rendering.rs     # 渲染系统
└── Cargo.toml
```

## 开发

```bash
# 开发模式运行
cargo run

# 构建发布版本
cargo build --release
```
