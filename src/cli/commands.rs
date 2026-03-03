//! CLI 命令实现

use crate::agents::{all_adapters, global_registry};
use crate::backup::BackupManager;
use crate::config::{ConfigStore, ModelConfig};
use crate::output::{format_models_table, print_info, print_success, print_warning};
use crate::utils::{validate_model_name, validate_url};
use colored::Colorize;

// 用于 backup list 命令
#[allow(unused_imports)]
use crate::backup::BackupInfo;

/// Model 管理命令
#[derive(clap::Subcommand, Debug)]
pub enum ModelCommands {
    Add {
        name: String,
        #[arg(long)]
        base_url: String,
        #[arg(long)]
        api_key: String,
        #[arg(long)]
        model: String,
    },
    List,
    Remove {
        name: String,
    },
    Edit {
        name: String,
        #[arg(long)]
        base_url: Option<String>,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        model: Option<String>,
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

impl ModelCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            ModelCommands::Add {
                name,
                base_url,
                api_key,
                model,
            } => execute_add_model(name, base_url, api_key, model),
            ModelCommands::List => execute_list_models(),
            ModelCommands::Remove { name } => execute_remove_model(name),
            ModelCommands::Edit {
                name,
                base_url,
                api_key,
                model,
            } => execute_edit_model(name, base_url.as_ref(), api_key.as_ref(), model.as_ref()),
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

fn execute_add_model(
    name: &str,
    base_url: &str,
    api_key: &str,
    model_id: &str,
) -> anyhow::Result<()> {
    validate_model_name(name)?;
    validate_url(base_url)?;

    let model_config = ModelConfig::new(
        name.to_string(),
        base_url.to_string(),
        api_key.to_string(),
        model_id.to_string(),
    );

    let mut store = ConfigStore::new()?;
    store.add_model(model_config)?;

    print_success(&format!("模型配置已添加: {}", name));

    Ok(())
}

fn execute_list_models() -> anyhow::Result<()> {
    let store = ConfigStore::new()?;
    let models = store.list_models();

    if models.is_empty() {
        print_warning("当前没有配置任何模型");
        print_info("使用 'asw model add <name>' 添加模型配置");
    } else {
        println!();
        println!("{}", format_models_table(models));
    }

    Ok(())
}

fn execute_remove_model(name: &str) -> anyhow::Result<()> {
    let mut store = ConfigStore::new()?;
    store.remove_model(name)?;

    print_success(&format!("模型配置已删除: {}", name));

    Ok(())
}

fn execute_edit_model(
    name: &str,
    base_url: Option<&String>,
    api_key: Option<&String>,
    model: Option<&String>,
) -> anyhow::Result<()> {
    let mut store = ConfigStore::new()?;

    if base_url.is_none() && api_key.is_none() && model.is_none() {
        print_warning("没有指定任何要更新的字段");
        print_info("使用 --base-url, --api-key, 或 --model 指定要更新的字段");
        return Ok(());
    }

    store.edit_model(name, |model_config| {
        if let Some(url) = base_url {
            validate_url(url)?;
            model_config.base_url = url.clone();
        }

        if let Some(key) = api_key {
            model_config.api_key = key.clone();
        }

        if let Some(model_id) = model {
            model_config.model_id = model_id.clone();
        }

        Ok(())
    })?;

    print_success(&format!("模型配置已更新: {}", name));

    Ok(())
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
        "提示: 使用 'asw switch <agent> <model>' 切换工具的模型配置".cyan()
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
    let active_models = store.get_all_active_models();

    let adapters = all_adapters();

    for adapter in adapters {
        let name = adapter.name();
        let is_installed = adapter.detect()?;
        let config_path = adapter.config_path()?;
        let _config_exists = config_path.exists();

        let model_name = active_models.get(name).cloned();
        let status = if model_name.is_some() {
            "✓".green()
        } else if is_installed {
            "⚠".yellow()
        } else {
            "✗".red()
        };

        let model_text = model_name.unwrap_or_else(|| {
            if is_installed {
                "-".to_string()
            } else {
                "".to_string()
            }
        });

        println!(
            "{:<20} {:<15} {:<40} {}",
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
        "提示: 使用 'asw switch <agent> <model>' 配置工具".cyan()
    );

    Ok(())
}

/// 执行 switch 命令
pub fn execute_switch(agent: &str, model: &str) -> anyhow::Result<()> {
    // 验证模型配置是否存在
    let mut store = ConfigStore::new()?;
    let model_config = store
        .list_models()
        .iter()
        .find(|m| m.name == model)
        .ok_or_else(|| anyhow::anyhow!("模型配置 '{}' 不存在", model))?
        .clone();

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
        format!("正在切换 {} 到 {} 模型...", agent, model).cyan()
    );
    adapter.apply(&model_config)?;
    println!("{}", format!("✓ {} 已切换到 {} 模型", agent, model).green());

    // 步骤 3: 更新 active_models 映射
    store.update_active_model(agent, model)?;

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
