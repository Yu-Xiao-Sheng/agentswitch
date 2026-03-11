//! 交互式配置向导模块
use crate::utils::tty;
use colored::Colorize;
use inquire::{Confirm, Password, Text};

use super::progress::ProgressManager;
use super::{FieldType, InputField, WizardError, WizardState, WizardStep, WizardType};

/// 运行交互式配置向导
pub fn run_wizard(resume: bool, reset: bool) -> Result<(), WizardError> {
    // 检查是否为交互式终端
    if !tty::supports_interactive_input() {
        return Err(WizardError::NotInteractive);
    }

    let progress = ProgressManager::new()?;

    // 如果指定了 reset，清除保存的状态
    if reset {
        progress.clear_state()?;
        println!("{}", "已清除之前的向导进度".green());
    }

    // 尝试加载保存的状态
    let mut wizard = if resume {
        match progress.load_state()? {
            Some(state) => {
                if state.is_expired() {
                    println!("{}", "保存的向导状态已过期（超过 24 小时）".yellow());
                    Wizard::new(WizardType::InitialSetup, progress)
                } else {
                    println!(
                        "{}",
                        format!("恢复向导进度（步骤 {}）", state.current_step).green()
                    );
                    Wizard::from_state(state, progress)
                }
            }
            None => {
                println!("{}", "未找到保存的向导进度，开始新向导".yellow());
                Wizard::new(WizardType::InitialSetup, progress)
            }
        }
    } else {
        Wizard::new(WizardType::InitialSetup, progress)
    };

    // 运行向导
    wizard.run()?;

    Ok(())
}

/// 向导执行器
struct Wizard {
    state: WizardState,
    progress: ProgressManager,
    steps: Vec<WizardStep>,
}

impl Wizard {
    fn new(wizard_type: WizardType, progress: ProgressManager) -> Self {
        let state = WizardState::new(wizard_type.clone());
        let steps = Self::create_steps(&wizard_type);

        Self {
            state,
            progress,
            steps,
        }
    }

    fn from_state(state: WizardState, progress: ProgressManager) -> Self {
        let steps = Self::create_steps(&state.wizard_type);

        Self {
            state,
            progress,
            steps,
        }
    }

    fn create_steps(wizard_type: &WizardType) -> Vec<WizardStep> {
        match wizard_type {
            WizardType::InitialSetup => vec![
                WizardStep {
                    id: 0,
                    name: "模型配置名称".to_string(),
                    description: "为模型配置指定一个易记的名称".to_string(),
                    fields: vec![InputField {
                        name: "model_name".to_string(),
                        field_type: FieldType::Text,
                        label: "模型配置名称".to_string(),
                        help_text: Some("例如: glm, gpt-4, minimax".to_string()),
                        default: None,
                        validators: vec![],
                    }],
                    optional: false,
                },
                WizardStep {
                    id: 1,
                    name: "Base URL".to_string(),
                    description: "模型 API 的 Base URL".to_string(),
                    fields: vec![InputField {
                        name: "base_url".to_string(),
                        field_type: FieldType::Text,
                        label: "Base URL".to_string(),
                        help_text: Some("例如: https://open.bigmodel.cn/api/v1".to_string()),
                        default: None,
                        validators: vec![],
                    }],
                    optional: false,
                },
                WizardStep {
                    id: 2,
                    name: "API Key".to_string(),
                    description: "模型的 API Key".to_string(),
                    fields: vec![InputField {
                        name: "api_key".to_string(),
                        field_type: FieldType::Password,
                        label: "API Key".to_string(),
                        help_text: Some("输入您的 API Key（至少 32 个字符）".to_string()),
                        default: None,
                        validators: vec![],
                    }],
                    optional: false,
                },
                WizardStep {
                    id: 3,
                    name: "Model ID".to_string(),
                    description: "要使用的模型 ID".to_string(),
                    fields: vec![InputField {
                        name: "model_id".to_string(),
                        field_type: FieldType::Text,
                        label: "Model ID".to_string(),
                        help_text: Some("例如: glm-4, gpt-4, abab6.5s-chat".to_string()),
                        default: None,
                        validators: vec![],
                    }],
                    optional: false,
                },
            ],
            WizardType::AddModel => vec![
                // 添加单个模型的步骤
                WizardStep {
                    id: 0,
                    name: "添加模型配置".to_string(),
                    description: "添加新的模型配置".to_string(),
                    fields: vec![],
                    optional: false,
                },
            ],
            WizardType::BatchSetup => vec![],
        }
    }

