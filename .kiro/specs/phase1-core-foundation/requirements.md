# 需求文档

## 简介

AgentSwitch Phase 1 核心基础功能，为 `asw` CLI 工具建立配置存储模块和 CLI 框架。本阶段实现模型配置的统一数据结构定义、配置文件（`~/.agentswitch/config.toml`）的读写操作、API Key 的安全存储，以及基础 CLI 命令（add/list/switch/remove）的完整实现，包含彩色输出和友好的错误提示。

## 术语表

- **ConfigStore**: 配置存储管理器，负责 `~/.agentswitch/config.toml` 文件的读写操作
- **ModelConfig**: 模型配置数据结构，包含名称、Base URL、API Key、Model ID 和额外参数
- **AppConfig**: 应用全局配置数据结构，包含所有模型配置列表和各 Agent 的当前活跃模型映射
- **CLI**: 命令行界面，`asw` 二进制程序的用户交互入口
- **TOML_Serializer**: TOML 格式序列化/反序列化模块，用于配置文件的持久化
- **Agent**: 代码终端代理工具（如 Claude Code、Codex、Gemini CLI 等）
- **Config_Directory**: 配置目录，固定路径为 `~/.agentswitch/`

## 需求

### 需求 1: 配置目录初始化

**用户故事:** 作为开发者，我希望在首次使用时自动创建配置目录，以便后续配置文件能正确存储。

#### 验收标准

1. WHEN `asw init` 命令被执行, THE ConfigStore SHALL 在 `~/.agentswitch/` 路径创建配置目录
2. WHEN 配置目录已存在, THE ConfigStore SHALL 跳过目录创建并继续正常运行
3. WHEN `~/.agentswitch/config.toml` 文件不存在, THE ConfigStore SHALL 创建包含空模型列表和空活跃模型映射的默认配置文件
4. IF 配置目录创建失败（如权限不足）, THEN THE CLI SHALL 输出包含具体失败原因的错误信息并以非零退出码退出

### 需求 2: 模型配置数据结构

**用户故事:** 作为开发者，我希望有一个统一的模型配置数据结构，以便在不同 Agent 工具间共享模型配置。

#### 验收标准

1. THE ModelConfig SHALL 包含以下必填字段：name（唯一标识符）、base_url（API 基础地址）、api_key（认证密钥）、model_id（模型标识）
2. THE ModelConfig SHALL 包含一个可选的 extra_params 字段，用于存储键值对形式的额外参数
3. THE AppConfig SHALL 包含一个 models 字段（ModelConfig 列表）和一个 active_models 字段（Agent 名称到模型名称的映射）
4. THE TOML_Serializer SHALL 将 AppConfig 序列化为合法的 TOML 格式字符串
5. THE TOML_Serializer SHALL 将合法的 TOML 格式字符串反序列化为 AppConfig 对象
6. FOR ALL 合法的 AppConfig 对象, 序列化后再反序列化 SHALL 产生与原始对象等价的结果（往返一致性）

### 需求 3: 配置文件读写

**用户故事:** 作为开发者，我希望配置能持久化到文件中，以便在不同会话间保留我的模型配置。

#### 验收标准

1. THE ConfigStore SHALL 从 `~/.agentswitch/config.toml` 文件读取配置
2. THE ConfigStore SHALL 将配置以 TOML 格式写入 `~/.agentswitch/config.toml` 文件
3. WHEN 配置文件不存在, THE ConfigStore SHALL 返回包含空模型列表的默认 AppConfig
4. IF 配置文件内容不是合法的 TOML 格式, THEN THE ConfigStore SHALL 返回包含文件路径和解析错误详情的错误信息
5. WHEN 配置被保存, THE ConfigStore SHALL 在写入前确保配置目录存在
6. IF 配置文件写入失败, THEN THE ConfigStore SHALL 返回包含具体 I/O 错误原因的错误信息

### 需求 4: API Key 安全存储

**用户故事:** 作为开发者，我希望 API Key 在配置文件中得到安全处理，以避免敏感信息泄露。

#### 验收标准

1. WHEN 模型配置被列出（`asw model list`）, THE CLI SHALL 对 API Key 进行掩码处理，仅显示前 4 个字符加 `****` 后缀
2. THE ConfigStore SHALL 将 API Key 以完整形式存储在配置文件中，以便 Agent 适配器能正确读取
3. WHEN 配置文件被保存, THE ConfigStore SHALL 将配置文件权限设置为仅所有者可读写（Unix 权限 0600）
4. IF 文件权限设置失败, THEN THE ConfigStore SHALL 输出警告信息但继续正常保存配置

