# 数据模型: AgentSwitch Agent 工具适配器系统

**分支**: `002-agent-adapter` | **日期**: 2026-02-28

## 核心实体

### 1. AgentAdapter（Agent 适配器接口）

**职责**: 定义所有 Code Agent 工具适配器必须实现的统一接口

**Rust Trait 定义**:
```rust
pub trait AgentAdapter: Send + Sync {
    /// 返回工具的唯一标识符
    fn name(&self) -> &str;

    /// 检查工具是否已安装
    fn detect(&self) -> anyhow::Result<bool>;

    /// 返回工具的配置文件路径
    fn config_path(&self) -> anyhow::Result<PathBuf>;

    /// 创建配置文件的备份
    fn backup(&self) -> anyhow::Result<Backup>;

    /// 将模型配置应用到工具的配置文件
    fn apply(&self, model_config: &ModelConfig) -> anyhow::Result<()>;

    /// 从备份恢复配置文件
    fn restore(&self, backup: &Backup) -> anyhow::Result<()>;

    /// 返回工具当前使用的模型配置名称
    fn current_model(&self) -> anyhow::Result<Option<String>>;
}
```

**方法说明**:
- `name()`: 返回工具标识符，如 "claude-code"、"codex"、"gemini-cli"
- `detect()`: 通过检查可执行文件是否存在来判断工具是否已安装
- `config_path()`: 返回主配置文件的完整路径
- `backup()`: 创建完整备份，返回备份元数据
- `apply()`: 修改配置文件以使用指定的模型配置
- `restore()`: 将备份内容恢复到配置文件
- `current_model()`: 读取配置文件，返回当前使用的模型名称

**实现要求**:
- 每个适配器必须支持 JSON 或 TOML 或 YAML 格式
- 必须处理配置文件的原子性写入
- 必须支持版本检测和兼容性验证

---

### 2. Backup（配置备份）

**职责**: 表示一个工具配置文件的备份快照

**数据结构**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    /// 工具名称（如 "claude-code"）
    pub agent_name: String,

    /// 备份创建时间（ISO 8601 格式）
    pub timestamp: String,

    /// 备份文件的完整路径
    pub file_path: PathBuf,

    /// 原配置文件的路径
    pub original_path: PathBuf,

    /// 备份文件大小（字节）
    pub size_bytes: u64,

    /// 配置文件的格式（JSON/TOML/YAML）
    pub format: ConfigFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}
```

**字段约束**:
- `agent_name`: 必须是有效的工具标识符，匹配适配器的 `name()` 返回值
- `timestamp`: 必须是有效的 ISO 8601 格式时间字符串
- `file_path`: 文件必须存在于文件系统中
- `size_bytes`: 必须匹配实际文件大小
- `format`: 必须与原配置文件格式一致

**生命周期**:
- **创建**: 在执行 `apply()` 前自动创建
- **存储**: 保存在 `~/.agentswitch/backups/` 目录
- **清理**: 每个工具最多保留 10 个备份，超过时自动删除最旧的
- **恢复**: 通过 `restore()` 方法将备份内容写回原配置文件

---

### 3. AgentConfigState（Agent 配置状态）

**职责**: 表示一个工具的当前配置状态，用于 `asw status` 命令显示

**数据结构**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfigState {
    /// 工具名称
    pub agent_name: String,

    /// 当前使用的模型配置名称（如果已配置）
    pub model_name: Option<String>,

    /// 配置文件路径
    pub config_path: PathBuf,

    /// 最后切换时间（如果已切换）
    pub last_switched: Option<String>,

    /// 工具是否已安装
    pub is_installed: bool,

    /// 配置文件是否存在
    pub config_exists: bool,
}
```

**字段说明**:
- `model_name`: 从 `~/.agentswitch/config.toml` 的 `active_models` 映射中读取
- `config_path`: 通过适配器的 `config_path()` 方法获取
- `last_switched`: 从配置文件的修改时间或元数据中读取
- `is_installed`: 通过适配器的 `detect()` 方法获取
- `config_exists`: 检查配置文件路径是否存在

