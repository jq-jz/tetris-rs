// 游戏系统模块
// 包含游戏逻辑、输入处理和渲染系统

mod game; // 游戏核心逻辑（下落、碰撞、消行）
mod player_input; // 玩家输入处理
mod rendering; // 游戏渲染和 UI 更新

// 公共导出接口
pub use game::update_game_logic;
pub use player_input::handle_player_input;
pub use rendering::{render_game, update_ui};
