# 数据模型: 便捷安装与分发系统

**功能**: Spec 001 - 便捷安装与分发系统
**日期**: 2026-03-11
**状态**: N/A

## 概述

**注意**: 本功能（便捷安装与分发系统）不涉及新的数据实体。它是纯安装和打包系统，不扩展 AgentSwitch 的核心数据模型。

## 无数据模型的原因

1. **功能性质**: 本功能是安装和分发工具，不涉及应用运行时的数据存储
2. **现有数据模型**: AgentSwitch 已有完整的数据模型（模型配置、预设、备份等），无需修改
3. **关注点分离**: 安装包的职责是交付二进制文件，不涉及应用逻辑

## 现有数据模型参考

AgentSwitch 的现有数据模型定义在以下文件中：

- **配置模型**: `src/config/models.rs`
  - `ModelConfig`: 模型配置
  - `AgentConfig`: Agent 工具配置
  - `Preset`: 配置预设

- **存储模型**: `src/config/store.rs`
  - `ConfigStore`: 配置存储
  - `BackupStore`: 备份存储

这些现有模型**不受**本功能影响。

## 安装包元数据

虽然本功能不定义新的数据实体，但安装包本身包含元数据：

### DEB 包元数据

```toml
[package.metadata.deb]
name = "agentswitch"
version = "0.4.0"
architecture = "amd64" | "arm64"
maintainer = "..."
depends = "$auto"
```

### GitHub Release 元数据

```json
{
  "tag_name": "v0.4.0",
  "name": "AgentSwitch v0.4.0",
  "assets": [
    {
      "name": "agentswitch_0.4.0_amd64.deb",
      "size": 1234567,
      "browser_download_url": "..."
    }
  ]
}
```

这些是**打包时**的元数据，不是运行时的数据模型。

## 文件系统布局

安装包定义了文件在系统中的布局，但这**不是**数据模型：

```
/usr/bin/asw                 # 二进制文件
/usr/share/man/man1/asw.1    # man 手册
/usr/share/bash-completion/completions/asw  # Bash 补全
~/.agentswitch/              # 用户配置目录（运行时创建）
```

## 结论

本功能**不定义新的数据模型**。所有相关的信息已在以下文档中定义：

- [Shell 安装脚本契约](./contracts/install-script.md)
- [DEB 包契约](./contracts/deb-package.md)
- [快速开始指南](./quickstart.md)
