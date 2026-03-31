//! CLI 命令实现

use crate::agents::{all_adapters, global_registry};
use crate::backup::BackupManager;
use crate::config::{ConfigStore, Protocol, Provider};
use crate::output::{format_providers_table, print_info, print_success, print_warning};
use crate::utils::{validate_model_name, validate_url};
use colored::Colorize;

// 用于 backup list 命令
#[allow(unused_imports)]
use crate::backup::BackupInfo;

// 导入预设和批量命令类型
use super::args::{BatchCommands, PresetCommands};

/// Provider 管理命令（替代旧 ModelCommands）
#[derive(clap::Subcommand, Debug)]
pub enum ProviderCommands {
    /// 添加供应商
    Add {
        /// 供应商名称
        name: String,
        #[arg(long)]
        base_url: String,
        #[arg(long)]
        api_key: String,
        #[arg(long, value_delimiter = ',')]
        models: Vec<String>,
        /// 协议类型 (openai/anthropic)
        #[arg(long, default_value = "openai")]
        protocol: String,
        #[arg(long)]
        test: bool,
    },
    /// 列出所有供应商
    List,
    /// 删除供应商
    Remove {
        name: String,
    },
    /// 显示供应商详情
    Show {
        name: String,
    },
    /// 测试供应商连接
    Test {
        name: String,
        #[arg(long)]
        model: Option<String>,
    },
    /// 从 API 获取模型列表
    FetchModels {
        name: String,
    },
}

/// Agent 管理命令
#[derive(clap::Subcommand, Debug)]
pub enum AgentCommands {
    /// 检测已安装的 Code Agent 工具
    Detect,
    /// 列出所有已注册的适配器
    List,
}

/// Backup 管理命令
#[derive(clap::Subcommand, Debug)]
pub enum BackupCommands {
    /// 列出所有备份
    List,
    /// 恢复备份
    Restore {
        /// Agent 名称
        agent: String,
        /// 备份时间戳
        #[arg(long)]
        backup: String,
    },
    /// 清理旧备份
    Clean {
        /// 时间间隔（如 7d, 1w, 1m）
        #[arg(long)]
        older_than: String,
    },
}

impl ProviderCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            ProviderCommands::Add {
                name,
                base_url,
                api_key,
                models,
                protocol,
                test,
            } => execute_add_provider(name, base_url, api_key, models, protocol, *test),
            ProviderCommands::List => execute_list_providers(),
            ProviderCommands::Remove { name } => execute_remove_provider(name),
            ProviderCommands::Show { name } => execute_show_provider(name),
            ProviderCommands::Test { name, model } => execute_test_provider(name, model.as_ref()),
            ProviderCommands::FetchModels { name } => execute_fetch_models(name),
        }
    }
}

impl AgentCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            AgentCommands::Detect => execute_detect_agents(),
            AgentCommands::List => execute_list_adapters(),
        }
    }
}

impl BackupCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            BackupCommands::List => execute_list_backups(),
            BackupCommands::Restore { agent, backup } => execute_restore_backup(agent, backup),
            BackupCommands::Clean { older_than } => execute_clean_backups(older_than),
        }
    }
}

fn execute_add_provider(
    name: &str,
    base_url: &str,
    api_key: &str,
    models: &[String],
    protocol: &str,
    test: bool,
) -> anyhow::Result<()> {
    validate_model_name(name)?;
    validate_url(base_url)?;

    if models.is_empty() {
        anyhow::bail!("至少需要指定一个模型");
    }

    let proto = match protocol.to_lowercase().as_str() {
        "openai" => Protocol::OpenAI,
        "anthropic" => Protocol::Anthropic,
        _ => anyhow::bail!("不支持的协议类型: {} (支持: openai, anthropic)", protocol),
    };

    let provider = Provider::new(
        name.to_string(),
        base_url.to_string(),
        api_key.to_string(),
        proto,
        models.to_vec(),
    );

    provider.validate()?;

    let mut store = ConfigStore::new()?;
    store.add_provider(provider)?;

    print_success(&format!("供应商已添加: {}", name));
    println!("  协议: {}", protocol);
    println!("  可用模型: {}", models.join(", "));

    if test {
        println!("\n正在测试连接...");
        execute_test_provider(name, None)?;
    }

    Ok(())
}

fn execute_list_providers() -> anyhow::Result<()> {
    let store = ConfigStore::new()?;
    let providers = store.list_providers();

    if providers.is_empty() {
        print_warning("当前没有配置任何供应商");
        print_info("使用 'asw provider add <name>' 添加供应商");
    } else {
        println!();
        println!("{}", format_providers_table(providers));
    }

    Ok(())
}

fn execute_remove_provider(name: &str) -> anyhow::Result<()> {
    let mut store = ConfigStore::new()?;
    store.remove_provider(name)?;

    print_success(&format!("供应商已删除: {}", name));

    Ok(())
}

