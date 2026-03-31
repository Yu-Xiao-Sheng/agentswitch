//! 自动更新检测模块

pub mod checker;

pub use checker::{check_for_update, check_on_startup, display_update_notification};
// UpdateInfo 通过 check_for_update 返回值获取，无需直接导出
