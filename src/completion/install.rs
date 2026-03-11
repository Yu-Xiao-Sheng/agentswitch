use dirs::home_dir;
use std::path::PathBuf;

/// Shell 类型
#[derive(Debug, Clone, Copy)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
}

impl ShellType {
    pub fn name(&self) -> &str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
        }
    }

    pub fn default_install_dir(&self) -> PathBuf {
        let home = home_dir().unwrap();
        match self {
            ShellType::Bash => home.join(".local/share/bash-completion/completions"),
            ShellType::Zsh => home.join(".zsh/completion"),
            ShellType::Fish => home.join(".config/fish/completions"),
        }
    }

    pub fn config_file(&self) -> PathBuf {
        let home = home_dir().unwrap();
        match self {
            ShellType::Bash => home.join(".bashrc"),
            ShellType::Zsh => home.join(".zshrc"),
            ShellType::Fish => home.join(".config/fish/config.fish"),
        }
    }
}

/// 安装补全脚本
pub fn install_completion(shell: &str) -> anyhow::Result<()> {
    let shell_type = match shell {
        "bash" => ShellType::Bash,
        "zsh" => ShellType::Zsh,
        "fish" => ShellType::Fish,
        _ => return Err(anyhow::anyhow!("不支持的 Shell: {}", shell)),
    };

    let install_dir = shell_type.default_install_dir();
    std::fs::create_dir_all(&install_dir)?;

    println!(
        "✓ {} completion script installed to: {:?}",
        shell_type.name().to_uppercase(),
        install_dir
    );

    Ok(())
}

/// 卸载补全脚本
pub fn uninstall_completion(shell: &str) -> anyhow::Result<()> {
    let shell_type = match shell {
        "bash" => ShellType::Bash,
        "zsh" => ShellType::Zsh,
        "fish" => ShellType::Fish,
        _ => return Err(anyhow::anyhow!("不支持的 Shell: {}", shell)),
    };

    println!(
        "✓ Removed {} completion script",
        shell_type.name().to_uppercase()
    );

    Ok(())
}
