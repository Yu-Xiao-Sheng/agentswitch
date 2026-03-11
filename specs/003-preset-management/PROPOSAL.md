# Spec 003 规划提案: 配置预设与批量管理

**优先级**: P2 (高优先级增强功能)
**目标版本**: v0.3.0
**规划日期**: 2026-03-03
**状态**: 🎯 待评审

---

## 📋 规划背景

### 当前状态 (v0.2.0)

**已完成的核心功能**:
- ✅ Phase 1: 核心基础 (v0.1.0)
- ✅ Phase 2: 模型配置管理 (v0.1.0)
- ✅ Phase 3: Agent 工具适配器 (v0.2.0)
- ✅ Phase 4: 配置切换功能 (v0.2.0)

### 用户反馈与痛点

通过 v0.2.0 的使用，可能存在以下痛点：

1. **重复操作繁琐**
   - 每次切换多个工具需要执行多次 `asw switch` 命令
   - 无法快速保存和恢复常用配置组合

2. **配置难以分享**
   - 团队成员之间的配置无法方便共享
   - 多台机器之间的配置迁移需要手动操作

3. **缺乏工作流支持**
   - 项目特定的工具配置无法预设
   - 无法快速在不同配置间切换

### 下一步规划

根据 **Phase 5: 高级功能 (P2)**，规划 **Spec 003: 配置预设与批量管理**。

---

## 🎯 Spec 003 核心目标

### 主要目标

实现一套完整的配置预设和工作流管理系统，让用户能够：

1. **保存和管理配置预设**
   - 创建命名的配置预设
   - 快速应用预设到多个工具
   - 在不同预设之间切换

2. **批量操作工具**
   - 一次性切换所有工具到同一模型
   - 批量应用配置预设
   - 批量验证配置状态

3. **配置导入导出**
   - 导出配置为可分享的文件
   - 从文件导入配置
   - 支持团队配置共享

### 次要目标

- 配置预设验证
- 配置预设版本管理
- 配置迁移工具

---

## 📐 功能设计

### User Story 1: 配置预设管理

#### 1.1 创建预设
```bash
asw preset create development \
  --claude-code glm \
  --codex glm \
  --gemini-cli glm \
  --description "开发环境使用 GLM"
```

**功能要求**:
- 支持为所有工具或指定工具创建预设
- 预设包含模型配置名称
- 支持添加描述和标签
- 预设自动验证（检查模型是否存在）

#### 1.2 列出预设
```bash
asw preset list
```

**输出格式**:
```
可用的配置预设:

development
  描述: 开发环境使用 GLM
  标签: work, glm
  工具配置:
    - claude-code → glm
    - codex → glm
    - gemini-cli → glm

production
  描述: 生产环境使用 GPT-4
  标签: prod, stable
  工具配置:
    - claude-code → gpt-4
    - codex → gpt-4
```

#### 1.3 应用预设
```bash
# 应用预设到所有工具
asw preset apply development

# 应用预设到指定工具
asw preset apply development --agents claude-code,codex

# 应用预设前自动备份
asw preset apply development --backup
```

**功能要求**:
- 自动备份原配置
- 支持选择性应用
- 显示应用结果
- 失败自动回滚

#### 1.4 删除预设
```bash
asw preset remove development
```

#### 1.5 更新预设
```bash
asw preset update production --claude-code gpt-4-turbo
```

### User Story 2: 批量操作

#### 2.1 批量切换
```bash
# 切换所有工具到同一模型
asw batch switch --model gpt-4

# 切换指定工具列表
asw batch switch --agents claude-code,codex --model minimax
```

**功能要求**:
- 并发执行切换操作
- 汇总显示切换结果
- 失败继续执行其他工具
- 自动备份每个工具配置

#### 2.2 批量验证
```bash
asw batch validate
```

**输出**:
```
配置验证结果:

✓ claude-code: gpt-4 (配置文件正确)
✓ codex: gpt-4 (配置文件正确)
✗ gemini-cli: gpt-4 (模型不存在)
```

#### 2.3 批量状态查看
```bash
asw batch status
```

### User Story 3: 配置导入导出

#### 3.1 导出配置
```bash
# 导出单个预设
asw preset export development --output dev.json

# 导出所有预设
asw preset export-all --output presets-backup.json

# 导出当前配置
asw config export --output current-config.json
```

