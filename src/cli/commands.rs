//! CLI 命令实现

use crate::config::{ConfigStore, ModelConfig};
use crate::output::{format_models_table, print_info, print_success, print_warning};
use crate::utils::{validate_model_name, validate_url};

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

impl ModelCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            ModelCommands::Add {
                name,
                base_url,
                api_key,
                model,
            } => execute_add_model(&name, &base_url, &api_key, &model),
            ModelCommands::List => execute_list_models(),
            ModelCommands::Remove { name } => execute_remove_model(&name),
            ModelCommands::Edit {
                name,
                base_url,
                api_key,
                model,
            } => execute_edit_model(&name, base_url.as_ref(), api_key.as_ref(), model.as_ref()),
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
