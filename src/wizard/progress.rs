use super::WizardState;
use crate::utils::permissions;
use crate::wizard::error::WizardError;
use std::path::PathBuf;

/// 向导进度持久化
pub struct ProgressManager {
    state_file: PathBuf,
}

impl ProgressManager {
    /// 创建新的进度管理器
    pub fn new() -> anyhow::Result<Self> {
        let cache_dir = dirs::cache_dir().unwrap().join("agentswitch");

        // 确保缓存目录存在
        permissions::create_directory_with_700_perms(&cache_dir)?;

        let state_file = cache_dir.join("wizard_state.toml");

        Ok(Self { state_file })
    }

    /// 保存向导状态
    pub fn save_state(&self, state: &WizardState) -> Result<(), WizardError> {
        state.save(&self.state_file)
    }

    /// 加载向导状态
    pub fn load_state(&self) -> Result<Option<WizardState>, WizardError> {
        WizardState::load(&self.state_file)
    }

    /// 删除保存的状态
    pub fn clear_state(&self) -> Result<(), WizardError> {
        if self.state_file.exists() {
            std::fs::remove_file(&self.state_file)?;
        }
        Ok(())
    }

    /// 获取状态文件路径
    pub fn state_file_path(&self) -> &PathBuf {
        &self.state_file
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