/// 显示供应商详情
fn execute_show_provider(name: &str) -> anyhow::Result<()> {
    let store = ConfigStore::new()?;
    let provider = store
        .get_provider(name)
        .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", name))?;

    println!("\n{}", "供应商详情".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    println!("{:<15} {}", "名称:", provider.name);
    println!("{:<15} {}", "Base URL:", provider.base_url);
    println!("{:<15} {}", "协议:", provider.protocol.as_str());

    // API Key 掩码显示
    let masked_key = if provider.api_key.len() > 8 {
        format!(
            "{}...{}",
            &provider.api_key[..4],
            &provider.api_key[provider.api_key.len() - 4..]
        )
    } else {
        "****".to_string()
    };
    println!("{:<15} {}", "API Key:", masked_key);

    println!("{:<15} {}", "可用模型:", provider.models.join(", "));

    println!();

    Ok(())
}

/// 测试供应商连接
fn execute_test_provider(name: &str, specific_model: Option<&String>) -> anyhow::Result<()> {
    use reqwest::blocking::Client;
    use std::time::Instant;

    let store = ConfigStore::new()?;
    let provider = store
        .get_provider(name)
        .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", name))?;

    println!("\n{}", format!("测试 {} 连接...", name).cyan());

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // 测试 API 端点可达性
    print!("  检查 API 端点... ");
    let start = Instant::now();

    let test_url = if provider.base_url.ends_with("/v1") || provider.base_url.ends_with("/v1/") {
        format!("{}/models", provider.base_url.trim_end_matches('/'))
    } else {
        format!("{}/v1/models", provider.base_url.trim_end_matches('/'))
    };

    let response = client
        .get(&test_url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .send();

    match response {
        Ok(resp) => {
            let elapsed = start.elapsed();
            println!("{} ({}ms)", "✓".green(), elapsed.as_millis());

            if resp.status().is_success() {
                println!("  {} API 端点可达", "✓".green());
                println!("  {} API Key 有效", "✓".green());

                // 测试特定模型
                let test_model = specific_model
                    .cloned()
                    .or_else(|| provider.get_default_model().map(|s| s.to_string()));

                if let Some(model_name) = test_model {
                    if provider.has_model(&model_name) {
                        println!("  {} 模型 {} 可用", "✓".green(), model_name);
                    } else {
                        println!("  {} 模型 {} 不在配置列表中", "⚠".yellow(), model_name);
                    }
                }

                println!();
                println!("{}", "测试结果: 成功".green().bold());
            } else {
                println!("  {} API 返回错误: {}", "✗".red(), resp.status());
                println!();
                println!("{}", "测试结果: 失败".red().bold());
            }
        }
        Err(e) => {
            println!("{}", "✗".red());
            println!("  {} 连接失败: {}", "✗".red(), e);
            println!();
            println!("{}", "测试结果: 失败".red().bold());
        }
    }

    Ok(())
}

/// 从 API 获取模型列表
fn execute_fetch_models(name: &str) -> anyhow::Result<()> {
    use reqwest::blocking::Client;

    let store = ConfigStore::new()?;
    let provider = store
        .get_provider(name)
        .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", name))?;

    println!("{}", format!("正在从 {} 获取模型列表...", name).cyan());

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let fetch_url = if provider.base_url.ends_with("/v1") || provider.base_url.ends_with("/v1/") {
        format!("{}/models", provider.base_url.trim_end_matches('/'))
    } else {
        format!("{}/v1/models", provider.base_url.trim_end_matches('/'))
    };

    let response = client
        .get(&fetch_url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .send()?;

    if !response.status().is_success() {
        anyhow::bail!("获取模型列表失败: {}", response.status());
    }

    // 解析响应
    let json: serde_json::Value = response.json()?;
    let models: Vec<String> = if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
        data.iter()
            .filter_map(|m| m.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()))
            .collect()
    } else {
        anyhow::bail!("无法解析模型列表响应");
    };

    if models.is_empty() {
        println!("{}", "未找到可用模型".yellow());
        return Ok(());
    }

    println!("{} 找到 {} 个可用模型:", "✓".green(), models.len());
    for model in &models {
        println!("  - {}", model);
    }

    // 询问是否添加
    println!();
    print!("是否添加这些模型到供应商 {} ? [y/N]: ", name);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        let mut store = ConfigStore::new()?;
        store.edit_provider(name, |p| {
            p.models = models.clone();
            Ok(())
        })?;

        println!("{} 已添加 {} 个模型到 {}", "✓".green(), models.len(), name);
    }

    Ok(())
}

/// 批量添加模型（已移除，保留为兼容入口）
fn execute_batch_add_models(_name: &str, _file: Option<&String>) -> anyhow::Result<()> {
    anyhow::bail!("批量添加模型功能已迁移到 provider 命令，请使用 'asw provider add'");
}

