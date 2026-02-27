//! 输入验证工具

use anyhow::bail;

pub fn validate_url(url_str: &str) -> anyhow::Result<()> {
    let parsed_url =
        url::Url::parse(url_str).map_err(|e| anyhow::anyhow!("URL 格式无效: {}", e))?;

    match parsed_url.scheme() {
        "http" | "https" => Ok(()),
        scheme => bail!("不支持的 URL scheme: '{}'，仅支持 http 和 https", scheme),
    }
}

pub fn validate_model_name(name: &str) -> anyhow::Result<()> {
    if name.trim().is_empty() {
        bail!("模型名称不能为空");
    }

    if name.len() > 100 {
        bail!("模型名称过长（最大 100 字符）");
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        bail!("模型名称只能包含字母、数字、连字符和下划线");
    }

    if name.starts_with('-') || name.starts_with('_') || name.ends_with('-') || name.ends_with('_')
    {
        bail!("模型名称不能以连字符或下划线开头或结尾");
    }

    Ok(())
}
