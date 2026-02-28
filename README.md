# AgentSwitch

> 一个通用的代码终端代理工具配置切换器，支持将任意 OpenAI 协议模型接入到主流 Code Agent 工具中。

[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📖 项目简介

AgentSwitch 是一个配置管理工具，让你可以在不同的 Code Agent 工具（如 Claude Code、Codex、Gemini CLI 等）之间轻松切换不同的模型提供商。

## ✨ 核心功能

- **模型配置管理**: 添加、列出、编辑、删除模型配置
- **自动初始化**: 首次运行自动创建配置目录和文件
- **安全存储**: API Key 保护（掩码显示 + 文件权限 0600）
- **友好输出**: 彩色输出、清晰的错误提示

## 🚀 快速开始

### 安装

```bash
# 从源码编译
git clone https://github.com/Yu-Xiao-Sheng/agentswitch.git
cd agentswitch
cargo build --release

# 或使用 cargo install
cargo install --path .
```

### 使用示例

```bash
# 添加模型配置
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-..." \
  --model "glm-4"

# 列出所有模型
asw model list

# 编辑模型配置
asw model edit glm --model "glm-4-turbo"

# 删除模型配置
asw model remove glm
```

## 📋 命令参考

```
asw
├── model             # 模型配置管理
│   ├── add          # 添加模型配置
│   ├── list         # 列出所有模型
│   ├── remove       # 删除模型配置
│   └── edit         # 编辑模型配置
└── status           # 显示当前状态
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
```

## 📄 开源协议

本项目采用 [MIT License](LICENSE) 开源协议。

## 🔗 相关资源

- [项目文档](specs/001-core-foundation/)
- [Issue Tracker](https://github.com/Yu-Xiao-Sheng/agentswitch/issues)