**显示格式**:
```
Agent            Status         Model         Config Path
───────────────────────────────────────────────────────────────────────
Claude Code       ✓ 已配置      glm           ~/.claude/settings.json
Codex            ✗ 未安装      -             ~/.codex/config.toml
Gemini CLI       ✓ 已配置      minimax       ~/.gemini/settings.json
```

---

### 4. ModelConfig（模型配置）

**职责**: 从 v0.1.0 继承，代表一个模型提供商的 API 配置

**数据结构**（v0.1.0 已有）:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// 唯一标识符
    pub name: String,

    /// API 基础 URL
    pub base_url: String,

    /// API 密钥
    pub api_key: String,

    /// 模型 ID
    pub model_id: String,

    /// 额外的键值对参数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, serde_json::Value>>,
}
```

**字段验证规则**:
- `name`: 不能为空字符串，不能包含特殊字符（除 `-` 和 `_`）
- `base_url`: 必须是合法的 URL（http:// 或 https://）
- `api_key`: 不能为空字符串
- `model_id`: 不能为空字符串
- `extra_params`: 可选，用于存储工具特定的参数

**使用场景**:
- 用户通过 `asw model add` 命令添加
- 存储在 `~/.agentswitch/config.toml` 的 `models` 数组中
- 通过 `asw switch <agent> <model>` 命令应用到工具配置文件

---

## 实体关系图

```
┌─────────────────┐
│  ModelConfig    │
│  (v0.1.0 已有)  │
└────────┬────────┘
         │
         │ 1:N
         ▼
┌─────────────────┐         ┌──────────────────┐
│   AppConfig    │         │  AgentAdapter    │
│  (v0.1.0 已有)  │         │  (Trait Interface) │
└─────────────────┘         └────────┬─────────┘
                                       │
                                       │ 实现
                                       ▼
                        ┌──────────────────────────────────────┐
                        │ 1:1                                    │
                        ┌────────────────┴────────────────┐ │
                        │                                  │ │
                 ┌────────▼────────┐          ┌────────▼────────┐ │
                 │ ClaudeCodeAdapter │          │   CodexAdapter  │ │
                 └─────────────────┘          └─────────────────┘ │
                        │                                  │ │
                        │                ┌─────────────┴───────────┐ │
                        │                │                       │ │
                 ┌───────▼────────┐   ┌───────▼───────────┐  │
                 │   GeminiAdapter │   │  更多适配器...     │  │
                 └────────────────┘   └───────────────────┘  │
                        │                                     │
                        │                                     │
            ┌───────────┴────────────┐        ┌──────────────┴─────────┐
            │                        │        │                        │
      ┌─────▼──────┐          ┌──────▼─────┐   ┌──────▼──────┐    ┌──────▼─────┐
      │  Backup   │          │AgentConfig │   │ConfigFile │    │ 更多实体... │
      └──────────┘          │   State    │   │ (具体实现)│    └────────────┘
                             └───────────┘   └────────────┘
```

---

## 配置文件格式映射

### Claude Code 配置文件结构

**文件路径**: `~/.claude/settings.json`

**JSON 结构**:
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "${api_key}",
    "ANTHROPIC_BASE_URL": "${base_url}",
    "ANTHROPIC_MODEL": "${model_id}"
  }
}
```

**字段映射**:
- `base_url` → `env.ANTHROPIC_BASE_URL`
- `api_key` → `env.ANTHROPIC_AUTH_TOKEN`
- `model_id` → `env.ANTHROPIC_MODEL`

### Codex 配置文件结构

**主配置文件**: `~/.codex/config.toml`

**TOML 结构**:
```toml
model_provider = "custom_provider"
model = "${model_id}"
preferred_auth_method = "apikey"

[model_providers.custom_provider]
name = "AgentSwitch"
base_url = "${base_url}"
wire_api = "responses"
```

**认证配置文件**: `~/.codex/auth.json`

**JSON 结构**:
```json
{
  "OPENAI_API_KEY": "${api_key}"
}
```

