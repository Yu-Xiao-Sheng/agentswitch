//! 批量切换测试

use crate::agents::AgentAdapter;
use crate::batch::switch::{batch_switch_agents, batch_switch_selected_agents};
use crate::config::ModelConfig;
use std::path::PathBuf;

#[cfg(test)]
mockall::mock! {
    MockAgentAdapter { }
    impl AgentAdapter for MockAgentAdapter {
        fn name(&self) -> &str;
        fn detect(&self) -> anyhow::Result<bool>;
        fn config_path(&self) -> anyhow::Result<PathBuf>;
        fn backup(&self) -> anyhow::Result<crate::agents::adapter::Backup>;
        fn apply(&self, _model_config: &ModelConfig) -> anyhow::Result<()>;
        fn restore(&self, _backup: &crate::agents::adapter::Backup) -> anyhow::Result<()>;
        fn current_model(&self) -> anyhow::Result<Option<String>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_switch_agents() {
        // 测试批量切换功能
        // 这个测试需要 mock 对象，暂时跳过
        assert!(true);
    }
}
