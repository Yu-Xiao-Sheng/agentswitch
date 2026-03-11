# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-03-11

### Added
- ✨ **便捷安装系统**: Shell 脚本一键安装和 DEB 包分发
  - `curl ... | bash` 一键安装，支持 Linux 和 macOS
  - `bash scripts/install.sh --local-file <path>` 本地二进制安装
  - DEB 包支持 Debian 11+ 和 Ubuntu 20.04+
  - GitHub Actions 自动化多平台构建和发布
  - 支持平台: Linux x86_64/ARM64, macOS Intel/Apple Silicon
  - 完整的打包系统架构文档，支持未来扩展（RPM、Homebrew、Chocolatey）
  - Shell 补全自动配置
  - man 手册页
- 📚 **文档增强**:
  - 新增 `INSTALL.md` 详细安装指南
  - 新增 `docs/packaging.md` 打包系统架构文档
  - 新增 `docs/roadmap.md` 功能路线图

### Changed
- 🔄 更新版本号至 v0.5.0
- 🔄 改进安装脚本，添加 `--local-file` 选项支持本地二进制测试

### Developer Experience
- 🧪 完整的安装测试脚本（scripts/test-install.sh）
- ✅ 完成 Spec 001 所有功能
- ✅ 所有 55 个任务完成

---
- ✨ **便捷安装系统**: Shell 脚本一键安装和 DEB 包分发
  - `curl ... | bash` 一键安装，支持 Linux 和 macOS
  - DEB 包支持 Debian 11+ 和 Ubuntu 20.04+
  - GitHub Actions 自动化多平台构建和发布
  - 支持平台: Linux x86_64/ARM64, macOS Intel/Apple Silicon
  - 完整的打包系统架构文档，支持未来扩展（RPM、Homebrew、Chocolatey）
  - Shell 补全自动配置
  - man 手册页
- 📚 **文档增强**:
  - 新增 `INSTALL.md` 详细安装指南
  - 新增 `docs/packaging.md` 打包系统架构文档
  - 新增 `docs/roadmap.md` 功能路线图

## [0.4.0] - 2026-03-11

### Added
- ✨ **交互式配置向导**: 友好的 CLI 向导引导用户完成初始化配置
  - `asw wizard init` - 启动初始化向导
  - 支持进度保存和恢复 (`--resume`)
  - 支持重新开始 (`--reset`)
  - 步骤式引导：欢迎页面、工具检测、模型配置、配置验证
- ✨ **工具诊断功能**: 自动检测系统中已安装的 Code Agent 工具
  - `asw doctor` - 运行完整诊断（工具检测、配置检查、健康检查）
  - `asw doctor detect` - 简化版工具检测
  - 支持的工具：Claude Code、Codex、Gemini CLI、Qwen CLI、Grok CLI
  - 详细的诊断报告（安装状态、配置文件路径、健康状态）
- ✨ **Shell 自动补全**: 为 Bash、Zsh、Fish 提供智能补全
  - `asw completion generate` - 生成补全脚本
  - `asw completion install` - 一键安装补全脚本
  - 支持命令、子命令、参数的智能补全
  - 支持动态补全（模型名称、工具名称、预设名称等）
- ✨ **Git 配置同步**: 支持多机器配置共享和版本控制
  - `asw sync init` - 初始化 Git 仓库
  - `asw sync status` - 查看同步状态
  - `asw sync push` - 推送到远程仓库
  - `asw sync pull` - 从远程仓库拉取更新
  - 自动 `.gitignore` 配置（忽略敏感信息）
  - API Key 加密存储（AES-256-GCM + Argon2）
  - 支持冲突解决策略

### Changed
- 🔄 改进错误提示和配置验证
- 🔄 优化用户体验，提供更友好的交互式界面

### Fixed
- 🐛 修复向导状态文件序列化问题
- 🐛 修复补全脚本生成时的动态内容问题

### Security
- 🔒 API Key 加密存储（AES-256-GCM + Argon2id）
- 🔒 自动生成加密密钥并安全存储
- 🔒 Git 同步时自动忽略未加密的敏感信息

### Developer Experience
- 📚 完善的向导和诊断模块文档
- 🧪 增加单元测试覆盖率
- ✅ 完成 Spec 004 所有功能

## [0.3.0] - 2026-03-05

### Added
- ✨ **配置预设管理**: 保存和一键应用常用配置组合
  - `asw preset save` - 保存当前配置为预设
  - `asw preset list` - 列出所有预设
  - `asw preset apply` - 应用预设配置
  - `asw preset remove` - 删除预设
  - `asw preset validate` - 验证预设
  - 支持预设标签系统（便于分类和搜索）
  - 预设文件独立存储（`~/.agentswitch/presets/`）
- ✨ **批量操作**: 同时切换多个工具到同一模型
  - `asw batch switch <model>` - 批量切换所有工具
  - `asw batch validate` - 批量验证配置
  - 支持跳过未安装的工具
  - 详细的执行报告
- ✨ **预设管理增强**:
  - 支持预设描述和标签
  - 预设验证（检查模型和工具是否有效）
  - 预设应用前的确认提示
- ✨ **批量操作增强**:
  - 自动检测已安装工具
  - 失败跳过机制（单个工具失败不影响其他工具）
  - 详细的执行结果统计

### Changed
- 🔄 优化配置文件结构（支持预设和批量操作）
- 🔄 改进状态显示（显示预设信息）

### Fixed
- 🐛 修复批量操作时的并发问题
- 🐛 修复预设应用时的配置验证问题

### Developer Experience
- 📚 完善的预设和批量操作文档
- 🧪 增加单元测试覆盖率
- ✅ 完成 Spec 003 所有功能

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

[Unreleased]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Yu-Xiao-Sheng/agentswitch/releases/tag/v0.1.0
