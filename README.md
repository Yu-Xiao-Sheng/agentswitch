# AgentSwitch

> 一个通用的 Code Agent CLI 工具配置切换器，支持将任意 OpenAI 或 Anthropic 协议模型接入到主流 Code Agent 工具中。

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

## 🎯 支持的工具

| 工具 | 支持状态 | 支持的协议 |
|------|---------|-----------|
| **claude-code** | ✅ 支持 | Anthropic `/v1/messages` |
| **opencode** | ✅ 支持 | OpenAI `/v1/chat/completions` + Anthropic `/v1/messages` |
| **gemini-cli** | ✅ 支持 | OpenAI `/v1/chat/completions` |
| **qwen-cli** | ✅ 支持 | OpenAI `/v1/chat/completions` |
| **grok-cli** | ✅ 支持 | OpenAI `/v1/chat/completions` |
| **codex** | ❌ 暂不支持 | 使用 Response API，兼容性问题 |

### 协议兼容说明

- **OpenAI 兼容协议**: `/v1/chat/completions` - 大多数国内模型厂商（智谱、百川、MiniMax 等）支持
- **Anthropic 兼容协议**: `/v1/messages` - Claude Code 专用协议

> ⚠️ **Codex 说明**: Codex 使用 OpenAI Response API（非标准 chat/completions 协议），与自定义供应商兼容性较差，暂不支持切换。如需使用自定义供应商，推荐使用 opencode 或 gemini-cli。

## 🎯 核心功能

### 1. 模型配置统一管理
- 集中管理多个模型厂商的 API 配置
- 支持添加、编辑、删除模型提供商配置
- 安全地存储 API Key 和配置信息

### 2. Code Agent 工具支持 ✅ (已完成 - v0.2.0)
自动检测并适配已安装的 Code Agent 工具，修改其配置文件以使用指定的模型：

| 工具 | 配置文件路径 | 协议类型 |
|------|-------------|---------|
| Claude Code | `~/.claude/settings.json` | Anthropic |
| OpenCode | `~/.config/opencode/opencode.json` | OpenAI + Anthropic |
| Gemini CLI | `~/.gemini/settings.json` | OpenAI |
| Qwen CLI | `~/.qwen-cli/config.json` | OpenAI |
| Grok CLI | `~/.grok-cli/config.toml` | OpenAI |

> **注意**: Codex 使用 Response API，暂不支持自定义供应商切换。

### 3. 配置切换与对比 ✅ (已完成 - v0.2.0)
快速在不同工具间切换模型配置，方便对比不同模型在同一工具下的表现：
```bash
# 将 opencode 切换到 GLM-4
asw switch opencode glm

# 将 claude-code 切换到自定义 Anthropic 兼容模型
asw switch claude-code my-claude-model

# 将 gemini-cli 切换到 DeepSeek
asw switch gemini-cli deepseek
```

### 4. 配置文件备份与恢复 ✅ (已完成 - v0.2.0)
在修改配置前自动备份原配置，支持一键恢复：
```bash
# 查看所有备份
asw backup list

# 恢复备份
asw backup restore claude-code --backup 20260227-101533

# 清理旧备份
asw backup clean --older-than 7d
```

### 5. 配置预设管理 ✅ (已完成 - v0.3.0)
保存常用的配置组合，一键应用预设配置：
```bash
# 保存当前配置为预设
asw preset save my-work --tags "daily,llm" --agents "claude-code:glm,codex:glm-4"

# 列出所有预设
asw preset list

# 应用预设
asw preset apply my-work

# 验证预设
asw preset validate my-work
```

### 6. 批量操作 ✅ (已完成 - v0.3.0)
同时切换多个工具到同一模型：
```bash
# 批量切换所有工具到 GLM-4
asw batch switch glm

# 批量验证配置
asw batch validate
```

### 7. 交互式配置向导 ✅ (已完成 - v0.4.0)
友好的 CLI 交互式向导，引导新用户完成初始化配置：
```bash
# 启动向导
asw wizard init

# 恢复向导进度
asw wizard init --resume

# 重新开始
asw wizard init --reset
```

### 8. 工具诊断 ✅ (已完成 - v0.4.0)
自动检测系统中已安装的 Code Agent 工具：
```bash
# 运行完整诊断
asw doctor

# 检测已安装工具（简化版）
asw doctor detect
```

### 9. Shell 自动补全 ✅ (已完成 - v0.4.0)
为 Bash、Zsh、Fish 提供智能补全：
```bash
# 安装补全
asw completion install bash

# 生成补全脚本
asw completion generate bash > /tmp/bash_completion.sh
```

