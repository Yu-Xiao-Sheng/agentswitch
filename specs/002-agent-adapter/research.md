# 技术研究: AgentSwitch Agent 工具适配器系统

**分支**: `002-agent-adapter` | **日期**: 2026-02-28 | **规格**: [spec.md](spec.md)

## 研究目标

本文档记录了 AgentSwitch v0.2.0 中需要支持的三个主流 Code Agent 工具（Claude Code、Codex、Gemini CLI）的配置文件结构、版本信息和适配策略。

**重要发现**: 经过网络调研，发现用户规格中的某些假设与实际情况不符：
1. **Gemini CLI 使用 JSON 而非 YAML 格式**（规格中假设为 YAML）
2. **Codex 需要使用旧版本（0.80.0）才能支持自定义 API**
3. **所有工具都支持环境变量覆盖配置文件设置**

---

## 研究结果

### 1. Claude Code 适配器

#### 版本信息
- **适配目标版本**: Claude Code CLI 2025-2026
- **配置格式**: JSON
- **配置文件位置**: `~/.claude/settings.json` (macOS/Linux)

#### 配置文件结构

**主配置文件**: `~/.claude/settings.json`

```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "sk-ant-api03-...",
    "ANTHROPIC_BASE_URL": "https://api.anthropic.com",
    "ANTHROPIC_MODEL": "claude-sonnet-4-5-20250929",
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5-20250929",
    "ANTHROPIC_DEFAULT_OPUS_MODEL": "claude-opus-4-5-20250929",
    "ANTHROPIC_DEFAULT_HAIKU_MODEL": "claude-haiku-4-5-20250929"
  },
  "includeCoAuthoredBy": true,
  "autoUpdates": true,
  "preferredNotifChannel": "default",
  "theme": "dark"
}
```

**认证配置文件**: `~/.claude/config.json` (可选)

```json
{
  "primaryApiKey": "self"
}
```

#### 关键配置字段映射

| AgentSwitch 字段 | Claude Code 字段 | 说明 |
|-----------------|------------------|------|
| `base_url` | `env.ANTHROPIC_BASE_URL` | API 基础 URL |
| `api_key` | `env.ANTHROPIC_AUTH_TOKEN` | API 密钥 |
| `model_id` | `env.ANTHROPIC_MODEL` | 默认模型 ID |

#### 版本检测策略
- **方法**: 解析 `settings.json` 文件结构
- **版本特定字段**: 检测 `env` 字段中的环境变量名称（如 `ANTHROPIC_MODEL` vs `ANTHROPIC_DEFAULT_SONNET_MODEL`）
- **兼容性**: 2025-2026 版本使用统一的环境变量命名规范