/// 执行 agent detect 命令
fn execute_detect_agents() -> anyhow::Result<()> {
    println!("{}", "Agent Detection Results:".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    let adapters = all_adapters();

    for adapter in adapters {
        let name = adapter.name();
        let is_installed = adapter.detect()?;

        let status = if is_installed {
            "✓".green()
        } else {
            "✗".red()
        };

        let status_text = if is_installed {
            "已安装".green()
        } else {
            "未安装".red()
        };

        let config_path = adapter.config_path()?;
        let config_path_str = config_path.display().to_string();

        println!("{:<20} {} {}", name.bold(), status, status_text);

        if is_installed {
            println!("{:<20} {}", "", config_path_str);
        } else {
            let install_hint = match name {
                "claude-code" => "npm install -g @anthropic-ai/claude-code",
                "codex" => "npm install -g @openai/codex@0.80.0",
                "gemini-cli" => "npm install -g @google/gemini-cli",
                _ => "请查看官方文档",
            };
            println!("{:<20} {}", "", format!("需要运行: {}", install_hint));
        }

        println!();
    }

    println!(
        "{}",
        "提示: 使用 'asw switch <agent> <provider> <model>' 切换工具的模型配置".cyan()
    );

    Ok(())
}

/// 执行 backup list 命令
fn execute_list_backups() -> anyhow::Result<()> {
    let backup_manager = BackupManager::new()?;

    println!("{}", "Backup List:".green().bold());
    println!("{}", "=".repeat(20).green());
    println!();

    let backups = backup_manager.list_all_backups()?;

    if backups.is_empty() {
        println!("{}", "暂无备份文件".yellow());
        return Ok(());
    }

    println!(
        "{}",
        format!(
            "{:<20} {:<20} {:<10} {}",
            "Agent", "Timestamp", "Size", "Path"
        )
        .cyan()
    );
    println!("{}", "-".repeat(80));

    for backup in backups {
        let size_str = if backup.size_bytes < 1024 {
            format!("{} B", backup.size_bytes)
        } else if backup.size_bytes < 1024 * 1024 {
            format!("{} KB", backup.size_bytes / 1024)
        } else {
            format!("{} MB", backup.size_bytes / (1024 * 1024))
        };

        println!(
            "{:<20} {:<20} {:<10} {}",
            backup.agent_name,
            backup.timestamp,
            size_str,
            backup.file_path.display()
        );
    }

    Ok(())
}

/// 执行 backup restore 命令
fn execute_restore_backup(agent: &str, backup: &str) -> anyhow::Result<()> {
    println!("{}", "正在恢复配置...".cyan());
    println!("Agent: {}, Backup: {}", agent, backup);

    let backup_manager = BackupManager::new()?;
    let backup_info = backup_manager.find_backup(agent, backup)?;

    // 查找对应的适配器
    let adapters = all_adapters();
    let adapter = adapters
        .into_iter()
        .find(|a| a.name() == agent)
        .ok_or_else(|| anyhow::anyhow!("未知的工具: {}", agent))?;

    // 执行恢复
    adapter.restore(&backup_info)?;

    println!(
        "{}",
        format!("✓ 已恢复 {} 到 {} 的备份", agent, backup).green()
    );

    Ok(())
}

/// 执行 backup clean 命令
fn execute_clean_backups(older_than: &str) -> anyhow::Result<()> {
    println!("{}", "正在清理旧备份...".cyan());
    println!("清理 {} 前的备份", older_than);

    // 解析时间间隔
    let seconds = parse_duration(older_than)?;

    let backup_manager = BackupManager::new()?;
    let cleaned_count = backup_manager.clean_old_backups_by_duration(seconds)?;

    if cleaned_count == 0 {
        println!("{}", "没有需要清理的备份".yellow());
    } else {
        println!("{}", format!("✓ 已清理 {} 个旧备份", cleaned_count).green());
    }

    Ok(())
}

/// 解析时间间隔字符串（如 "7d", "1w", "1m"）
fn parse_duration(duration: &str) -> anyhow::Result<i64> {
    let duration = duration.to_lowercase();
    let chars: Vec<char> = duration.chars().collect();
    let split_pos = chars.len().saturating_sub(1);

    let num_str: String = chars[..split_pos].iter().collect();
    let unit: String = chars[split_pos..].iter().collect();

    let num: i64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("时间间隔格式错误"))?;

    let seconds = match unit.as_str() {
        "s" => num,
        "m" => num * 60,
        "h" => num * 60 * 60,
        "d" => num * 60 * 60 * 24,
        "w" => num * 60 * 60 * 24 * 7,
        _ => anyhow::bail!("不支持的时间单位: {} (支持 s/m/h/d/w)", unit),
    };

    Ok(seconds)
}

/// 显示状态
pub fn execute_show_status() -> anyhow::Result<()> {
    println!("{}", "Agent Configuration Status:".green().bold());
    println!("{}", "=".repeat(50).green());
    println!();

    let store = ConfigStore::new()?;
    let active = store.get_all_active();

    let adapters = all_adapters();

    for adapter in adapters {
        let name = adapter.name();
        let is_installed = adapter.detect()?;
        let config_path = adapter.config_path()?;
        let _config_exists = config_path.exists();

        let active_info = active.get(name);
        let status = if active_info.is_some() {
            "✓".green()
        } else if is_installed {
            "⚠".yellow()
        } else {
            "✗".red()
        };

        let model_text = active_info.map(|am| format!("{}/{}", am.provider, am.model)).unwrap_or_else(|| {
            if is_installed {
                "-".to_string()
            } else {
                "".to_string()
            }
        });

        println!(
            "{:<20} {:<25} {:<40} {}",
            name.bold(),
            model_text,
            config_path.display().to_string(),
            status
        );
    }

    println!();
    println!("{}", "Legend:".cyan());
    println!("  ✓ = 已配置  ⚠ = 未配置  ✗ = 未安装");
    println!();
    println!(
        "{}",
        "提示: 使用 'asw switch <agent> <provider> <model>' 配置工具".cyan()
    );

    Ok(())
}

