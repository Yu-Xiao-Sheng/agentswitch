# Changelog

All notable changes to this project will be documented in this file.

## [0.8.2] - 2026-03-31

### Documentation
- **README 完整更新**: 同步到 v0.8.x 架构
  - 更新命令参考：`model` → `provider` 命令
  - 更新 switch 语法：`asw switch <tool> <provider> <model>`
  - 更新供应商配置示例（新增协议参数）
  - 更新 AgentAdapter trait 签名
  - 新增 Provider 和 ActiveConfig 结构说明
  - 新增配置文件格式示例
  - 更新预设管理命令（create/delete/update/import/export）
  - 更新批量操作命令（--agent/--parallel/--dry-run）
  - 更新 wizard/doctor/completion/sync 命令格式
  - 新增配置加密功能说明
- **CHANGELOG 整理**: 修复重复标题，添加 v0.8.1 记录
- **版本历史更新**: 添加 v0.5.1 ~ v0.8.2 完整版本记录

---

## [0.8.1] - 2026-03-31

### Changed
- **Adapter 签名更新**: `AgentAdapter::apply()` 方法签名从 `apply(&self, model_config: &ModelConfig)` 改为 `apply(&self, provider: &Provider, model: &str)`
- **代码清理**: 清理所有适配器中的旧代码，统一使用新的 Provider-Model 架构
- **文档更新**: 忽略 `docs/superpowers/` 目录（开发计划文件不纳入版本控制）

### Fixed
- 修复所有适配器编译警告
- 统一代码风格

---

## [0.8.0] - 2026-03-31

### 重构
**完全重构为 Provider-Model 架构**

#### 新数据模型
- `Provider`: 供应商（base_url, api_key, models）
- `ActiveModel`: 当前激活配置（tool -> {provider, model})

#### 新命令
- `asw provider add/list/show/remove/test`
- `asw switch <tool> <provider> <model>`
- `asw status`

#### 配置文件
```toml
# ~/.agentswitch/config.toml

[[providers]]
name = "zhipu"
base_url = "https://open.bigmodel.cn/api/anthropic"
api_key = "your-key"
protocol = "anthropic"
models = ["glm-4.7-flash", "glm-5"]

[active.claude-code]
provider = "zhipu"
model = "glm-4.7-flash"
```

#### 废弃的命令
- `model` 命令（被 `provider` 替代）
- `model fetch/batch` 命令合并到 `provider fetch-models`

---

## [0.7.0] - 2026-03-31

### Added
- **多模型支持**: 一个渠道支持多个模型
  - `model add --models`: 添加多个模型
  - `model show`: 查看渠道所有模型
- **Provider 测试**: 测试 API 连接和模型可用性
  - `model test <provider>`: 测试渠道
  - `model test <provider> --model <name>`: 测试特定模型
  - `model add --test`: 添加时自动测试
- **模型列表获取**: 从 API 自动获取可用模型
  - `model fetch <provider>`: 从 /v1/models 获取
- **批量配置**: 从文件批量添加模型
  - `model batch <provider> --file models.txt`
- **配置文件管理**: 支持直接编辑配置文件
  - 位置: `~/.agentswitch/config.toml`
  - 格式: TOML

### Changed
- ModelConfig 结构支持 models 数组
- model_id 字段改为 default_model
- 所有适配器更新使用新 API

### Fixed
- 编译错误修复（model_id → get_default_model()）

---

## [0.6.0] - 2026-03-31

### Added
- **配置加密系统**: AES-256-GCM 加密，密钥本地存储
  - `asw crypto keygen` - 生成加密密钥
  - `asw crypto key-export` - 导出密钥（Base64）
  - `asw crypto key-import` - 导入密钥
  - `asw crypto status` - 查看加密状态
- **Git 同步完善**: 完整的多机配置同步
  - `asw sync init` - 初始化 Git 仓库 + 密钥检查
  - `asw sync push` - 加密配置 + 提交 + 推送
  - `asw sync pull` - 拉取 + 解密配置
  - `asw sync status` - 查看同步状态
  - `asw sync remote` - 管理远程仓库
- **安全特性**:
  - API Key 自动加密存储
  - 密钥仅存本地，不上传云端
  - 新机器需导入密钥才能解密
  - 密钥丢失时友好提示

- **opencode 适配器**: 新增对 opencode CLI 工具的支持
  - 配置文件: `~/.config/opencode/opencode.json`
  - 认证文件: `~/.local/share/opencode/auth.json`
  - 支持 OpenAI 兼容协议

### Changed
- **codex 适配器**: 暂时禁用（待 OpenAI Response API 兼容性改善后重新启用)
  - `detect()` 返回 `false`
  - `apply()` 返回详细错误信息

### Fixed
- **registry**: 修复 `global_registry()` 注册逻辑
- **claude-code detect**: 修复检测问题，使用正确的可执行文件名
- **doctor detect**: 修复 `detect_tool()` 函数
- **qwen 适配器**: 实现完整的 `apply()` 方法
- **grok 适配器**: 实现完整的 `apply()` 方法
- **Cargo.toml**: edition 从 `2024` 改为 `2021`
- **dead_code 警告**: 清理未使用的代码

- **cargo fmt**: 修复格式问题

- **README**: 更新描述，支持 OpenAI/Anthropic 双协议

- **CI/CD**: 修复 cargo fmt 检查错误

### Security Notes
⚠️ **重要**:
- 密钥文件位于 `~/.agentswitch/keys/master.key`
- 请务必妥善保管导出的密钥
- 密钥丢失后将无法解密配置
- 同步到新机器时需先导入密钥

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
- **doctor detect**: 修复 `detect_tool()` 函数,使用 `adapter.detect()` 而非直接 `which::which()`
- **qwen 适配器**: 实现完整的 `apply()` 方法
- **grok 适配器**: 实现完整的 `apply()` 方法
- **Cargo.toml**: edition 从 `2024` 改为 `2021`（2024 尚未稳定)
- **dead_code 警告**: 清理未使用的代码

- **cargo fmt**: 修复格式问题
- **README**: 更新描述，支持 OpenAI/Anthropic 双协议
- **CI/CD**: 修复 cargo fmt 检查错误

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
- Git 同步（框架)