    fn run(&mut self) -> Result<(), WizardError> {
        println!();
        println!(
            "{}",
            "Welcome to AgentSwitch configuration wizard!"
                .green()
                .bold()
        );
        println!();
        println!("This wizard will guide you through setting up your first model configuration.");
        println!();
        println!("Press Ctrl+C at any time to exit (progress will be saved).");
        println!();

        // 执行每个步骤
        let steps: Vec<_> = self.steps.iter().cloned().collect();
        for (index, step) in steps.iter().enumerate() {
            println!(
                "{}",
                format!("\n=== Step {}: {} ===", index + 1, step.name).cyan()
            );
            println!("{}", step.description);

            // 执行步骤
            self.execute_step(step)?;

            // 标记步骤完成
            self.state.completed_steps.push(step.id);
            self.state.current_step = index + 1;

            // 保存进度
            self.progress.save_state(&self.state)?;
        }

        // 显示配置摘要
        self.display_summary()?;

        // 确认保存
        let save = Confirm::new("是否保存此配置?")
            .with_default(true)
            .prompt()?;

        if save {
            self.save_config()?;
            println!();
            println!("{}", "✓ Configuration saved successfully!".green().bold());
            println!();
            println!("{}", "Next steps:".cyan());
            println!("  - Run 'asw model list' to see all configured models");
            println!("  - Run 'asw doctor' to detect installed tools");
            println!("  - Run 'asw switch <tool> <model>' to apply a model to a tool");
        } else {
            println!("{}", "配置未保存".yellow());
        }

        // 清除保存的状态
        self.progress.clear_state()?;

        Ok(())
    }

    fn execute_step(&mut self, step: &WizardStep) -> Result<(), WizardError> {
        for field in &step.fields {
            let value = self.prompt_field(field)?;
            self.state.data.insert(field.name.clone(), value);
        }
        Ok(())
    }

    fn prompt_field(&self, field: &InputField) -> Result<String, WizardError> {
        let value = match &field.field_type {
            FieldType::Text => Text::new(&field.label)
                .with_help_message(field.help_text.as_deref().unwrap_or(""))
                .with_default(field.default.as_deref().unwrap_or(""))
                .prompt()?,
            FieldType::Password => {
                let key = Password::new(&field.label)
                    .with_help_message(field.help_text.as_deref().unwrap_or(""))
                    .prompt()?;

                // 显示掩码后的 API Key
                if key.len() > 8 {
                    let masked = format!("sk-***{}", &key[key.len() - 4..]);
                    println!("{}", format!("  API Key: {}", masked).dimmed());
                }

                key
            }
            _ => {
                // 简化实现，其他类型暂不支持
                Text::new(&field.label).prompt()?
            }
        };

        Ok(value)
    }

    fn display_summary(&self) -> Result<(), WizardError> {
        println!();
        println!("{}", "Configuration summary:".cyan().bold());
        println!();

        if let Some(name) = self.state.data.get("model_name") {
            println!("  Name: {}", name.green());
        }
        if let Some(url) = self.state.data.get("base_url") {
            println!("  Base URL: {}", url.cyan());
        }
        if let Some(key) = self.state.data.get("api_key") {
            let masked = if key.len() > 8 {
                format!("sk-***{}", &key[key.len() - 4..])
            } else {
                "sk-***".to_string()
            };
            println!("  API Key: {}", masked.yellow());
        }
        if let Some(model_id) = self.state.data.get("model_id") {
            println!("  Model ID: {}", model_id.cyan());
        }

        Ok(())
    }

    fn save_config(&self) -> Result<(), WizardError> {
        use crate::config::ConfigStore;
        use crate::config::ModelConfig;

        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("无法找到主目录"))?
            .join(".agentswitch");

        // 创建配置目录
        std::fs::create_dir_all(&config_dir)?;

        // 创建 ModelConfig
        let model_config = ModelConfig {
            name: self
                .state
                .data
                .get("model_name")
                .ok_or_else(|| WizardError::Config("模型名称未设置".to_string()))?
                .clone(),
            base_url: self
                .state
                .data
                .get("base_url")
                .ok_or_else(|| WizardError::Config("Base URL 未设置".to_string()))?
                .clone(),
            api_key: self
                .state
                .data
                .get("api_key")
                .ok_or_else(|| WizardError::Config("API Key 未设置".to_string()))?
                .clone(),
            model_id: self
                .state
                .data
                .get("model_id")
                .ok_or_else(|| WizardError::Config("Model ID 未设置".to_string()))?
                .clone(),
            extra_params: None,
        };

        // 保存配置
        let mut store = ConfigStore::new()?;
        store.add_model(model_config)?;

        Ok(())
    }
}
