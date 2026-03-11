//! 批量操作模块
//!
//! 本模块提供批量操作功能，包括：
//! - 批量切换工具配置
//! - 批量验证工具状态
//! - 批量获取工具状态

pub mod status;
pub mod switch;
pub mod validate;

// 重新导出主要类型
// 重新导出批量操作函数
pub use switch::batch_switch_agents;
pub use validate::batch_validate_agents;
