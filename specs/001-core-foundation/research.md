# 技术研究: AgentSwitch 核心基础功能

**功能**: 核心基础功能
**日期**: 2026-02-27
**状态**: ✅ 完成

## 研究目标

探索 Rust 生态系统中用于 CLI 工具开发的最佳实践和成熟库，为架构设计提供技术依据。

## 研究问题

### RQ1: 如何选择 CLI 框架？

**问题**: Rust 生态中有多个 CLI 框架，如何选择最适合的？

**候选方案**:

1. **clap 4.x (derive 特性)** ⭐ 选择
   - GitHub: https://github.com/clap-rs/clap
   - Stars: 13k+
   - 维护状态: 活跃
   - 优点:
     - 声明式 API，通过 derive 宏定义
     - 类型安全，编译时检查
     - 自动生成 `--help` 和 `--version`
     - 支持子命令、参数验证、自动补全
   - 缺点:
     - derive 宏增加编译时间
   - 使用示例:
     ```rust
     #[derive(Parser)]
     struct Cli {
         #[arg(short, long)]
         name: String,
     }
     ```

2. **clap 4.x (builder 模式)**
   - 优点: 运行时构建、灵活性高
   - 缺点: 代码冗长、类型安全性较弱

3. **pico-args**
   - 优点: 零依赖、极轻量
   - 缺点: 功能有限、需手动解析

**决策**: 选择 **clap 4.x (derive 特性)**

**理由**:
- clap 是 Rust CLI 的事实标准，被广泛采用（ripgrep、bat 等）
- derive 特性提供类型安全和编译时验证
- 自动生成文档，减少维护成本
- 支持复杂场景（子命令、参数验证）

