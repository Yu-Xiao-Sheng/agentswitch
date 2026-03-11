use std::io::IsTerminal;

/// 检测是否为交互式终端
pub fn is_interactive_terminal() -> bool {
    std::io::stdin().is_terminal()
}

/// 检测是否支持交互式输入
pub fn supports_interactive_input() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}