### 10. 配置同步 (Git) ✅ (已完成 - v0.4.0)
通过 Git 同步配置，支持多机器配置共享：
```bash
# 初始化 Git 仓库
asw sync init

# 推送到远程
asw sync push

# 从远程拉取
asw sync pull

# 查看同步状态
asw sync status
```

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

### Phase 3: Agent 工具适配器 ✅ (已完成 - v0.2.0)
- [x] **适配器接口设计**
  - [x] 定义统一的 `AgentAdapter` trait
  - [x] 方法：`detect()`, `backup()`, `apply()`, `restore()`, `current_model()`
- [x] **Claude Code 适配器**
  - [x] 检测安装路径
  - [x] 解析/修改配置文件
- [x] **Codex 适配器**
  - [x] TOML + JSON 配置解析
  - [x] 配置项映射
- [x] **Gemini CLI 适配器**
  - [x] JSON + .env 配置解析
- [x] **其他工具适配器** (Qwen, Grok)

### Phase 4: 配置切换功能 ✅ (已完成 - v0.2.0)
- [x] **切换命令** (`asw switch <agent> <model>`)
  - [x] 自动备份原配置
  - [x] 应用新配置
  - [x] 环境变量警告
- [x] **状态显示** (`asw status`)
  - [x] 显示所有工具当前使用的模型
  - [x] 显示配置文件路径
- [x] **备份管理**
  - [x] `asw backup list` - 列出所有备份
  - [x] `asw backup restore` - 恢复备份
  - [x] `asw backup clean` - 清理旧备份

### Phase 5: 高级功能 ✅ (已完成 - v0.3.0)
- [x] **配置预设** (Presets)
  - [x] 保存常用的配置组合
  - [x] 一键应用预设配置
- [x] **批量切换**
  - [x] 同时切换多个工具到同一模型
- [x] **配置导入/导出**
  - [x] 支持配置文件的分享和迁移
- [ ] **自动更新检测**
  - [ ] 检测新版本
  - [ ] 支持配置迁移

### Phase 6: 用户体验优化 ✅ (已完成 - v0.4.0)
- [x] **交互式配置向导**
- [x] **自动发现已安装的 Agent 工具**
- [x] **配置验证与错误提示**
- [x] **Shell 自动补全** (Bash/Zsh/Fish)
- [x] **丰富的文档和示例**

### Phase 7: 扩展功能 (部分完成 - v0.4.0)
- [x] **配置同步** ✅ (已完成 - v0.4.0)
  - [x] 通过 Git 同步配置
  - [x] 多机器配置共享
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
# 添加智谱 GLM 模型配置
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/paas/v4" \
  --api-key "sk-..." \
  --model "glm-4"

# 添加京东云模型配置
asw model add jdcloud \
  --base-url "https://aiapi.jdcloud.com/v1" \
  --api-key "your-api-key" \
  --model "glm-4"

# 添加 MiniMax 配置
asw model add minimax \
  --base-url "https://api.minimax.chat/v1" \
  --api-key "your-api-key" \
  --model "abab6.5s-chat"

# 添加 DeepSeek 配置
asw model add deepseek \
  --base-url "https://api.deepseek.com/v1" \
  --api-key "your-api-key" \
  --model "deepseek-chat"
```

### 3. 切换工具配置
```bash
# 将 opencode 切换到 GLM-4
asw switch opencode glm

# 将 claude-code 切换到自定义 Anthropic 兼容模型
asw switch claude-code my-claude-model

# 将 gemini-cli 切换到 DeepSeek
asw switch gemini-cli deepseek
```

### 4. 查看状态
```bash
# 查看所有工具当前使用的模型
asw status

# 列出所有已配置的模型
asw model list
```

### 5. 配置管理
```bash
# 编辑模型配置
asw model edit glm --model "glm-4-turbo"

# 删除模型配置
asw model remove glm
```

## 📋 命令参考

### 当前可用命令（v0.4.0）
```
asw
├── model             # 模型配置管理
│   ├── add          # 添加模型配置
│   ├── list         # 列出所有模型
│   ├── remove       # 删除模型配置
│   └── edit         # 编辑模型配置
├── switch            # 切换工具的模型配置
├── status            # 显示当前配置状态
├── backup            # 配置备份管理
│   ├── list         # 列出备份
│   ├── restore      # 恢复备份
│   └── clean        # 清理旧备份
├── preset            # 配置预设管理
│   ├── save         # 保存当前配置为预设
│   ├── list         # 列出所有预设
│   ├── apply        # 应用预设配置
│   ├── remove       # 删除预设
│   └── validate     # 验证预设
├── batch             # 批量操作
│   ├── switch       # 批量切换所有工具
│   └── validate     # 批量验证配置
├── wizard            # 交互式向导
│   └── init         # 初始化向导
├── doctor            # 工具诊断
│   └── detect       # 检测已安装工具
├── completion        # Shell 自动补全
│   ├── generate     # 生成补全脚本
│   └── install      # 安装补全脚本
└── sync              # Git 同步
    ├── init         # 初始化 Git 仓库
    ├── status       # 查看同步状态
    ├── push         # 推送到远程
    └── pull         # 从远程拉取