**参考**:
- [clap 文档](https://docs.rs/clap/latest/clap/)
- [Command-line apps in Rust](https://rust-cli.github.io/book/index.html)

---

### RQ2: 如何选择序列化框架？

**问题**: 配置文件需要持久化，选择什么格式和库？

**候选方案**:

1. **serde + toml** ⭐ 选择
   - GitHub: https://github.com/toml-lang/toml
   - 优点:
     - TOML 是配置文件的事实标准（Cargo、npm、git）
     - 人类可读且易编辑
     - serde 提供编译时类型检查
     - 支持注释
   - 缺点:
     - 不支持复杂嵌套
   - 使用示例:
     ```rust
     #[derive(Serialize, Deserialize)]
     struct Config {
         name: String,
         value: i32,
     }
     ```

2. **serde + JSON**
   - 优点: JSON 通用
   - 缺点: 不可读、无注释

3. **serde + YAML**
   - 优点: 支持注释
   - 缺点: 缩进敏感、解析慢

**决策**: 选择 **serde + toml**

**理由**:
- TOML 专为配置文件设计
- 社区广泛采用
- 人类可读且易于手动编辑

**参考**:
- [TOML 规范](https://toml.io/en/)
- [serde 文档](https://docs.rs/serde/)

---

### RQ3: 如何处理错误？

**问题**: CLI 应用如何优雅地处理和显示错误？

**候选方案**:

1. **anyhow** ⭐ 选择
   - GitHub: https://github.com/dtolnay/anyhow
   - 优点:
     - 简洁的错误链（`context()` 方法）
     - 自动转换（兼容 `std::error::Error`）
     - 与 `?` 操作符无缝集成
   - 缺点:
     - 类型擦除（不适合库）
   - 使用示例:
     ```rust
     use anyhow::{Result, Context};
     fn read_file() -> Result<String> {
         let content = fs::read_to_string("config.toml")
             .context("无法读取配置文件")?;
         Ok(content)
     }
     ```

2. **thiserror + anyhow**
   - 优点: thiserror 定义错误类型
   - 缺点: 增加复杂度

3. **自定义错误类型**
   - 优点: 完全控制
   - 缺点: 大量样板代码

**决策**: 选择 **anyhow**

**理由**:
- CLI 应用不需要暴露错误类型
- anyhow 的错误链功能非常适合
- 减少样板代码

**参考**:
- [anyhow 文档](https://docs.rs/anyhow/)
- [Error Handling Theory](https://blog.burntsushi.net/rust-error-handling/)

---

### RQ4: 如何实现彩色输出？

**问题**: CLI 如何提供清晰的视觉反馈？

**候选方案**:

1. **colored** ⭐ 选择
   - GitHub: https://github.com/mackwic/colored
   - 优点:
     - 极简 API（`"success".green()`）
     - 零依赖（除终端控制）
   - 缺点:
     - 功能基础
   - 使用示例:
     ```rust
     use colored::Colorize;
     println!("{} 操作成功", "✓".green());
     println!("{} 发生错误", "✗".red());
     ```

2. **termcolor**
   - 优点: 跨平台、支持 no-std
   - 缺点: API 冗长

3. **ratatui**
   - 优点: 终端 UI 框架
   - 缺点: 过度设计

**决策**: 选择 **colored**

**理由**:
- API 简洁直观
- 满足当前需求（成功/错误/警告/提示）

**参考**:
- [colored 文档](https://docs.rs/colored/)

---

### RQ5: 如何处理文件权限？

**问题**: 如何保护配置文件中的 API Key？

**技术方案**:

**Unix/Linux/macOS**:
```rust
use std::os::unix::fs::PermissionsExt;

fn set_secure_permissions(path: &Path) -> std::io::Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o600); // 仅所有者可读写
    fs::set_permissions(path, perms)?;
    Ok(())
}
```

**Windows**:
- Windows 不支持 Unix 风格权限
- 使用 ACL（访问控制列表）
- 如果设置失败，警告但继续运行

**决策**:
- 使用条件编译（`#[cfg(unix)]`）
- 权限设置失败时警告但继续（非阻塞）

**参考**:
- [std::fs::Permissions](https://doc.rust-lang.org/std/fs/struct.Permissions.html)
- [Unix Permissions](https://en.wikipedia.org/wiki/File-system_permissions)

---

### RQ6: 如何验证 URL？

**问题**: 如何确保用户提供的 URL 是合法的？

**候选方案**:

1. **url crate** ⭐ 选择
   - GitHub: https://github.com/servo/rust-url
   - 优点:
     - URL 解析的官方标准
     - 支持多种 scheme（http/https）
     - 严格验证
   - 缺点:
     - 依赖较重
   - 使用示例:
     ```rust
     use url::Url;

     fn validate_url(url_str: &str) -> anyhow::Result<()> {
         let _url = Url::parse(url_str)
             .context("URL 格式无效")?;
         Ok(())
     }
     ```

2. **正则表达式**
   - 优点: 轻量级
   - 缺点: 容易出错、不完整

**决策**: 选择 **url crate**

**理由**:
- URL 解析的标准库
- 避免手动实现正则的复杂性
- 严格验证确保安全性

**参考**:
- [url crate 文档](https://docs.rs/url/)

---

### RQ7: 如何格式化表格输出？

**问题**: `asw model list` 需要表格化显示模型配置

**候选方案**:

1. **comfy-table** ⭐ 选择
   - GitHub: https://github.com/Nukesor/comfy-table
   - 优点:
     - 现代、易用的 API
     - 支持列对齐、边框自定义
     - 支持颜色
   - 缺点:
     - 依赖较多
   - 使用示例:
     ```rust
     use comfy_table::Table;

     let mut table = Table::new();
     table.set_header(vec!["Name", "Base URL", "API Key"]);
     table.add_row(vec!["glm", "https://...", "sk12****"]);
     println!("{table}");
     ```

2. **tabled**
   - 优点: 灵活、动态列宽
   - 缺点: 学习曲线

3. **手动格式化**
   - 优点: 零依赖
   - 缺点: 维护困难

**决策**: 选择 **comfy-table**

**理由**:
- 现代 API，易于使用
- 适合 `asw model list` 的需求
- 支持对齐和边框

**参考**:
- [comfy-table 文档](https://docs.rs/comfy-table/)

---

## 技术栈总结

| 类别 | 选择 | 版本 | 理由 |
|------|------|------|------|
| CLI 框架 | clap | 4.x | 社区标准、类型安全、derive API |
| 序列化 | serde + toml | latest | 配置文件标准、人类可读 |
| 错误处理 | anyhow | latest | 简洁错误链、适合应用 |
| 彩色输出 | colored | latest | 零依赖、简单 API |
| URL 验证 | url | latest | 标准、严格验证 |
| 表格输出 | comfy-table | latest | 现代 API、美观输出 |
| 路径处理 | dirs | latest | 跨平台标准路径 |

## 参考资料

### 官方文档
- [The Rust CLI Book](https://rust-cli.github.io/book/)
- [clap Documentation](https://docs.rs/clap/)
- [serde Documentation](https://docs.rs/serde/)

### 社区资源
- [Command-line apps in Rust](https://rust-cli.github.io/book/index.html)
- [Rust CLI Workshop](https://github.com/rust-cli/cli)

### 示例项目
- [ripgrep](https://github.com/BurntSushi/ripgrep) - 搜索工具
- [bat](https://github.com/sharkdp/bat) - cat 替代品
- [eza](https://github.com/eza-community/eza) - ls 替代品

---

**研究状态**: ✅ 完成
**下一步**: 创建数据模型文档（data-model.md）