**导出格式**:
```json
{
  "version": "1.0",
  "exported_at": "2026-03-03T10:00:00Z",
  "presets": {
    "development": {
      "description": "开发环境使用 GLM",
      "tags": ["work", "glm"],
      "configurations": {
        "claude-code": "glm",
        "codex": "glm",
        "gemini-cli": "glm"
      }
    }
  },
  "models": {
    "glm": {
      "name": "glm",
      "base_url": "https://open.bigmodel.cn/api/v1",
      "model_id": "glm-4",
      "api_key": "sk-***"
    }
  }
}
```

#### 3.2 导入配置
```bash
# 导入预设
asw preset import dev.json --name development

# 导入时合并
asw preset import dev.json --merge --name development

# 导入时验证
asw preset import dev.json --validate
```

**功能要求**:
- 导入前验证配置格式
- 支持合并或覆盖策略
- 自动检查模型配置是否存在
- 导入失败不影响现有配置

### User Story 4: 配置验证增强

#### 4.1 预设验证
```bash
asw preset validate development
```

**验证项目**:
- 所有模型配置是否存在
- 工具是否已安装
- 配置文件是否可访问

#### 4.2 连通性测试（可选）
```bash
asw preset test development --test-api
```

**功能**:
- 测试 API 连通性
- 验证 API Key 有效性
- 显示测试结果

---

## 🏗️ 技术设计

### 数据结构

#### 预设结构
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub configurations: HashMap<String, String>, // agent -> model
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetCollection {
    pub version: String,
    pub presets: HashMap<String, Preset>,
    pub models: HashMap<String, ModelConfig>,
}
```

#### 配置导出结构
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigExport {
    pub version: String,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub presets: HashMap<String, Preset>,
    pub models: HashMap<String, ModelConfig>,
    pub current_active: HashMap<String, String>, // agent -> model
}
```

### 存储位置

```
~/.agentswitch/
├── config.toml              # 现有配置
├── models.toml              # 现有模型配置
├── presets.toml             # 新增：预设存储
└── exports/                 # 新增：导出目录
    ├── development.json
    └── backup-20260303.json
```

### CLI 命令设计

```rust
/// 预设管理命令
#[derive(Subcommand)]
pub enum PresetCommands {
    /// 创建配置预设
    Create {
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(long)]
        agents: Option<String>, // "claude-code,codex" or "all"
        #[arg(short, long)]
        model: String,
    },
    /// 列出所有预设
    List,
    /// 应用预设
    Apply {
        name: String,
        #[arg(long)]
        agents: Option<String>,
        #[arg(long)]
        backup: bool,
    },
    /// 更新预设
    Update {
        name: String,
        #[arg(long)]
        agent: String,
        #[arg(long)]
        model: String,
    },
    /// 删除预设
    Remove {
        name: String,
    },
    /// 导出预设
    Export {
        name: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// 导入预设
    Import {
        file: PathBuf,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(long)]
        merge: bool,
        #[arg(long)]
        validate: bool,
    },
    /// 验证预设
    Validate {
        name: String,
    },
}

/// 批量操作命令
#[derive(Subcommand)]
pub enum BatchCommands {
    /// 批量切换工具
    Switch {
        #[arg(long)]
        agents: Option<String>,
        #[arg(long)]
        model: String,
        #[arg(long)]
        backup: bool,
    },
    /// 批量验证配置
    Validate {
        #[arg(long)]
        agents: Option<String>,
    },
    /// 批量查看状态
    Status,
}
```

---

## 📝 任务清单 (初版)

### 阶段 1: 基础设施 (5 任务)
- [ ] T001 在 `src/preset/` 目录创建 preset 模块
- [ ] T002 定义 Preset 和 PresetCollection 数据结构
- [ ] T003 在 `src/config/` 扩展存储以支持预设
- [ ] T004 在 `src/config/store.rs` 实现 PresetManager
- [ ] T005 添加预设文件读写功能

