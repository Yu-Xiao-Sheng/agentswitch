# Changelog

All notable changes to this project will be documented in this file.

## [0.8.6] - 2026-03-31

### Fixed
- **代码质量优化**: 修复所有 Clippy 警告
  - 修复 `borrowed_box` 警告（添加 `#[allow(clippy::borrowed_box)]`）
  - 修复 `format_in_format_args` 警告（简化 println 格式）
  - 修复 `useless_format` 警告（使用 .to_string()）
  - 修复 `cloned_ref_to_slice_refs` 警告（使用 std::slice::from_ref）
  - 修复 `module_inception` 警告（添加 `#[allow(clippy::module_inception)]`）
  - 修复 `type_complexity` 警告（定义 CustomValidator 类型别名）

### Changed
- 代码风格统一，提升代码质量

---

## [0.8.5] - 2026-03-31

### Fixed
- **Claude Code adapter 实现**: 完整实现 Claude Code adapter 的 `apply()` 和 `current_model()` 方法
  - 配置文件使用 `~/.claude/settings.json`，通过 `env` 字段设置环境变量
  - 支持 `ANTHROPIC_BASE_URL`、`ANTHROPIC_API_KEY`、`ANTHROPIC_MODEL` 配置
  - 保留现有的 settings.json 中的其他配置字段

- **preset create --agent 格式修复**: 修复 `--agent` 参数的解析逻辑
  - 现在支持值部分包含冒号的格式，如 `--agent gemini-cli:provider:model`
  - 改进错误提示，明确说明格式要求

- **新增 agent show 命令**: 添加 `asw agent show <name>` 子命令
  - 显示指定 Agent 的详细信息
  - 包括安装状态、配置文件路径、当前模型、激活配置等
  - 对于未安装的工具提供安装提示

- **status --detailed 功能实现**: 实现详细状态显示模式
  - `asw status` 显示简洁的单行状态
  - `asw status --detailed` 显示详细的块状状态信息
  - 包括配置文件状态、激活配置等详细信息

- **backup list 功能修复**: 确保备份列表正确显示
  - 备份目录创建逻辑与列表读取逻辑一致
  - 支持空备份创建（当配置文件不存在时）
  - 改进备份文件的存储路径

- **doctor 命令简化**: 修复需要双重命令的问题
  - `asw doctor` 直接运行完整诊断（原来是 `asw doctor doctor`）
  - `asw doctor detect` 运行简化版工具检测
  - 支持参数：`--verbose`、`--json`、`--fix`

### Changed
- 重构 Doctor 命令结构，提供更直观的命令行体验
- 改进 parse_kv 函数，支持更灵活的键值对格式
- 统一 adapter 的 backup 方法，支持空配置文件的情况

---

## [0.8.4] - 2026-03-31

### Added
- **智能错误提示系统**: 友好的错误消息、原因分析和解决建议
  - 统一错误类型系统 `AswError`，支持 Config/Network/Permission/Provider/Tool/Crypto/Git 等类型
  - 错误消息包含类型、描述、可能原因、建议操作
  - 错误输出格式化显示，使用颜色区分不同信息

### Changed
- **改进 provider add 命令错误提示**:
  - URL 格式验证失败时显示正确的 URL 格式示例
  - API Key 格式验证失败时提供检查建议
  - 模型名称验证失败时显示命名规则
  - 协议类型错误时显示支持的协议列表

- **改进 provider test 命令错误提示**:
  - 网络连接失败时提供诊断建议（检查 URL、测试网络、配置代理）
  - HTTP 错误码针对性提示（401/403/404/429/5xx）
  - 显示完整的 API 端点 URL 和响应状态

- **改进 switch 命令错误提示**:
  - 供应商不存在时提示使用 `asw provider list` 查看
  - 模型不在列表时显示可用模型
  - 工具未安装时显示安装命令
  - 配置应用失败时提示运行 `asw doctor` 诊断

### Documentation
- 新增设计文档: `docs/superpowers/plans/2026-03-31-smart-error-messages.md`

### Error Output Format
```
✗ 错误

  类型: 网络错误
  URL: https://api.xxx.com/v1
  描述: 无法连接到 API 端点

可能的原因:
  1. API 地址错误
  2. 网络连接问题
  3. 需要配置代理
  4. API 服务不可用

建议操作:
  • 检查 base_url 配置是否正确
  • 测试网络连接: ping <api-host>
  • 如需代理，设置环境变量: export HTTP_PROXY=http://proxy:port
  • 运行 'asw provider test <name> --verbose' 获取详细诊断
```

---

## [0.8.3] - 2026-03-31

### Added
- **自动更新检测**: 检查 crates.io/GitHub 上的新版本
  - `asw update check` - 手动检查更新
  - `asw update check --force` - 强制检查（忽略缓存）
  - 启动时自动检查（带 24 小时缓存）
  - 显示更新提示和发布说明链接
- **版本缓存**: 24 小时内不重复检查，减少 API 调用
- **降级策略**: 优先从 crates.io 获取版本，失败则从 GitHub 获取

### Changed
- **命令参考**: 将 `update` 命令从"计划中的命令"移动到"当前可用命令"

---

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
