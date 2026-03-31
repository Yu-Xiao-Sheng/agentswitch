//! 批量验证功能

use crate::agents::AgentAdapter;
use crate::batch::status::{BatchOperationResult, ToolOperationResult};
use rayon::prelude::*;
use std::time::Instant;

/// 验证工具配置（简化版，检查工具是否已配置）
#[allow(clippy::borrowed_box)]
fn validate_agent_config(adapter: &Box<dyn AgentAdapter>) -> anyhow::Result<()> {
    // 检查配置文件是否存在
    let config_path = adapter.config_path()?;
    if config_path.exists() {
        Ok(())
    } else {
        anyhow::bail!("配置文件不存在")
    }
}

/// 批量验证工具配置
pub fn batch_validate_agents(adapters: Vec<Box<dyn AgentAdapter>>) -> BatchOperationResult {
    let start = Instant::now();
    let total = adapters.len();

    let results: Vec<ToolOperationResult> = adapters
        .par_iter()
        .map(|adapter| {
            let agent_name = adapter.name().to_string();

            // 验证配置
            let result = validate_agent_config(adapter);

            match result {
                Ok(()) => ToolOperationResult {
                    agent_name,
                    success: true,
                    error_message: None,
                    backup_path: None,
                },
                Err(e) => ToolOperationResult {
                    agent_name,
                    success: false,
                    error_message: Some(e.to_string()),
                    backup_path: None,
                },
            }
        })
        .collect();

    let succeeded = results.iter().filter(|r| r.success).count();
    let failed = total - succeeded;
    let duration_ms = start.elapsed().as_millis();

    BatchOperationResult {
        total,
        succeeded,
        failed,
        results,
        duration_ms,
    }
}

/// 批量验证指定工具列表
pub fn batch_validate_selected_agents(
    adapters: Vec<Box<dyn AgentAdapter>>,
    selected_agents: Vec<String>,
) -> BatchOperationResult {
    let filtered_adapters: Vec<_> = adapters
        .into_iter()
        .filter(|a| selected_agents.contains(&a.name().to_string()))
        .collect();

    batch_validate_agents(filtered_adapters)
}
