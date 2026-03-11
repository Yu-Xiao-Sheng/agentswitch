use crate::cli::Cli;
use clap_complete::{generate, shells::Bash, shells::Fish, shells::Zsh};

/// 生成补全脚本
pub fn generate_completion(shell: &str, bin_name: &str) -> anyhow::Result<String> {
    let mut cmd = Cli::command();
    let mut buffer = Vec::new();

    match shell {
        "bash" => {
            generate(Bash, &mut cmd, bin_name, &mut buffer);
        }
        "zsh" => {
            generate(Zsh, &mut cmd, bin_name, &mut buffer);
        }
        "fish" => {
            generate(Fish, &mut cmd, bin_name, &mut buffer);
        }
        _ => return Err(anyhow::anyhow!("不支持的 Shell: {}", shell)),
    };

    Ok(String::from_utf8(buffer)?)
}