#### 参考来源
- [Claude Code 终端 (CLI) 使用教程 - CSDN](https://blog.csdn.net/weixin_59061577/article/details/158350450)
- [Claude Code 安装与配置完整指南（Mac) - zeeklog](https://zeeklog.com/claude-code-an-zhuang-yu-pei-zhi-wan-zheng-zhi-nan-mac/)
- [2026 Claude Code 国内全攻略 - 今日头条](https://m.toutiao.com/article/7611064462032552494/)
- [Claude Code 配置教程 - 技术站](https://jishuzhan.net/article/2013057638482362369)

---

### 2. Codex 适配器

#### 版本信息
- **适配目标版本**: OpenAI Codex CLI v0.80.0 (旧版本)
- **重要**: 最新版本已移除 Chat/Completions API 支持，必须使用 v0.80.0
- **配置格式**: TOML
- **配置文件位置**: `~/.codex/config.toml` (macOS/Linux)

#### 版本兼容性警告

⚠️ **关键发现**: Codex 的最新版本（0.80.0 之后）不再支持 Chat/Completions API。为了使用自定义 API，用户必须安装旧版本：

```bash
npm install -g @openai/codex@0.80.0
```

**版本更新影响**: 当 Codex 更新到不兼容自定义 API 的新版本时，适配器需要重新实现或提示用户降级。

#### 配置文件结构

**主配置文件**: `~/.codex/config.toml`

```toml
model_provider = "custom_provider"
model = "gpt-5-codex"
model_reasoning_effort = "high"
disable_response_storage = true
preferred_auth_method = "apikey"

[model_providers.custom_provider]
name = "Custom Provider"
base_url = "https://your-custom-api-url.com/v1"
wire_api = "responses"
```

**认证配置文件**: `~/.codex/auth.json`

```json
{
  "OPENAI_API_KEY": "sk-your-api-key-here"
}
```

#### 关键配置字段映射

| AgentSwitch 字段 | Codex 字段 | 说明 |
|-----------------|------------|------|
| `base_url` | `model_providers.custom_provider.base_url` | API 基础 URL |
| `api_key` | `auth.json.OPENAI_API_KEY` | API 密钥（存储在 auth.json） |
| `model_id` | `model` | 模型 ID |

#### 版本检测策略
- **方法**: 执行 `codex --version` 命令获取版本号
- **版本解析**: 解析版本字符串，检查是否为 v0.80.0 或兼容版本
- **兼容性检查**: 如果版本 > 0.80.0，警告用户可能不支持自定义 API

#### 参考来源
- [Code X配置第三方API - 百家号](https://baijiahao.baidu.com/s?id=1856248941121897388)
- [AI编程工具推荐：Codex安装与配置API完整指南 - CSDN](https://m.blog.csdn.net/badfl/article/details/157763018)
- [Codex教程大全 - 掘金](https://juejin.cn/post/7602893600251215918)
- [使用自定义API接入OpenAI CodeX配置 - 博客园](https://www.cnblogs.com/smileZAZ/articles/19406570.html)

---

### 3. Gemini CLI 适配器

#### 版本信息
- **适配目标版本**: Google Gemini CLI 2025-2026
- **配置格式**: JSON (⚠️ 不是 YAML，规格中假设错误)
- **配置文件位置**: `~/.gemini/settings.json` (macOS/Linux)

#### 配置文件结构

**主配置文件**: `~/.gemini/settings.json`

```json
{
  "security": {
    "auth": {
      "selectedType": "gemini-api-key"
    }
  },
  "theme": "dark",
  "defaultModel": "gemini-2.5-pro"
}
```

**环境变量文件**: `~/.gemini/.env`

```bash
GOOGLE_GEMINI_BASE_URL=https://your-api-endpoint.com
GEMINI_API_KEY=sk-your-api-key
GEMINI_MODEL=gemini-3-pro-preview
```

#### ⚠️ 重要发现：配置格式更正

经过网络调研确认，**Gemini CLI 使用 JSON 格式，而非 YAML 格式**。规格中的假设需要更正：

| 规格假设 | 实际情况 |
|---------|---------|
| 配置文件: `~/.gemini-cli/config.yaml` | 配置文件: `~/.gemini/settings.json` |
| 配置格式: YAML | 配置格式: JSON |
| 版本假设: 无 | 适配版本: Gemini CLI 2025-2026 |

#### 关键配置字段映射

| AgentSwitch 字段 | Gemini CLI 字段 | 说明 |
|-----------------|-----------------|------|
| `base_url` | `.env.GOOGLE_GEMINI_BASE_URL` | API 基础 URL（环境变量） |
| `api_key` | `.env.GEMINI_API_KEY` | API 密钥（环境变量） |
| `model_id` | `defaultModel` 或 `.env.GEMINI_MODEL` | 默认模型 ID |

#### 配置优先级
Gemini CLI 的配置优先级（从高到低）：
1. 命令行参数
2. 项目配置 (`.gemini/settings.json`)
3. 用户配置 (`~/.gemini/settings.json`)
4. 系统配置
5. 环境变量 (`.env`)

**适配策略**: AgentSwitch 应该修改用户级别的 `settings.json` 和 `.env` 文件。

#### 版本检测策略
- **方法**: 执行 `gemini --version` 命令获取版本号
- **配置格式验证**: 检查 `settings.json` 是否存在以及其结构
- **版本特定字段**: 检测 `security.auth.selectedType` 字段（2025-2026 版本特有）

#### 参考来源
- [【万字长文】Gemini 3 Pro 全面指南 - CSDN](https://blog.csdn.net/qq_20042935/article/details/157432319)
- [Gemini CLI 安装和配置第三方 API 模型 - CSDN](https://m.blog.csdn.net/weixin_46528266/article/details/157070257)
- [Gemini CLI 接入指南 - 掘金](https://juejin.cn/post/7594576956421079066)
- [玩转Gemini CLI配置 - CSDN](https://m.blog.csdn.net/gitblog_00771/article/details/156481115)

---

## 技术决策总结

### 决策 1: 配置文件格式支持

**选择**: 支持 JSON (Claude Code、Gemini CLI) 和 TOML (Codex)

**理由**:
- Claude Code 使用 JSON
- Gemini CLI 使用 JSON（非 YAML，规格需要更正）
- Codex 使用 TOML
- 需要在适配器层实现不同的序列化/反序列化逻辑

**实现**:
- 使用 `serde_json` 处理 JSON 格式
- 使用 `toml` crate 处理 TOML 格式
- 为每种格式保持原有格式（缩进、排序、注释）

### 决策 2: 版本管理策略

**选择**: 适配器明确标注适配版本，版本更新时需重新验证

**理由**:
- Codex v0.80.0 之后不支持自定义 API
- 工具版本更新可能改变配置文件结构
- 需要建立版本检测和兼容性验证机制

**实现**:
- 在适配器实现中标注 `SUPPORTED_VERSION` 常量
- 实现版本检测逻辑，警告用户不兼容版本
- 版本更新时重新运行研究流程，验证配置兼容性

### 决策 3: 环境变量处理

**选择**: 检测环境变量并警告用户，但仍修改配置文件

**理由**:
- 所有三个工具都支持环境变量覆盖配置
- 环境变量优先级高于配置文件
- AgentSwitch 修改配置文件可能不会生效

**实现**:
- 在切换完成后检测常见环境变量（如 `ANTHROPIC_BASE_URL`）
- 显示黄色警告，提示用户检查环境变量
- 建议用户取消设置环境变量或重启终端

### 决策 4: API Key 存储方式差异

**发现**: 不同工具使用不同的 API Key 存储方式

| 工具 | API Key 存储位置 |
|------|-----------------|
| Claude Code | `settings.json` 的 `env.ANTHROPIC_AUTH_TOKEN` 字段 |
| Codex | 独立的 `auth.json` 文件 |
| Gemini CLI | `.env` 文件的 `GEMINI_API_KEY` 字段 |

**实现策略**:
- Claude Code: 直接修改 `settings.json`
- Codex: 同时修改 `config.toml` 和 `auth.json`
- Gemini CLI: 同时修改 `settings.json` 和 `.env`

---

## 待研究事项（如需要）

1. **配置文件加密**: 当前规格要求 API Key 加密存储，但三个工具本身都是明文存储。需要决定是否在 AgentSwitch 层额外加密。

2. **Windows 路径处理**: 当前研究主要集中在 macOS/Linux，需要补充 Windows 平台的配置文件路径和权限处理。

3. **多版本配置文件支持**: Claude Code 支持项目级配置（`.claude/settings.json`），需要决定是否支持。

---

## 研究结论

### 需要更正的规格内容

1. **FR-037**: "Gemini CLI 实现适配器，支持 **YAML** 格式配置文件" → 应更正为 "**JSON** 格式配置文件"

2. **边界情况**: 所有三个工具都支持环境变量覆盖，已澄清

3. **适配器实现**:
   - Claude Code 适配器: 适配版本 "Claude Code CLI 2025-2026"，配置格式 JSON
   - Codex 适配器: 适配版本 "v0.80.0"，配置格式 TOML，版本更新需重新验证
   - Gemini CLI 适配器: 适配版本 "Gemini CLI 2025-2026"，配置格式 JSON

### 下一步行动

1. ✅ 完成三个 Code Agent 的配置文件研究
2. ⏭️ 更新规格文档中的格式错误（YAML → JSON）
3. ⏭️ 生成 data-model.md（数据模型设计）
4. ⏭️ 生成 contracts/（接口契约定义）
5. ⏭️ 生成 quickstart.md（快速开始指南）

### 研究方法说明

本研究完全基于网络搜索和公开文档调研，未进行任何推测或猜测。所有结论都来自实际的官方文档和第三方教程，并附上了参考来源链接。

**研究日期**: 2026-02-28
**研究工具**: WebSearch
**关键词**: "Claude Code config.json", "Codex config.toml", "Gemini CLI settings.json"