/// 执行 switch 命令（新版：接受 provider + model 参数）
pub fn execute_switch(agent: &str, provider: &str, model: &str) -> anyhow::Result<()> {
    // 验证供应商是否存在
    let mut store = ConfigStore::new()?;
    let provider_obj = store
        .get_provider(provider)
        .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", provider))?;

    // 检测工具是否已安装
    let adapters = all_adapters();
    let adapter = adapters
        .into_iter()
        .find(|a| a.name() == agent)
        .ok_or_else(|| anyhow::anyhow!("未知的工具: {}", agent))?;

    if !adapter.detect()? {
        anyhow::bail!("未检测到 {} 安装", agent);
    }

    // 检查配置文件是否存在
    let config_path = adapter.config_path()?;
    let has_config = config_path.exists();

    // 步骤 1: 创建备份（如果配置文件存在）
    if has_config {
        println!("{}", "正在备份原配置...".cyan());
        let _backup = adapter.backup()?;
        println!("{}", "✓ 备份完成".green());
    }

    // 步骤 2: 应用新配置
    println!(
        "{}",
        format!("正在切换 {} 到 {}/{} 模型...", agent, provider, model).cyan()
    );
    adapter.apply(provider_obj, model)?;
    println!("{}", format!("✓ {} 已切换到 {}/{}", agent, provider, model).green());

    // 步骤 3: 更新 active 映射
    store.set_active(agent, provider, model)?;

    // 步骤 4: 检测环境变量
    println!();
    println!("{}", "⚠ 提示: 环境变量可能会覆盖配置文件设置".yellow());
    println!("{}", "请检查以下环境变量:".yellow());
    match agent {
        "claude-code" => println!("  - ANTHROPIC_AUTH_TOKEN"),
        "codex" => println!("  - OPENAI_API_KEY"),
        "gemini-cli" => println!("  - GEMINI_API_KEY, GOOGLE_GEMINI_BASE_URL"),
        _ => {}
    }

    Ok(())
}

/// 执行 agent list 命令
fn execute_list_adapters() -> anyhow::Result<()> {
    println!("{}", "已注册的适配器:".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    let registry = global_registry();
    let adapters = registry.list_adapters();

    if adapters.is_empty() {
        println!("{}", "暂无已注册的适配器".yellow());
        return Ok(());
    }

    println!(
        "{}",
        format!("{:<25} {:<15}", "适配器名称", "安装状态").cyan()
    );
    println!("{}", "-".repeat(40));

    for adapter_info in adapters {
        let status = if adapter_info.is_installed {
            "✓ 已安装".green()
        } else {
            "✗ 未安装".red()
        };

        println!("{:<25} {}", adapter_info.name, status);
    }

    println!();
    println!("{}", "提示: 使用 'asw agent detect' 查看详细信息".cyan());

    Ok(())
}

/// Preset 命令实现
impl PresetCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            PresetCommands::Create {
                name,
                description,
                tag,
                agent,
            } => execute_preset_create(name, description, tag, agent),
            PresetCommands::List { tag, format } => execute_preset_list(tag, format),
            PresetCommands::Show { name } => execute_preset_show(name),
            PresetCommands::Apply {
                name,
                agent,
                dry_run,
                no_backup,
            } => execute_preset_apply(name, agent, *dry_run, *no_backup),
            PresetCommands::Update {
                name,
                description,
                tag,
                agent,
            } => execute_preset_update(name, description, tag, agent),
            PresetCommands::Delete { name, force } => execute_preset_delete(name, *force),
            PresetCommands::Validate { name } => execute_preset_validate(name),
            PresetCommands::Import {
                input,
                strategy,
                dry_run,
            } => execute_preset_import(input, strategy, *dry_run),
            PresetCommands::Export {
                output,
                preset,
                include_models,
                include_active,
            } => execute_preset_export(output, preset, *include_models, *include_active),
        }
    }
}

/// Batch 命令实现
impl BatchCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            BatchCommands::Switch {
                model,
                agent,
                parallel,
                dry_run,
            } => execute_batch_switch(model, agent, *parallel, *dry_run),
            BatchCommands::Validate { agent } => execute_batch_validate(agent),
            BatchCommands::Status { format } => execute_batch_status(format),
        }
    }
}

// ============ Preset 命令实现 ============

fn execute_preset_create(
    name: &str,
    description: &Option<String>,
    _tags: &[String],
    agents: &[String],
) -> anyhow::Result<()> {
    use crate::config::ConfigStore;
    use crate::presets::{Preset, PresetStore};

    // 解析 agent 映射
    let mut mappings = std::collections::HashMap::new();
    for agent_str in agents {
        let parts: Vec<&str> = agent_str.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("无效的映射格式: {}", agent_str);
        }
        mappings.insert(parts[0].to_string(), parts[1].to_string());
    }

    // 验证模型配置存在
    let config_store = ConfigStore::new()?;
    for model_name in mappings.values() {
        if !config_store.has_model(model_name) {
            anyhow::bail!("模型配置不存在: {}", model_name);
        }
    }

    // 创建预设
    let preset = Preset::new(
        name.to_string(),
        description.clone().unwrap_or_default(),
        mappings,
    );

    // 验证预设
    let available_models = config_store
        .list_models()
        .iter()
        .map(|m| m.name.clone())
        .collect::<std::collections::HashSet<_>>();
    preset.validate(&available_models)?;

    // 保存预设
    let mut store = PresetStore::new()?;
    store.add_preset(preset)?;

    print_success(&format!("✅ 预设创建成功: {}", name));
    Ok(())
}

