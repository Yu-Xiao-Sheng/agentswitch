//! 预设管理模块
//!
//! 本模块提供配置预设的管理功能，包括：
//! - 预设的创建、读取、更新、删除（CRUD）
//! - 预设的应用和回滚
//! - 预设的验证

pub mod apply;
pub mod error;
pub mod preset;
pub mod store;
pub mod validator;

// 重新导出主要类型
pub use apply::PresetAppplier;
pub use error::PresetError;
pub use preset::Preset;
pub use store::PresetCollection;
pub use store::PresetStore;
// 重新导出验证函数
pub use validator::{validate_preset, validate_preset_agents};
