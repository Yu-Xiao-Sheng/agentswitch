//! 彩色输出主题
//!
//! 提供彩色输出函数（成功、错误、警告、信息）

use colored::Colorize;

/// 打印成功信息
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// 打印错误信息
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// 打印警告信息
pub fn print_warning(message: &str) {
    println!("{} {}", "💡".yellow(), message);
}

/// 打印信息提示
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

/// 打印详细错误信息（带上下文）
pub fn print_detailed_error(error: &anyhow::Error, context: &str) {
    print_error(context);
    eprintln!("  详细信息: {}", error);

    // 显示错误链
    let mut source = error.source();
    let mut depth = 0;
    while let Some(err) = source {
        if depth < 2 {
            eprintln!("  原因 {}: {}", depth + 1, err);
        }
        source = err.source();
        depth += 1;
    }
}