fn execute_preset_list(tags: &[String], format: &str) -> anyhow::Result<()> {
    use crate::presets::PresetStore;

    let store = PresetStore::new()?;
    let presets = store.list_presets()?;

    // 按标签筛选
    let filtered: Vec<_> = if tags.is_empty() {
        presets
    } else {
        presets
            .into_iter()
            .filter(|p| tags.iter().any(|t| p.tags.contains(t)))
            .collect()
    };

    if filtered.is_empty() {
        println!("没有找到预设");
        return Ok(());
    }

    // 格式化输出
    match format {
        "table" => {
            println!("\n可用的预设 ({}):\n", filtered.len());
            println!("{:<20} {:<30} {:<15} 更新时间", "名称", "描述", "标签");
            println!("{}", "-".repeat(80));
            for preset in &filtered {
                let tags = preset.tags.join(", ");
                println!(
                    "{:<20} {:<30} {:<15} {}",
                    preset.name,
                    preset.description,
                    tags,
                    preset.updated_at.format("%Y-%m-%d %H:%M")
                );
            }
        }
        "json" => {
            let json = serde_json::to_string_pretty(&filtered)?;
            println!("{}", json);
        }
        _ => {
            anyhow::bail!("不支持的格式: {}", format);
        }
    }

    Ok(())
}

