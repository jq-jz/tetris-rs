// 组件模块：定义游戏中使用的 Bevy 组件

use bevy::prelude::*;

/// 游戏方块组件
/// 用于标记游戏板中的方块实体（包括背景网格和已锁定的方块）
#[derive(Component)]
pub struct Block;

/// UI 文本类型枚举组件
/// 用于区分不同的 UI 文本元素（分数、游戏结束、暂停提示）
#[derive(Component, Clone, Copy, PartialEq)]
pub enum UiText {
    /// 分数显示文本
    Score,
    /// 游戏结束提示文本
    GameOver,
    /// 暂停提示文本
    Pause,
}
