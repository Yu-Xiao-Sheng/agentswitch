# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- 不兼容字段检测和警告功能

## [0.2.0] - 2026-03-03

### Added
- ✨ **Agent 适配器系统**: 完整的 Code Agent 工具适配器框架
  - 支持 Claude Code、Codex、Gemini CLI、Qwen、Grok
  - 自动检测工具安装状态
  - 统一的配置文件解析和生成接口
- ✨ **配置切换功能**: `asw switch <agent> <model>` 命令
  - 自动备份原配置
  - 支持多种配置格式（JSON、TOML、.env）
  - 环境变量覆盖警告
- ✨ **备份管理系统**: 完整的配置备份和恢复功能
  - `asw backup list` - 列出所有备份
  - `asw backup restore` - 恢复备份
  - `asw backup clean` - 清理旧备份
  - 文件锁保证原子性
  - 自动限制备份数量（最多 10 个）
- ✨ **状态显示**: `asw status` 命令
  - 显示所有工具的配置状态
  - 显示当前使用的模型
  - 显示配置文件路径
- ✨ **适配器注册表**: 动态适配器注册机制
  - `asw agent list` - 列出所有已注册适配器
  - 支持运行时注册新适配器
  - 适配器验证功能
- ✨ **开发者文档**: 完整的适配器开发指南
  - ADAPTER_EXAMPLES.md（5500+ 字）
  - 自定义适配器示例代码
  - 配置格式处理示例（JSON、TOML、.env）
- ✨ **字段兼容性检测**: 不兼容配置字段检测和警告
  - 常见不兼容字段列表
  - 工具特定字段检测
  - 警告级别分类

### Changed
- 🔄 重构适配器接口，添加 `current_model()` 方法
- 🔄 优化配置文件解析，保留非 API 配置字段
- 🔄 改进错误处理，提供更详细的错误信息

### Fixed
- 🐛 修复备份文件权限设置（0600）
- 🐛 修复配置文件并发写入问题（使用文件锁）
- 🐛 修复 Gemini CLI 备份目录名称问题

### Security
- 🔒 所有配置文件权限设置为 0600（仅所有者可读写）
- 🔒 备份文件权限设置为 0600
- 🔒 API Key 掩码显示

### Developer Experience
- 📚 完善的代码文档（中文注释）
- 📚 适配器开发指南和示例
- 🧪 单元测试框架（覆盖率 60%）
- ✅ Clippy 检查通过

## [0.1.0] - 2026-02-XX

### Added
- ✨ 模型配置统一管理
  - `asw model add` - 添加模型配置
  - `asw model list` - 列出所有模型
  - `asw model remove` - 删除模型配置
  - `asw model edit` - 编辑模型配置
- ✨ 配置文件存储（`~/.agentswitch/config.toml`）
- ✨ API Key 安全存储（文件权限 0600）
- ✨ 输入验证（URL 格式、模型名称）
- ✨ 彩色 CLI 输出

[Unreleased]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/releases/tag/v0.1.0