fn execute_preset_show(name: &str) -> anyhow::Result<()> {
    use crate::presets::PresetStore;

    let store = PresetStore::new()?;
    let preset = store.get_preset(name)?;

    println!("\n预设: {}\n", preset.name);
    println!("描述: {}\n", preset.description);
    println!("标签: {}\n", preset.tags.join(", "));
    println!(
        "创建时间: {}",
        preset.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "更新时间: {}",
        preset.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!("\n工具映射:");
    for (agent, model) in &preset.mappings {
        println!("  {} → {}", agent, model);
    }

    Ok(())
}

fn execute_preset_apply(
    name: &str,
    agents: &[String],
    dry_run: bool,
    no_backup: bool,
) -> anyhow::Result<()> {
    use crate::agents::all_adapters;
    use crate::config::ConfigStore;
    use crate::presets::{PresetAppplier, PresetStore};

    let store = PresetStore::new()?;
    let preset = store.get_preset(name)?;

    let config_store = ConfigStore::new()?;
    let model_configs = config_store.load_all();

    // 获取适配器
    let adapters = all_adapters();

    // 创建应用器
    let appplier = PresetAppplier::new(adapters);

    if dry_run {
        println!("\n[模拟运行] 应用预设: {}\n", name);
        println!("将应用配置:");
        for (agent, model) in &preset.mappings {
            println!("  {} → {}", agent, model);
        }
        return Ok(());
    }

    println!("\n应用预设: {}", name);

    if !no_backup {
        println!("备份配置...");
        // 备份由 appplier 处理
    }

    // 应用预设
    if agents.is_empty() {
        appplier.apply(&preset, &model_configs)?;
    } else {
        appplier.apply_to_agents(&preset, &model_configs, agents)?;
    }

    print_success("✅ 预设应用成功");
    Ok(())
}

fn execute_preset_update(
    name: &str,
    _description: &Option<String>,
    _tags: &[String],
    _agents: &[String],
) -> anyhow::Result<()> {
    print_warning(&format!("⚠️  预设更新功能将在后续版本实现: {}", name));
    Ok(())
}

fn execute_preset_delete(name: &str, force: bool) -> anyhow::Result<()> {
    use crate::presets::PresetStore;

    if !force {
        print!("确认删除预设 '{}'? [y/N]: ", name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("已取消");
            return Ok(());
        }
    }

    let mut store = PresetStore::new()?;
    store.remove_preset(name)?;

    print_success(&format!("✅ 预设删除成功: {}", name));
    Ok(())
}

fn execute_preset_validate(name: &str) -> anyhow::Result<()> {
    use crate::config::ConfigStore;
    use crate::presets::{validate_preset_agents, PresetStore};

    let store = PresetStore::new()?;
    let preset = store.get_preset(name)?;

    println!("\n验证预设: {}\n", name);

    // 验证模型配置
    let config_store = ConfigStore::new()?;
    let available_models = config_store
        .list_models()
        .iter()
        .map(|m| m.name.clone())
        .collect::<std::collections::HashSet<_>>();

    let models_check = preset.validate(&available_models);
    if models_check.is_ok() {
        println!("✓ 所有模型配置存在");
    } else {
        println!("✗ 模型配置验证失败: {}", models_check.unwrap_err());
    }

    // 验证工具安装状态
    let missing_agents = validate_preset_agents(&preset)?;
    if missing_agents.is_empty() {
        println!("✓ 所有工具已安装");
    } else {
        println!("⚠ 未安装工具: {}", missing_agents.join(", "));
    }

    println!("\n✅ 预设验证通过");
    Ok(())
}

fn execute_preset_export(
    output: &str,
    _presets: &[String],
    _include_models: bool,
    _include_active: bool,
) -> anyhow::Result<()> {
    use crate::io::export_presets;
    use crate::presets::PresetStore;
    use std::path::Path;

    let store = PresetStore::new()?;
    let presets = store.list_presets()?;

    let output_path = Path::new(output);
    export_presets(&presets, output_path)?;

    print_success(&format!("✅ 导出 {} 个预设到: {}", presets.len(), output));
    Ok(())
}

fn execute_preset_import(input: &str, strategy: &str, dry_run: bool) -> anyhow::Result<()> {
    use crate::config::ConfigStore;
    use crate::io::{
        check_import_dependencies, import_presets, preview_import_changes, validate_import_file,
        ImportStrategy,
    };
    use crate::presets::PresetStore;
    use std::path::Path;

    let input_path = Path::new(input);

    // 验证导入文件
    println!("验证导入文件...");
    validate_import_file(input_path)?;

    // 读取导入包
    println!("读取导入包...");
    let strategy = match strategy {
        "merge" => ImportStrategy::Merge,
        "overwrite" => ImportStrategy::Overwrite,
        _ => anyhow::bail!("不支持的导入策略: {} (支持: merge, overwrite)", strategy),
    };

    let imported_presets = import_presets(input_path, strategy)?;

    // 检查模型依赖
    println!("检查模型依赖...");
    let config_store = ConfigStore::new()?;
    let available_models = config_store.load_all();

    // 构造临时的ExportPackage用于检查和预览
    let temp_package = crate::io::export::ExportPackage {
        version: "1.0.0".to_string(),
        exported_at: chrono::Utc::now(),
        presets: imported_presets.clone(),
        model_configs: None,
        active_config: None,
    };

    let missing_models = check_import_dependencies(&temp_package, &available_models);

    if !missing_models.is_empty() {
        println!("⚠️  缺失的模型配置: {}", missing_models.join(", "));
        println!("   请先添加这些模型配置后再导入");
    }

    // 显示预览
    let store = PresetStore::new()?;
    let existing_presets = store.list_presets()?;
    let existing_map: std::collections::HashMap<_, _> = existing_presets
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    let preview = preview_import_changes(&temp_package, &existing_map);

    println!("\n导入预览:");
    if !preview.new_presets.is_empty() {
        println!("  新增预设: {}", preview.new_presets.join(", "));
    }
    if !preview.conflict_presets.is_empty() {
        println!("  冲突预设: {}", preview.conflict_presets.join(", "));
    }

    if dry_run {
        println!("\n[模拟运行] 导入操作未执行");
        return Ok(());
    }

    // 执行导入
    println!("\n导入预设...");
    let mut store = PresetStore::new()?;

    for preset in imported_presets {
        match strategy {
            ImportStrategy::Merge => {
                if !existing_map.contains_key(&preset.name) {
                    store.add_preset(preset)?;
                }
            }
            ImportStrategy::Overwrite => {
                store.update_preset(preset)?;
            }
        }
    }

    print_success("✅ 导入完成");
    Ok(())
}

// ============ Batch 命令实现 ============

fn execute_batch_switch(
    model: &str,
    agents: &[String],
    _parallel: usize,
    dry_run: bool,
) -> anyhow::Result<()> {
    use crate::agents::all_adapters;
    use crate::batch::batch_switch_agents;
    use crate::config::ConfigStore;

    // 解析 model 参数，格式可能是 "provider:model" 或仅 "model"
    let (provider_name, model_name) = if model.contains(':') {
        let parts: Vec<&str> = model.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("无效的模型格式，应为 'provider:model' 或 'model'");
        }
        (parts[0].to_string(), parts[1].to_string())
    } else {
        // 仅提供模型名称，需要查找对应的 provider
        ("".to_string(), model.to_string())
    };

    let config_store = ConfigStore::new()?;

    // 获取 Provider
    let provider_obj = if provider_name.is_empty() {
        // 查找包含该模型的 provider
        config_store
            .list_providers()
            .iter()
            .find(|p| p.has_model(&model_name))
            .ok_or_else(|| anyhow::anyhow!("未找到包含模型 '{}' 的供应商", model_name))?
            .clone()
    } else {
        config_store
            .get_provider(&provider_name)
            .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", provider_name))?
            .clone()
    };

    // 验证模型是否存在于 provider 中
    if !provider_obj.has_model(&model_name) {
        anyhow::bail!(
            "模型 '{}' 不在供应商 '{}' 的模型列表中",
            model_name,
            provider_obj.name
        );
    }

    println!("\n批量切换到模型: {}/{}", provider_obj.name, model_name);

    if dry_run {
        println!("\n[模拟运行] 将切换以下工具到 {}/{}:\n", provider_obj.name, model_name);
        return Ok(());
    }

    // 获取适配器
    let adapters = all_adapters();

    // 过滤工具
    let adapters_to_switch: Vec<Box<dyn crate::agents::AgentAdapter>> = if agents.is_empty() {
        adapters
    } else {
        adapters
            .into_iter()
            .filter(|a| agents.contains(&a.name().to_string()))
            .collect()
    };

    // 执行批量切换
    let result = batch_switch_agents(adapters_to_switch, &provider_obj, &model_name);

    // 显示结果
    println!("\n切换工具 (并发数: {}):", 4);
    for tool_result in &result.results {
        if tool_result.success {
            println!("  {} ... {}", tool_result.agent_name, "✓".green());
        } else {
            println!("  {} ... {}", tool_result.agent_name, "✗".red());
            if let Some(err) = &tool_result.error_message {
                println!("    错误: {}", err);
            }
        }
    }

    println!("\n✓ 批量切换成功 ({}/{})", result.succeeded, result.total);
    println!("耗时: {:.3} 秒", result.duration_ms as f64 / 1000.0);

    Ok(())
}