### 需求 5: 添加模型配置命令

**用户故事:** 作为开发者，我希望通过命令行添加新的模型配置，以便快速注册新的 API 提供商。

#### 验收标准

1. WHEN `asw model add <name> --base-url <url> --api-key <key> --model <id>` 命令被执行, THE CLI SHALL 创建一个包含指定参数的 ModelConfig 并通过 ConfigStore 保存
2. WHEN 添加成功, THE CLI SHALL 以绿色文本输出包含模型名称的成功确认信息
3. IF 同名模型配置已存在, THEN THE CLI SHALL 以红色文本输出错误信息，提示该名称已被使用
4. IF name 参数为空字符串, THEN THE CLI SHALL 以红色文本输出错误信息，提示名称不能为空
5. IF base_url 参数不是合法的 URL 格式, THEN THE CLI SHALL 以红色文本输出错误信息，提示 URL 格式无效

### 需求 6: 列出模型配置命令

**用户故事:** 作为开发者，我希望查看所有已配置的模型，以便了解当前可用的配置。

#### 验收标准

1. WHEN `asw model list` 命令被执行, THE CLI SHALL 以表格形式显示所有已保存的模型配置
2. THE CLI SHALL 在表格中显示每个模型的 name、base_url、model_id 和掩码后的 api_key
3. WHEN 没有任何模型配置, THE CLI SHALL 以黄色文本输出提示信息，建议用户使用 `asw model add` 命令添加配置
4. WHILE 存在活跃模型映射, THE CLI SHALL 在对应模型名称旁标记活跃状态标识

### 需求 7: 删除模型配置命令

**用户故事:** 作为开发者，我希望删除不再需要的模型配置，以保持配置列表整洁。

#### 验收标准

1. WHEN `asw model remove <name>` 命令被执行且该名称存在, THE CLI SHALL 从 ConfigStore 中删除对应的模型配置
2. WHEN 删除成功, THE CLI SHALL 以绿色文本输出包含模型名称的成功确认信息
3. IF 指定名称的模型配置不存在, THEN THE CLI SHALL 以红色文本输出错误信息，提示未找到该配置
4. WHEN 被删除的模型正在被某个 Agent 使用（存在于 active_models 映射中）, THE CLI SHALL 同时清除 active_models 中对应的映射条目

### 需求 8: 编辑模型配置命令

**用户故事:** 作为开发者，我希望修改已有的模型配置，以便更新 API Key 或切换模型版本。

#### 验收标准

1. WHEN `asw model edit <name>` 命令被执行且该名称存在, THE CLI SHALL 显示当前配置值并允许用户通过命令行参数覆盖指定字段
2. THE CLI SHALL 支持 `--base-url`、`--api-key`、`--model` 可选参数，仅更新用户指定的字段
3. WHEN 编辑成功, THE CLI SHALL 以绿色文本输出包含模型名称的成功确认信息
4. IF 指定名称的模型配置不存在, THEN THE CLI SHALL 以红色文本输出错误信息，提示未找到该配置

### 需求 9: 彩色输出和错误提示

**用户故事:** 作为开发者，我希望 CLI 输出有清晰的视觉区分，以便快速识别操作结果。

#### 验收标准

1. THE CLI SHALL 使用绿色文本显示成功操作的输出信息
2. THE CLI SHALL 使用红色文本显示错误信息
3. THE CLI SHALL 使用黄色文本显示警告信息和空状态提示
4. THE CLI SHALL 使用蓝色文本显示信息性提示（如命令建议）
5. WHEN 操作失败, THE CLI SHALL 输出包含具体错误原因的描述性错误信息
6. THE CLI SHALL 在所有错误信息前添加 `✗` 前缀，在所有成功信息前添加 `✓` 前缀

### 需求 10: 配置文件 TOML 序列化往返一致性

**用户故事:** 作为开发者，我希望配置文件的读写过程不会丢失或篡改数据，以确保配置的可靠性。

#### 验收标准

1. FOR ALL 合法的 AppConfig 对象, THE TOML_Serializer SHALL 保证序列化为 TOML 字符串后再反序列化回 AppConfig 对象，所有字段值与原始对象一致
2. FOR ALL 合法的 ModelConfig 对象, THE TOML_Serializer SHALL 保证 extra_params 中的所有键值对在往返过程中保持不变
3. THE TOML_Serializer SHALL 将 AppConfig 格式化为人类可读的 TOML 格式（使用 pretty print）