**字段映射**:
- `base_url` → `model_providers.custom_provider.base_url`
- `api_key` → `auth.json.OPENAI_API_KEY`
- `model_id` → `model`

### Gemini CLI 配置文件结构

**主配置文件**: `~/.gemini/settings.json`

**JSON 结构**:
```json
{
  "defaultModel": "${model_id}"
}
```

**环境变量文件**: `~/.gemini/.env`

**环境变量结构**:
```bash
GOOGLE_GEMINI_BASE_URL=${base_url}
GEMINI_API_KEY=${api_key}
GEMINI_MODEL=${model_id}
```

**字段映射**:
- `base_url` → `.env.GOOGLE_GEMINI_BASE_URL`
- `api_key` → `.env.GEMINI_API_KEY`
- `model_id` → `defaultModel` 或 `.env.GEMINI_MODEL`

---

## 数据验证规则

### ModelConfig 验证

| 字段 | 规则 | 错误消息 |
|------|------|---------|
| `name` | 非空，仅字母数字和 `-_` | "模型名称只能包含字母、数字、连字符和下划线" |
| `base_url` | 合法的 URL，必须以 http:// 或 https:// 开头 | "Base URL 必须是合法的 HTTP/HTTPS 地址" |
| `api_key` | 非空字符串 | "API Key 不能为空" |
| `model_id` | 非空字符串 | "模型 ID 不能为空" |

### Backup 验证

| 字段 | 规则 | 错误消息 |
|------|------|---------|
| `agent_name` | 必须是已注册的工具标识符 | "未知工具: {agent_name}" |
| `timestamp` | ISO 8601 格式 | "时间戳格式错误" |
| `file_path` | 文件必须存在 | "备份文件不存在" |
| `format` | 必须匹配实际文件格式 | "配置文件格式不匹配" |

### AgentConfigState 验证

| 字段 | 规则 | 错误消息 |
|------|------|---------|
| `agent_name` | 必须是已注册的工具标识符 | "未知工具: {agent_name}" |
| `config_path` | 路径不能为空 | "配置文件路径不能为空" |
| `is_installed` | 通过 detect() 方法验证 | - |
| `config_exists` | 通过文件系统验证 | - |

---

## 状态转换

### ModelConfig 生命周期

```
[创建] → [已保存] → [已应用到工具] → [已删除]
   ↓          ↓              ↓
 用户输入   持久化存储    配置文件修改
```

### Backup 生命周期

```
[创建] → [已保存] → [已恢复] → [已清理]
   ↓         ↓          ↓          ↓
 apply()   文件系统    restore()   清理旧备份
```

### AgentConfigState 状态转换

```
[未安装] → [已安装] → [已配置] → [未配置]
   ↓         ↓          ↓          ↓
detect()  detect()   switch()   工具卸载
                     返回
```

---

## 序列化格式

所有数据结构使用 `serde` 进行序列化/反序列化：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig { /* ... */ }
```

**支持的格式**:
- JSON: `serde_json`
- TOML: `toml`
- YAML: `serde_yaml` (实际上 Gemini CLI 不需要)

**往返一致性保证**:
- 序列化后再反序列化必须得到相同的对象
- 所有字段值与原始对象完全一致
- 测试覆盖所有数据结构

---

## 数据持久化

### 配置文件: `~/.agentswitch/config.toml`

**结构** (v0.1.0 已有):
```toml
[models]
  [[models.items]]
  name = "glm"
  base_url = "https://open.bigmodel.cn/api/v1"
  api_key = "sk-..."
  model_id = "glm-4"

[active_models]
  claude-code = "glm"
  codex = "minimax"
```

### 备份存储: `~/.agentswitch/backups/`

**命名规范**: `<agent-name>-<YYYYMMDD-HHMMSS>.config.<ext>.bak`

**示例**:
- `claude-code-20260228-143022.config.json.bak`
- `codex-20260228-143545.config.toml.bak`

**保留策略**: 每个工具最多保留 10 个备份

---

## 下一步

- 生成 contracts/（接口契约文档）
- 生成 quickstart.md（快速开始指南）
- 完成实现计划文档
