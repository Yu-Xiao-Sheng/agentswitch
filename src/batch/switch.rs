//! 批量切换功能

use crate::agents::AgentAdapter;
use crate::batch::status::{BatchOperationResult, ToolOperationResult};
use crate::config::Provider;
use rayon::prelude::*;
use std::time::Instant;

/// 批量切换工具到指定模型
pub fn batch_switch_agents(
    adapters: Vec<Box<dyn AgentAdapter>>,
    provider: &Provider,
    model: &str,
) -> BatchOperationResult {
    let start = Instant::now();
    let total = adapters.len();

    // 并发执行切换操作
    let results: Vec<ToolOperationResult> = adapters
        .par_iter()
        .map(|adapter| {
            let agent_name = adapter.name().to_string();

            // 备份当前配置
            let backup_path = adapter
                .backup()
                .ok()
                .map(|p| p.backup_path.to_string_lossy().to_string());

            // 应用配置
            let result = adapter.apply(provider, model);

            match result {
                Ok(()) => ToolOperationResult {
                    agent_name,
                    success: true,
                    error_message: None,
                    backup_path,
                },
                Err(e) => ToolOperationResult {
                    agent_name,
                    success: false,
                    error_message: Some(e.to_string()),
                    backup_path,
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

/// 批量切换指定工具列表
pub fn batch_switch_selected_agents(
    adapters: Vec<Box<dyn AgentAdapter>>,
    selected_agents: Vec<String>,
    provider: &Provider,
    model: &str,
) -> BatchOperationResult {
    // 过滤出选中的工具
    let filtered_adapters: Vec<_> = adapters
        .into_iter()
        .filter(|a| selected_agents.contains(&a.name().to_string()))
        .collect();

    batch_switch_agents(filtered_adapters, provider, model)
}
