//! 输出格式化工具

use crate::config::models::ModelConfig;
use crate::config::provider::Provider;

/// 掩码 API Key，仅显示前 4 个字符
pub fn mask_api_key(api_key: &str) -> String {
    if api_key.len() <= 4 {
        "****".to_string()
    } else {
        format!("{}****", &api_key[..4])
    }
}

/// 格式化供应商列表为表格
pub fn format_providers_table(providers: &[Provider]) -> String {
    if providers.is_empty() {
        return "💡 当前没有配置任何供应商\n  提示: 使用 'asw provider add <name>' 添加供应商"
            .to_string();
    }

    let mut output = String::new();
    output.push_str(
        "┌──────────────────┬──────────────────────────────┬────────────┬──────────┬──────────────────────┐\n",
    );
    output.push_str(
        "│ Name             │ Base URL                     │ Protocol   │ API Key  │ Models               │\n",
    );
    output.push_str(
        "├──────────────────┼──────────────────────────────┼────────────┼──────────┼──────────────────────┤\n",
    );

    for provider in providers {
        let name = if provider.name.len() > 16 {
            format!("{}...", &provider.name[..13])
        } else {
            format!("{:<16}", provider.name)
        };
        let base_url = if provider.base_url.len() > 28 {
            format!("{}...", &provider.base_url[..25])
        } else {
            format!("{:<28}", provider.base_url)
        };
        let protocol = format!("{:<10}", provider.protocol.as_str());
        let api_key = mask_api_key(&provider.api_key);
        let models_str = if provider.models.len() <= 2 {
            provider.models.join(", ")
        } else {
            format!(
                "{}, ...({} total)",
                provider.models[0],
                provider.models.len()
            )
        };
        let models_display = if models_str.len() > 20 {
            format!("{}...", &models_str[..17])
        } else {
            format!("{:<20}", models_str)
        };

        output.push_str(&format!(
            "│ {} │ {} │ {} │ {} │ {} │\n",
            name, base_url, protocol, api_key, models_display
        ));
    }

    output.push_str(
        "└──────────────────┴──────────────────────────────┴────────────┴──────────┴──────────────────────┘",
    );

    output
}

/// 格式化模型列表为表格
pub fn format_models_table(models: &[ModelConfig]) -> String {
    if models.is_empty() {
        return "💡 当前没有配置任何模型\n  提示: 使用 'asw model add <name>' 添加模型配置"
            .to_string();
    }

    let mut output = String::new();
    output.push_str("┌─────────────────────────────────────────────────────────────┐\n");
    output.push_str(
        "│ Name                      │ Base URL                    │ Model ID    │ API Key  │\n",
    );
    output.push_str(
        "├───────────────────────────┼────────────────────────────┼─────────────┼──────────┤\n",
    );

    for model in models {
        let name = format!("{:<25}", model.name);
        let base_url = if model.base_url.len() > 26 {
            format!("{}...", &model.base_url[..23])
        } else {
            format!("{:<26}", model.base_url)
        };
        let model_id = format!("{:<11}", model.get_default_model().unwrap_or("N/A"));
        let api_key = mask_api_key(&model.api_key);

        output.push_str(&format!(
            "│ {} │ {} │ {} │ {} │\n",
            name, base_url, model_id, api_key
        ));
    }

    output.push_str("└─────────────────────────────────────────────────────────────┘");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_api_key_short() {
        assert_eq!(mask_api_key("sk12"), "****");
    }

    #[test]
    fn test_mask_api_key_long() {
        assert_eq!(mask_api_key("sk-abc123def456"), "sk-a****");
    }
}
