# Changelog

All notable changes to this project will be documented in this file.

## [0.5.1] - 2026-03-31

### Added
- **opencode 适配器**: 新增对 opencode CLI 工具的支持
  - 配置文件: `~/.config/opencode/opencode.json`
  - 认证文件: `~/.local/share/opencode/auth.json`
  - 支持 OpenAI 兼容协议

### Changed
- **codex 适配器**: 暂时禁用，因为 OpenAI Response API 兼容性问题
  - `detect()` 返回 `false`
  - `apply()` 返回详细错误信息，提示用户使用替代方案

### Fixed
- **registry**: 修复 `global_registry()` 注册逻辑，使用正确的适配器名称
- **claude-code detect**: 修复检测问题，使用正确的可执行文件名 `claude` 而非 `claude-code`
- **doctor detect**: 修复 `detect_tool()` 函数，使用 `adapter.detect()` 而非直接 `which::which()`
- **qwen 适配器**: 实现完整的 `apply()` 方法
- **grok 适配器**: 实现完整的 `apply()` 方法
- **Cargo.toml**: edition 从 `2024` 改为 `2021`（2024 尚未稳定）
- **dead_code 警告**: 清理未使用的代码

### Protocol Support
- OpenAI 兼容协议: `/v1/chat/completions`
- Anthropic 兼容协议: `/v1/messages`

### Supported Tools
| 工具 | 状态 | 协议 |
|------|------|------|
| claude-code | ✅ 支持 | Anthropic |
| opencode | ✅ 支持 | OpenAI 兼容 |
| gemini-cli | ✅ 支持 | OpenAI 兼容 |
| qwen-cli | ✅ 支持 | OpenAI 兼容 |
| grok-cli | ✅ 支持 | OpenAI 兼容 |
| codex | ❌ 暂不支持 | Response API |

---

## [0.1.0] - 2025-03-28

### Added
- 初始版本发布
- 基础 CLI 框架
- 模型配置管理 (add/list/remove/edit)
- 配置切换功能
- 备份管理
- 预设管理
- 批量操作
- 交互式向导
- 工具诊断
- Shell 补全
- Git 同步（框架）