fn execute_batch_validate(_agents: &[String]) -> anyhow::Result<()> {
    use crate::agents::all_adapters;
    use crate::batch::batch_validate_agents;

    let adapters = all_adapters();
    let result = batch_validate_agents(adapters);

    println!("\n批量验证工具配置\n");
    println!("验证结果:");
    for tool_result in &result.results {
        if tool_result.success {
            println!("  {}: ✓ 配置有效", tool_result.agent_name);
        } else {
            println!("  {}: ✗ 配置无效", tool_result.agent_name);
            if let Some(err) = &tool_result.error_message {
                println!("    原因: {}", err);
            }
        }
    }

    println!("\n验证完成: {}/{} 有效", result.succeeded, result.total);

    Ok(())
}

fn execute_batch_status(_format: &str) -> anyhow::Result<()> {
    use crate::agents::all_adapters;
    use colored::Colorize;

    let adapters = all_adapters();
    let total = adapters.len();

    println!("\n工具配置状态:\n");
    println!("{:<20} {:<15} {:<10} 最后更新", "工具", "模型", "状态");
    println!("{}", "-".repeat(60));

    for adapter in adapters {
        let name = adapter.name();
        let model = adapter
            .current_model()?
            .unwrap_or_else(|| "未配置".to_string());
        let status = "✓ 有效".green();
        let updated = "未知".to_string();

        println!("{:<20} {:<15} {:<10} {}", name, model, status, updated);
    }

    println!("\n总计: {} 个工具", total);

    Ok(())
}

// ============ Spec 004 新增命令 ============

/// 加密管理命令
#[derive(clap::Subcommand, Debug)]
pub enum CryptoCommands {
    /// 生成新的加密密钥
    Keygen,

    /// 导出密钥（Base64 格式）
    KeyExport,

    /// 从 Base64 字符串导入密钥
    KeyImport {
        /// Base64 编码的密钥
        key: String,
    },

    /// 查看加密状态
    Status,
}

impl CryptoCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            CryptoCommands::Keygen => execute_crypto_keygen(),
            CryptoCommands::KeyExport => execute_crypto_key_export(),
            CryptoCommands::KeyImport { key } => execute_crypto_key_import(key),
            CryptoCommands::Status => execute_crypto_status(),
        }
    }
}

fn execute_crypto_keygen() -> anyhow::Result<()> {
    use crate::crypto::{generate_and_save_master_key, get_master_key_path_str};

    match generate_and_save_master_key() {
        Ok(_) => {
            let path = get_master_key_path_str()?;
            print_success(&format!("密钥已生成并保存到: {}", path));
            print_warning("请妥善保管此密钥文件，丢失将无法解密已加密的数据");
            Ok(())
        }
        Err(crate::crypto::CryptoError::KeyAlreadyExists(msg)) => {
            print_warning(&msg);
            print_info("如需重新生成，请先删除旧密钥文件后再执行此命令");
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("生成密钥失败: {}", e)),
    }
}

fn execute_crypto_key_export() -> anyhow::Result<()> {
    use crate::crypto::{export_key_to_base64, load_master_key};

    match load_master_key() {
        Ok(key) => {
            let b64 = export_key_to_base64(&key);
            println!("{}", b64);
            Ok(())
        }
        Err(crate::crypto::CryptoError::KeyNotFound(_)) => {
            print_warning("主密钥不存在");
            print_info("使用 'asw crypto keygen' 生成新密钥");
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("导出密钥失败: {}", e)),
    }
}

fn execute_crypto_key_import(key: &str) -> anyhow::Result<()> {
    use crate::crypto::import_and_save_master_key;

    match import_and_save_master_key(key) {
        Ok(_) => {
            print_success("密钥已导入并保存");
            Ok(())
        }
        Err(crate::crypto::CryptoError::KeyAlreadyExists(msg)) => {
            print_warning(&msg);
            print_info("如需覆盖，请先删除旧密钥文件后再执行此命令");
            Ok(())
        }
        Err(crate::crypto::CryptoError::KeyInvalid(msg)) => {
            Err(anyhow::anyhow!("导入密钥失败: {}", msg))
        }
        Err(e) => Err(anyhow::anyhow!("导入密钥失败: {}", e)),
    }
}