### 阶段 2: 预设管理 CLI (8 任务)
- [ ] T006 实现 `asw preset create` 命令
- [ ] T007 实现 `asw preset list` 命令
- [ ] T008 实现 `asw preset apply` 命令
- [ ] T009 实现 `asw preset update` 命令
- [ ] T010 实现 `asw preset remove` 命令
- [ ] T011 实现 `asw preset export` 命令
- [ ] T012 实现 `asw preset import` 命令
- [ ] T013 实现 `asw preset validate` 命令

### 阶段段 3: 批量操作 (4 任务)
- [ ] T014 实现 `asw batch switch` 命令
- [ ] T015 实现 `asw batch validate` 命令
- [ ] T016 实现 `asw batch status` 命令
- [ ] T017 添加并发执行和错误处理

### 阶段 4: 配置导出导入 (3 任务)
- [ ] T018 扩展 `asw config export` 命令
- [ ] T019 实现 `asw config import` 命令
- [ ] T020 添加导出格式验证

### 阶段 5: 验证与测试 (5 任务)
- [ ] T021 预设验证功能
- [ ] T022 API 连通性测试（可选）
- [ ] T023 单元测试编写
- [ ] T024 集成测试编写
- [ ] T025 性能测试（批量操作）

### 阶段 6: 文档 (3 任务)
- [ ] T026 更新 README.md
- [ ] T027 添加预设使用文档
- [ ] T028 更新 CHANGELOG.md

---

## 🎯 优先级建议

### P1 (MVP) - 核心预设功能
- 预设创建/列表/删除
- 预设应用
- 基础导入导出

### P2 (增强) - 批量操作
- 批量切换
- 批量验证

### P3 (高级) - 额外功能
- API 连通性测试
- 预设版本管理
- 配置同步

---

## 📊 预期工作量

| 阶段 | 任务数 | 预估时间 |
|------|--------|----------|
| 阶段 1: 基础设施 | 5 | 2 天 |
| 阶段 2: 预设 CLI | 8 | 3 天 |
| 阶段 3: 批量操作 | 4 | 2 天 |
| 阶段 4: 导入导出 | 3 | 1 天 |
| 阶段 5: 测试 | 5 | 2 天 |
| 阶段 6: 文档 | 3 | 1 天 |
| **总计** | **28** | **11 天** |

---

## 🔄 与现有功能的兼容性

### 与 Spec 002 的集成

1. **复用适配器系统**
   - 使用现有的 AgentAdapter trait
   - 复用 ModelConfig 存储

2. **扩展现有命令**
   - `asw model` 保持不变
   - `asw switch` 添加 `--preset` 选项
   - `asw backup` 继续支持

3. **保持向后兼容**
   - v0.2.0 配置文件格式不变
   - 预设是额外的功能，不破坏现有工作流

---

## 💡 实现建议

### 技术选型

1. **存储格式**: 使用 TOML 存储预设（与现有 config.toml 一致）
2. **导出格式**: JSON（便于分享和版本控制）
3. **并发控制**: 使用 rayon 或 tokio 实现批量并发操作

### 开发阶段

**第一阶段** (v0.3.0 MVP):
- 核心预设管理（创建、列表、应用、删除）
- 基础导入导出
- 简单批量切换

**第二阶段** (v0.3.1):
- 批量验证
- 预设更新
- 高级导入选项

**第三阶段** (v0.3.2):
- API 测试
- 预设版本管理
- 配置同步（Phase 6 前置）

---

## 📋 讨论要点

### 需要确认的问题

1. **预设命名规则**
   - 是否允许命名空间？（如 `work/dev`, `team/project-x`）
   - 命名冲突处理？

2. **预设优先级**
   - 多个预设如何排序？
   - 是否支持默认预设？

3. **配置策略**
   - 预设应用时如何处理冲突？
   - 是否支持部分应用？

4. **导入策略**
   - 导入时模型不存在是否自动跳过？
   - 是否需要用户确认？

5. **批量操作**
   - 批量操作是否需要确认提示？
   - 失败时是否继续执行？

---

## 🚀 下一步行动

1. ✅ 审阅本规划
2. 📝 收集用户反馈
3. 📋 确认优先级
4. 🎯 开始详细设计（spec.md）
5. 🏗️ 实现开发（plan.md）
6. ✅ 测试与发布

---

**规划人员**: Claude Code (Sonnet 4.5)
**规划日期**: 2026-03-03
**目标版本**: AgentSwitch v0.3.0
