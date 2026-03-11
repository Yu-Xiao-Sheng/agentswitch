use crate::wizard::error::WizardError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 向导状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardState {
    /// 当前步骤索引
    pub current_step: usize,

    /// 已完成的步骤索引列表
    pub completed_steps: Vec<usize>,

    /// 已收集的数据（字段名 -> 值）
    pub data: HashMap<String, String>,

    /// 状态保存时间
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// 向导类型（首次配置、添加模型等）
    pub wizard_type: WizardType,
}

/// 向导类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WizardType {
    /// 首次配置向导
    InitialSetup,

    /// 添加单个模型配置
    AddModel,

    /// 批量配置向导
    BatchSetup,
}

impl WizardState {
    /// 创建新的向导状态
    pub fn new(wizard_type: WizardType) -> Self {
        Self {
            current_step: 0,
            completed_steps: Vec::new(),
            data: HashMap::new(),
            timestamp: chrono::Utc::now(),
            wizard_type,
        }
    }

    /// 保存向导状态到文件
    pub fn save(&self, path: &std::path::Path) -> Result<(), WizardError> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        // 设置文件权限 600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// 从文件加载向导状态
    pub fn load(path: &std::path::Path) -> Result<Option<Self>, WizardError> {
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(path)?;
        let state: WizardState = toml::from_str(&content)?;
        Ok(Some(state))
    }

    /// 检查状态是否过期（超过 24 小时）
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(self.timestamp);
        duration.num_hours() > 24
    }
}