```

### 计划中的命令
```
asw
├── update            # 检测并安装新版本
├── benchmark         # 性能测试模式
│   ├── run          # 运行性能测试
│   └── report       # 生成对比报告
└── web               # Web Dashboard（可选）
    └── serve        # 启动 Web 界面
```

## 📝 供应商配置示例

### 国内厂商

#### 智谱 GLM
```bash
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/paas/v4" \
  --api-key "your-zhipu-api-key" \
  --model "glm-4"
```

#### 京东云
```bash
asw model add jdcloud \
  --base-url "https://aiapi.jdcloud.com/v1" \
  --api-key "your-jdcloud-api-key" \
  --model "glm-4"
```

#### DeepSeek
```bash
asw model add deepseek \
  --base-url "https://api.deepseek.com/v1" \
  --api-key "your-deepseek-api-key" \
  --model "deepseek-chat"
```

#### MiniMax
```bash
asw model add minimax \
  --base-url "https://api.minimax.chat/v1" \
  --api-key "your-minimax-api-key" \
  --model "abab6.5s-chat"
```

#### 通义千问 (Qwen)
```bash
asw model add qwen \
  --base-url "https://dashscope.aliyuncs.com/compatible-mode/v1" \
  --api-key "your-qwen-api-key" \
  --model "qwen-turbo"
```

### 国际厂商

#### OpenAI
```bash
asw model add openai \
  --base-url "https://api.openai.com/v1" \
  --api-key "your-openai-api-key" \
  --model "gpt-4o"
```

#### Anthropic (Claude)
```bash
asw model add claude \
  --base-url "https://api.anthropic.com" \
  --api-key "your-anthropic-api-key" \
  --model "claude-sonnet-4-20250514"
```

### 本地部署

#### Ollama
```bash
asw model add ollama \
  --base-url "http://localhost:11434/v1" \
  --api-key "ollama" \
  --model "llama3"
```

#### vLLM
```bash
asw model add vllm \
  --base-url "http://localhost:8000/v1" \
  --api-key "none" \
  --model "your-model-name"
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
│   └── agents/              # Agent 适配器 ✅ (已实现)
│       ├── mod.rs
│       ├── adapter.rs       # 适配器 trait
│       ├── registry.rs      # 适配器注册表
│       ├── claude_code.rs   # Claude Code 适配器
│       ├── opencode.rs      # OpenCode 适配器
│       ├── gemini.rs        # Gemini CLI 适配器
│       ├── codex.rs         # Codex 适配器 (暂不支持)
│       ├── qwen.rs          # Qwen CLI 适配器
│       └── grok.rs          # Grok CLI 适配器
├── tests/                   # 集成测试
│   └── integration/
├── Cargo.toml
├── README.md
└── LICENSE
```

### 核心设计

#### AgentAdapter Trait（已实现）
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

### v0.4.0 (2026-03-11)
- ✨ **交互式配置向导** - 友好的 CLI 向导引导用户完成初始化配置
- ✨ **工具自动检测** - 自动发现已安装的 Code Agent 工具
- ✨ **Shell 自动补全** - 支持 Bash、Zsh、Fish 智能补全
- ✨ **Git 配置同步** - 支持多机器配置共享和版本控制
- ✅ 完成所有 Spec 004 用户体验优化功能
- ✅ 配置文件加密存储 API Key
- ✅ 增强的错误提示和配置验证

### v0.3.0 (2026-03-05)
- ✨ **配置预设管理** - 保存和一键应用常用配置组合
- ✨ **批量操作** - 同时切换多个工具到同一模型
- ✅ 完成所有 Spec 003 高级功能
- ✅ 预设标签系统便于管理
- ✅ 批量配置验证

### v0.2.0 (2026-03-01)
- ✨ **Agent 工具适配器系统** - 支持多个 Code Agent 工具
- ✨ **配置切换功能** - 快速在不同工具间切换模型
- ✨ **备份管理系统** - 自动备份和恢复配置
- ✅ 完成 Spec 002 Agent Adapter System
- ✅ 支持 Claude Code、Codex、Gemini CLI、Qwen CLI、Grok CLI

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