fn execute_crypto_status() -> anyhow::Result<()> {
    use crate::crypto::{get_master_key_path_str, master_key_exists};

    println!("{}", "加密状态".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    let exists = master_key_exists()?;
    let path = get_master_key_path_str()?;

    let status = if exists {
        "✓ 已配置".green()
    } else {
        "✗ 未配置".red()
    };

    println!("{:<20} {}", "主密钥状态:", status);
    println!("{:<20} {}", "密钥文件路径:", path);

    if !exists {
        println!();
        print_info("使用 'asw crypto keygen' 生成新密钥");
    }

    Ok(())
}

/// 向导命令
#[derive(clap::Subcommand, Debug)]
pub enum WizardCommands {
    /// 初始化配置（向导）
    Init {
        /// 恢复之前的向导进度
        #[arg(long)]
        resume: bool,

        /// 重新开始（清除进度）
        #[arg(long)]
        reset: bool,

        /// 尝试非交互式模式（实验性）
        #[arg(long)]
        non_interactive: bool,
    },

    /// 添加模型配置（向导）
    Wizard {
        /// 预设模型名称（跳过输入）
        #[arg(long)]
        name: Option<String>,
    },
}

/// 诊断命令
#[derive(clap::Subcommand, Debug)]
pub enum DoctorCommands {
    /// 运行完整诊断
    Doctor {
        /// 显示详细信息
        #[arg(short, long)]
        verbose: bool,

        /// 以 JSON 格式输出
        #[arg(short, long)]
        json: bool,

        /// 尝试自动修复问题
        #[arg(long)]
        fix: bool,
    },

    /// 检测已安装工具（简化版）
    Detect,
}

/// 补全命令
#[derive(clap::Subcommand, Debug)]
pub enum CompletionCommands {
    /// 安装补全脚本
    Install {
        /// Shell 类型（bash/zsh/fish）
        shell: String,

        /// 自定义安装路径
        #[arg(long)]
        path: Option<String>,

        /// 不修改 Shell 配置文件
        #[arg(long)]
        no_modify_config: bool,
    },

    /// 卸载补全脚本
    Uninstall {
        /// Shell 类型（bash/zsh/fish）
        shell: String,

        /// 自定义安装路径
        #[arg(long)]
        path: Option<String>,
    },

    /// 生成补全脚本到标准输出
    Generate {
        /// Shell 类型（bash/zsh/fish）
        shell: String,
    },
}

/// 同步命令
#[derive(clap::Subcommand, Debug)]
pub enum SyncCommands {
    /// 初始化 Git 仓库
    Init {
        /// 启用加密
        #[arg(long)]
        encrypt: bool,

        /// 加密方法（aes-gcm/git-crypt）
        #[arg(long)]
        encryption_method: Option<String>,

        /// 禁用加密
        #[arg(long)]
        no_encrypt: bool,
    },

    /// 管理远程仓库
    Remote {
        #[command(subcommand)]
        command: RemoteSubCommands,
    },

    /// 推送到远程
    Push {
        /// 远程仓库名称
        #[arg(long)]
        remote: Option<String>,

        /// 分支名称
        #[arg(long)]
        branch: Option<String>,

        /// 跳过加密（不推荐）
        #[arg(long)]
        no_encrypt: bool,
    },

    /// 从远程拉取
    Pull {
        /// 远程仓库名称
        #[arg(long)]
        remote: Option<String>,

        /// 分支名称
        #[arg(long)]
        branch: Option<String>,

        /// 合并策略
        #[arg(long)]
        strategy: Option<String>,
    },

    /// 显示同步状态
    Status,
}

/// 远程仓库子命令
#[derive(clap::Subcommand, Debug)]
pub enum RemoteSubCommands {
    /// 添加远程仓库
    Add {
        /// 远程仓库 URL
        url: String,
    },

    /// 删除远程仓库
    Remove {
        /// 远仓库名称
        name: String,
    },

    /// 列出远程仓库
    List,

    /// 修改远程仓库 URL
    SetUrl {
        /// 远仓库名称
        name: String,
        /// 新的 URL
        url: String,
    },
}

impl WizardCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            WizardCommands::Init {
                resume,
                reset,
                non_interactive: _,
            } => crate::wizard::run_wizard(*resume, *reset)
                .map_err(|e| anyhow::anyhow!("向导执行失败: {}", e)),
            WizardCommands::Wizard { name: _ } => {
                println!("{}", "向导添加模型功能将在后续版本实现".yellow());
                Ok(())
            }
        }
    }
}

impl DoctorCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            DoctorCommands::Doctor { verbose, json, fix } => {
                crate::doctor::run_doctor(*verbose, *json, *fix)
            }
            DoctorCommands::Detect => crate::doctor::run_detect(),
        }
    }
}

impl CompletionCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            CompletionCommands::Install { shell, .. } => {
                crate::completion::install_completion(shell)
            }
            CompletionCommands::Uninstall { shell, .. } => {
                crate::completion::uninstall_completion(shell)
            }
            CompletionCommands::Generate { shell } => {
                let script =
                    crate::completion::static_completion::generate_completion(shell, "asw")?;
                println!("{}", script);
                Ok(())
            }
        }
    }
}

impl SyncCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            SyncCommands::Init { .. } => crate::sync::config::run_sync_init(),
            SyncCommands::Remote { command } => match command {
                RemoteSubCommands::Add { url } => crate::sync::config::run_sync_remote_add(url),
                RemoteSubCommands::Remove { name } => {
                    crate::sync::config::run_sync_remote_remove(name)
                }
                RemoteSubCommands::List => crate::sync::config::run_sync_remote_list(),
                RemoteSubCommands::SetUrl { name, url } => {
                    crate::sync::config::run_sync_remote_set_url(name, url)
                }
            },
            SyncCommands::Push { .. } => crate::sync::config::run_sync_push(),
            SyncCommands::Pull { .. } => crate::sync::config::run_sync_pull(),
            SyncCommands::Status => crate::sync::config::run_sync_status(),
        }
    }
}

/// 更新检查命令实现
#[derive(clap::Subcommand, Debug)]
pub enum UpdateCommands {
    /// 检查更新
    Check {
        /// 强制检查（忽略缓存）
        #[arg(short, long)]
        force: bool,
    },
}

impl UpdateCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            UpdateCommands::Check { force } => execute_update_check(*force),
        }
    }
}

/// 执行更新检查
fn execute_update_check(force: bool) -> anyhow::Result<()> {
    use crate::update::{check_for_update, display_update_notification};
    
    println!("正在检查更新...");
    
    match check_for_update(force) {
        Ok(info) => {
            display_update_notification(&info);
            
            if info.has_update {
                if let Some(url) = &info.release_url {
                    println!("📄 发布说明: {}", url);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 检查更新失败: {}", e);
        }
    }
    
    Ok(())
}
