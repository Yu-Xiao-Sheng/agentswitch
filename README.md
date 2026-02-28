# AgentSwitch

> 一个通用的代码终端代理工具配置切换器，支持将任意 OpenAI 协议模型接入到主流 Code Agent 工具中。

[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📖 项目简介

当前市面上存在众多优秀的代码终端代理工具（Code Agent Tools），如：
- **Claude Code** - Anthropic 官方 CLI 工具
- **Codex** - OpenAI 的代码助手
- **Gemini CLI** - Google 的代码助手
- **OpenCode** - 开源代码助手
- **Qwen CLI** - 阿里通义千问 CLI
- **Grok CLI** - xAI 的代码助手

这些工具虽然原生支持各自厂商的模型，但大多数也支持任意兼容 OpenAI 协议的 API。本工具旨在提供一个统一的配置管理界面，让用户可以轻松地在这些工具间切换不同的模型提供商，实现"一次配置，处处使用"的便捷体验。

## 🎯 核心功能

### 1. 模型配置统一管理
- 集中管理多个模型厂商的 API 配置
- 支持添加、编辑、删除模型提供商配置
- 安全地存储 API Key 和配置信息

### 2. Code Agent 工具支持（计划中）
自动检测并适配已安装的 Code Agent 工具，修改其配置文件以使用指定的模型：
- `~/.codex/config.toml`
- `~/.claude/config.json`
- `~/.gemini-cli/config.yaml`
- 其他工具的配置文件

### 3. 配置切换与对比（计划中）
快速在不同工具间切换模型配置，方便对比不同模型在同一工具下的表现：
```bash
# 将 Codex 切换到 GLM-4
asw switch codex glm-4

# 将 Claude Code 切换到 MiniMax
asw switch claude-code minimax
```

### 4. 配置文件备份与恢复（计划中）
在修改配置前自动备份原配置，支持一键恢复。

## 🗺️ 项目蓝图

### Phase 1: 核心基础 ✅ (已完成 - v0.1.0)
- [x] 项目架构搭建
- [x] **配置存储模块**
  - [x] 定义统一的模型配置数据结构
  - [x] 实现配置文件的读写（使用 `~/.agentswitch/config.toml`）
  - [x] API Key 的安全存储（掩码显示 + 文件权限 0600）
- [x] **CLI 框架**
  - [x] 基础命令结构设计（add/list/remove/edit 等）
  - [x] 彩色输出和错误提示
- [x] **输入验证**
  - [x] URL 格式验证（http/https）
  - [x] 模型名称规范检查

### Phase 2: 模型配置管理 ✅ (已完成 - v0.1.0)
- [x] **添加模型配置** (`asw model add`)
  - [x] 支持配置：名称、Base URL、API Key、Model ID
  - [x] 配置验证（URL、模型名称）
- [x] **列出模型配置** (`asw model list`)
  - [x] 显示所有已配置的模型
  - [x] API Key 掩码显示
- [x] **删除模型配置** (`asw model remove`)
- [x] **编辑模型配置** (`asw model edit`)

### Phase 3: Agent 工具适配器 (P1 - 计划中)
- [ ] **适配器接口设计**
  - [ ] 定义统一的 `AgentAdapter` trait
  - [ ] 方法：`detect()`, `backup()`, `apply()`, `restore()`
- [ ] **Claude Code 适配器**
  - [ ] 检测安装路径
  - [ ] 解析/修改配置文件
- [ ] **Codex 适配器**
  - [ ] TOML 配置解析
  - [ ] 配置项映射
- [ ] **Gemini CLI 适配器**
  - [ ] YAML 配置解析
- [ ] **其他工具适配器** (Qwen, Grok, OpenCode 等)

### Phase 4: 配置切换功能 (P1)
- [ ] **切换命令** (`asw switch <agent> <model>`)
  - [ ] 自动备份原配置
  - [ ] 应用新配置
  - [ ] 验证配置生效
- [ ] **状态显示** (`asw status`)
  - [ ] 显示所有工具当前使用的模型
  - [ ] 显示配置文件路径

### Phase 5: 高级功能 (P2)
- [ ] **配置预设** (Presets)
  - [ ] 保存常用的配置组合
  - [ ] 一键应用预设配置
- [ ] **批量切换**
  - [ ] 同时切换多个工具到同一模型
- [ ] **配置导入/导出**
  - [ ] 支持配置文件的分享和迁移
- [ ] **自动更新检测**
  - [ ] 检测新版本
  - [ ] 支持配置迁移

### Phase 6: 用户体验优化 (P2)
- [ ] **交互式配置向导**
- [ ] **自动发现已安装的 Agent 工具**
- [ ] **配置验证与错误提示**
- [ ] **Shell 自动补全** (Bash/Zsh/Fish)
- [ ] **丰富的文档和示例**

### Phase 7: 扩展功能 (P3)
- [ ] **配置同步**
  - [ ] 通过 Git 同步配置
  - [ ] 多机器配置共享
- [ ] **性能测试模式**
  - [ ] 在不同模型上运行相同的测试用例
  - [ ] 生成对比报告
- [ ] **Web Dashboard** (可选)
  - [ ] 可视化配置管理界面
- [ ] **插件系统**
  - [ ] 支持社区贡献的适配器

## 📦 安装

### 从源码编译
```bash
# 克隆仓库
git clone https://github.com/Yu-Xiao-Sheng/agentswitch.git
cd agentswitch

# 编译安装
cargo build --release

# 或使用 cargo install
cargo install --path .
```

### 使用预编译二进制
访问 [Releases](https://github.com/Yu-Xiao-Sheng/agentswitch/releases) 页面下载最新版本。

## 🚀 快速开始

### 1. 自动初始化
首次运行任何命令时，AgentSwitch 会自动创建配置目录 `~/.agentswitch/`。

### 2. 添加模型配置
```bash
# 添加 GLM 模型配置
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-..." \
  --model "glm-4"

# 添加 MiniMax 配置
asw model add minimax \
  --base-url "https://api.minimax.chat/v1" \
  --api-key "your-api-key" \
  --model "abab6.5s-chat"
```

### 3. 列出所有模型
```bash
asw model list
```

### 4. 编辑模型配置
```bash
asw model edit glm --model "glm-4-turbo"
```

### 5. 删除模型配置
```bash
asw model remove glm
```

## 📋 命令参考

### 当前可用命令（v0.1.0）
```
asw
└── model             # 模型配置管理
    ├── add          # 添加模型配置
    ├── list         # 列出所有模型
    ├── remove       # 删除模型配置
    └── edit         # 编辑模型配置
```

### 计划中的命令
```
asw
├── init              # 初始化配置（现已自动初始化）
├── model             # 模型配置管理
├── agent             # Agent 工具管理
│   ├── list         # 列出已安装的工具
│   └── detect       # 检测工具安装状态
├── switch            # 切换工具的模型配置
├── status            # 显示当前配置状态
├── backup            # 配置备份管理
│   ├── list         # 列出备份
│   ├── restore      # 恢复备份
│   └── clean        # 清理旧备份
└── preset            # 配置预设管理
    ├── save         # 保存当前配置为预设
    ├── list         # 列出所有预设
    └── apply        # 应用预设配置
```

## 🛠️ 技术架构

### 目录结构
```
agentswitch/
├── src/
│   ├── main.rs              # 程序入口
│   ├── cli/                 # CLI 命令定义
│   │   ├── mod.rs
│   │   └── commands.rs      # 命令实现
│   ├── config/              # 配置管理
│   │   ├── mod.rs
│   │   ├── store.rs         # 配置存储
│   │   └── models.rs        # 数据模型
│   ├── output/              # 输出格式化
│   │   ├── mod.rs
│   │   ├── formatter.rs     # 表格格式化
│   │   └── theme.rs         # 彩色输出
│   ├── utils/               # 工具函数
│   │   ├── mod.rs
│   │   ├── validation.rs    # 输入验证
│   │   └── permissions.rs   # 文件权限
│   └── agents/              # Agent 适配器（计划中）
│       ├── mod.rs
│       ├── adapter.rs       # 适配器 trait
│       ├── claude_code.rs   # Claude Code 适配器
│       ├── codex.rs         # Codex 适配器
│       └── gemini.rs        # Gemini CLI 适配器
├── tests/                   # 集成测试
│   └── integration/
├── Cargo.toml
├── README.md
└── LICENSE
```

### 核心设计

#### AgentAdapter Trait（计划中）
```rust
pub trait AgentAdapter {
    fn name(&self) -> &str;
    fn detect(&self) -> anyhow::Result<bool>;
    fn config_path(&self) -> anyhow::Result<PathBuf>;
    fn backup(&self) -> anyhow::Result<Backup>;
    fn apply(&self, model_config: &ModelConfig) -> anyhow::Result<()>;
    fn restore(&self, backup: &Backup) -> anyhow::Result<()>;
    fn current_model(&self) -> anyhow::Result<Option<String>>;
}
```

#### ModelConfig 结构（已实现）
```rust
pub struct ModelConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub model_id: String,
    pub extra_params: Option<HashMap<String, serde_json::Value>>,
}
```

## 🔧 开发

```bash
# 构建
cargo build

# 测试
cargo test

# 代码检查
cargo clippy
cargo fmt

# 运行
cargo run -- model list
```

## 📊 版本历史

### v0.1.0 (2026-02-27)
- ✨ 首次发布
- ✅ 完成配置存储模块
- ✅ 实现 CRUD 命令（add/list/remove/edit）
- ✅ API Key 安全保护
- ✅ 输入验证
- ✅ 彩色输出

## 🤝 贡献指南

欢迎贡献！

### 贡献方式
1. 报告 Bug
2. 提出新功能建议
3. 提交代码
4. 改进文档
5. 添加新的 Agent 适配器

## 📄 开源协议

本项目采用 [MIT License](LICENSE) 开源协议。

## 🔗 相关资源

- [项目文档](specs/001-core-foundation/)
- [Issue Tracker](https://github.com/Yu-Xiao-Sheng/agentswitch/issues)
- [Claude Code](https://github.com/anthropics/claude-code)
- [OpenAI API](https://platform.openai.com/docs/api-reference)

## 📮 联系方式

- GitHub: [@Yu-Xiao-Sheng](https://github.com/Yu-Xiao-Sheng)
- Issue: [提交问题](https://github.com/Yu-Xiao-Sheng/agentswitch/issues)

---

⭐ 如果这个项目对你有帮助，请给一个 Star！
