# AgentSwitch 迭代计划 - v0.6.0

## 目标功能

1. **Cursor CLI 适配器**
2. **配置加密系统**
3. **Git 同步完善（含加密）**

---

## 任务拆解

### 阶段一：配置加密系统（基础设施）

#### 1.1 加密模块设计
- [ ] 设计加密方案（AES-256-GCM + Argon2 密钥派生）
- [ ] 定义密钥存储位置：`~/.agentswitch/keys/master.key`
- [ ] 定义加密配置格式

#### 1.2 密钥管理
- [ ] `asw crypto keygen` - 生成新密钥
- [ ] `asw crypto key-export` - 导出密钥（Base64）
- [ ] `asw crypto key-import` - 导入密钥
- [ ] `asw crypto status` - 查看加密状态

#### 1.3 配置加密/解密
- [ ] 加密 ModelConfig 中的 api_key 字段
- [ ] 加密预设中的敏感信息
- [ ] 透明加解密（读写时自动处理）

---

### 阶段二：Git 同步完善

#### 2.1 同步初始化
- [ ] `asw sync init` - 初始化 Git 仓库 + 生成/导入密钥
- [ ] 提示用户保存密钥

#### 2.2 推送功能
- [ ] `asw sync remote add <url>` - 添加远程仓库
- [ ] `asw sync push` - 加密配置 + 提交 + 推送

#### 2.3 拉取功能
- [ ] `asw sync pull` - 拉取 + 解密配置
- [ ] 检查密钥是否存在，不存在则提示导入

#### 2.4 状态查看
- [ ] `asw sync status` - 查看同步状态、密钥状态

#### 2.5 冲突处理
- [ ] 检测本地/远程冲突
- [ ] 提供解决策略

---

### 阶段三：Cursor CLI 适配器

#### 3.1 调研
- [ ] 确定 Cursor CLI 配置文件路径
- [ ] 确定配置文件格式
- [ ] 确定支持的协议

#### 3.2 实现
- [ ] 创建 `src/agents/cursor.rs`
- [ ] 实现 AgentAdapter trait
- [ ] 注册到 registry

---

### 阶段四：Docker 测试

#### 4.1 测试环境
- [ ] 更新 Dockerfile 安装 Cursor CLI
- [ ] 准备测试脚本

#### 4.2 功能测试
- [ ] 测试密钥生成/导出/导入
- [ ] 测试配置加密/解密
- [ ] 测试 Git 同步（push/pull）
- [ ] 测试 Cursor CLI 适配器

#### 4.3 边界测试
- [ ] 密钥丢失后的错误处理
- [ ] 新机器导入密钥流程
- [ ] 冲突解决

---

## 加密方案详细设计

### 密钥格式
```
~/.agentswitch/keys/
├── master.key      # 主密钥（32字节，Base64编码）
└── master.key.pub  # 公钥信息（可选，用于验证）
```

### 加密流程
1. 用户运行 `asw sync init`
2. 检查密钥是否存在
3. 不存在则生成新密钥
4. 显示密钥（Base64），提示用户保存
5. 所有敏感字段使用 AES-256-GCM 加密
6. 加密后配置提交到 Git

### 解密流程
1. 用户运行 `asw sync pull`
2. 检查密钥是否存在
3. 不存在则提示导入密钥
4. 使用密钥解密配置

### 密钥派生
```
master_key (32 bytes)
    ↓
argon2(password=user_password, salt=random)
    ↓
encryption_key (32 bytes)
```

---

## 执行顺序

1. **阶段一** → 配置加密系统（其他功能依赖此）
2. **阶段三** → Cursor CLI 适配器（独立）
3. **阶段二** → Git 同步完善（依赖加密）
4. **阶段四** → Docker 测试（全部完成后）

---

## 预估时间

| 阶段 | 预估时间 |
|------|----------|
| 阶段一：加密系统 | 2-3 小时 |
| 阶段三：Cursor 适配器 | 1 小时 |
| 阶段二：Git 同步 | 2-3 小时 |
| 阶段四：Docker 测试 | 1 小时 |
| **总计** | **6-8 小时** |

---

*创建时间: 2026-03-31 14:08*
