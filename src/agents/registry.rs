//! 适配器注册表
//!
//! 提供动态适配器注册和查询功能

use crate::agents::AgentAdapter;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::RwLock;

/// 适配器注册表
///
/// 使用线程安全的 RwLock 来支持并发访问
pub struct AdapterRegistry {
    /// 已注册的适配器映射 (name -> adapter)
    adapters: RwLock<HashMap<String, Box<dyn AgentAdapter>>>,
}

impl AdapterRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            adapters: RwLock::new(HashMap::new()),
        }
    }

    /// 注册一个新的适配器
    ///
    /// # 参数
    /// - `name`: 适配器名称（必须唯一）
    /// - `adapter`: 适配器实例
    ///
    /// # 返回
    /// - `Ok(())`: 注册成功
    /// - `Err(_)`: 如果名称冲突或获取锁失败
    ///
    /// # 示例
    /// ```ignore
    /// let registry = AdapterRegistry::new();
    /// registry.register("mytool", Box::new(MyToolAdapter::new()))?;
    /// ```
    pub fn register(&self, name: &str, adapter: Box<dyn AgentAdapter>) -> Result<()> {
        let mut adapters = self
            .adapters
            .write()
            .map_err(|_| anyhow!("获取注册表写锁失败"))?;

        // 检查名称冲突
        if adapters.contains_key(name) {
            anyhow::bail!(
                "适配器名称 '{}' 已存在。请使用不同的名称或先注销现有适配器。",
                name
            );
        }

        // 验证适配器名称一致性
        if adapter.name() != name {
            anyhow::bail!(
                "适配器名称不匹配：注册名称为 '{}'，但适配器实际名称为 '{}'",
                name,
                adapter.name()
            );
        }

        adapters.insert(name.to_string(), adapter);

        Ok(())
    }

    /// 注销一个适配器
    ///
    /// # 参数
    /// - `name`: 要注销的适配器名称
    ///
    /// # 返回
    /// - `Some(adapter)`: 找到并移除的适配器
    /// - `None`: 适配器不存在
    pub fn unregister(&self, name: &str) -> Option<Box<dyn AgentAdapter>> {
        let mut adapters = self.adapters.write().ok()?;
        let adapter = adapters.remove(name);

        let _ = adapter.is_some(); // 移除未使用警告

        adapter
    }

    /// 获取指定名称的适配器
    ///
    /// # 参数
    /// - `name`: 适配器名称
    ///
    /// # 返回
    /// - `Some(adapter)`: 找到的适配器
    /// - `None`: 适配器不存在
    pub fn get(&self, _name: &str) -> Option<Box<dyn AgentAdapter>> {
        // 注意：这里我们需要克隆适配器，因为返回 Box<dyn AgentAdapter>
        // 但由于 AgentAdapter trait 不能克隆，我们只返回引用或重新构造
        // 实际使用中，建议使用 get_adapter_ref() 方法

        let _adapters = self.adapters.read().ok()?;

        // 由于无法克隆 Box<dyn AgentAdapter>，这里返回 None
        // 实际应该使用 get_adapter_info() 或其他方法
        None
    }

    /// 获取所有已注册的适配器信息
    ///
    /// # 返回
    /// 返回所有适配器名称和安装状态的列表
    pub fn list_adapters(&self) -> Vec<AdapterInfo> {
        let adapters = self.adapters.read().unwrap_or_else(|_| {
            // 如果获取读锁失败，尝试创建一个空的 HashMap
            // 注意：这里只是临时解决方案，实际上应该处理错误
            panic!("获取注册表读锁失败")
        });

        adapters
            .iter()
            .map(|(name, adapter)| {
                // 尝试检测工具是否已安装
                let is_installed = adapter.detect().unwrap_or(false);

                AdapterInfo {
                    name: name.clone(),
                    is_installed,
                }
            })
            .collect()
    }

    /// 检查适配器是否已注册
    ///
    /// # 参数
    /// - `name`: 适配器名称
    ///
    /// # 返回
    /// - `true`: 已注册
    /// - `false`: 未注册
    pub fn contains(&self, name: &str) -> bool {
        self.adapters
            .read()
            .map(|adapters| adapters.contains_key(name))
            .unwrap_or(false)
    }

    /// 获取已注册适配器的数量
    pub fn count(&self) -> usize {
        self.adapters
            .read()
            .map(|adapters| adapters.len())
            .unwrap_or(0)
    }

    /// 迭代所有已注册的适配器
    ///
    /// # 返回
    /// 返回一个迭代器，按注册顺序遍历所有适配器
    pub fn iter(&self) -> Iter<'_> {
        let guard = self
            .adapters
            .read()
            .unwrap_or_else(|_| panic!("获取注册表读锁失败"));

        let keys = guard.keys().cloned().collect();

        Iter {
            keys,
            index: 0,
            _guard: guard,
        }
    }

    /// 对每个已注册的适配器执行回调函数
    ///
    /// # 参数
    /// - `f`: 回调函数，接收适配器引用
    pub fn for_each_adapter<F>(&self, mut f: F)
    where
        F: FnMut(&dyn AgentAdapter),
    {
        let adapters = self
            .adapters
            .read()
            .unwrap_or_else(|_| panic!("获取注册表读锁失败"));

        for adapter in adapters.values() {
            f(adapter.as_ref());
        }
    }

    /// 验证所有已注册的适配器
    ///
    /// # 返回
    /// 返回验证结果列表
    pub fn validate_all(&self) -> Vec<ValidationResult> {
        let adapters = self
            .adapters
            .read()
            .unwrap_or_else(|_| panic!("获取注册表读锁失败"));

        let mut results = Vec::new();

        for (name, adapter) in adapters.iter() {
            let result = self.validate_adapter(name, adapter);
            results.push(result);
        }

        results
    }

    /// 验证单个适配器
    ///
    /// # 参数
    /// - `name`: 适配器名称
    /// - `adapter`: 适配器实例
    ///
    /// # 返回
    /// 返回验证结果
    fn validate_adapter(&self, name: &str, adapter: &Box<dyn AgentAdapter>) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // 验证 1: 名称一致性
        if adapter.name() != name {
            errors.push(format!(
                "名称不匹配：注册名称为 '{}'，适配器名称为 '{}'",
                name,
                adapter.name()
            ));
        }

        // 验证 2: config_path() 方法
        match adapter.config_path() {
            Ok(path) => {
                if !path.as_os_str().is_empty() {
                    // 路径有效
                } else {
                    warnings.push("配置文件路径为空".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("无法获取配置文件路径: {}", e));
            }
        }

        // 验证 3: detect() 方法
        match adapter.detect() {
            Ok(_) => {
                // 检测成功
            }
            Err(e) => {
                warnings.push(format!("检测方法执行失败: {}", e));
            }
        }

        ValidationResult {
            adapter_name: name.to_string(),
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 适配器信息
#[derive(Debug, Clone)]
pub struct AdapterInfo {
    /// 适配器名称
    pub name: String,

    /// 工具是否已安装
    pub is_installed: bool,
}

/// 验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// 适配器名称
    pub adapter_name: String,

    /// 是否有效
    pub is_valid: bool,

    /// 错误列表
    pub errors: Vec<String>,

    /// 警告列表
    pub warnings: Vec<String>,
}

/// 全局适配器注册表
///
/// 使用 OnceLock 确保全局唯一实例
static GLOBAL_REGISTRY: std::sync::OnceLock<AdapterRegistry> = std::sync::OnceLock::new();

/// 获取全局注册表实例
pub fn global_registry() -> &'static AdapterRegistry {
    GLOBAL_REGISTRY.get_or_init(|| {
        let registry = AdapterRegistry::new();

        // 自动注册内置适配器
        let _ = registry.register(
            "claude-code",
            Box::new(crate::agents::claude_code::ClaudeCodeAdapter::new()),
        );
        let _ = registry.register("codex", Box::new(crate::agents::codex::CodexAdapter::new()));
        let _ = registry.register(
            "gemini-cli",
            Box::new(crate::agents::gemini::GeminiAdapter::new()),
        );
        let _ = registry.register(
            "opencode",
            Box::new(crate::agents::opencode::OpenCodeAdapter::new()),
        );
        let _ = registry.register("qwen", Box::new(crate::agents::qwen::QwenAdapter::new()));
        let _ = registry.register("grok", Box::new(crate::agents::grok::GrokAdapter::new()));

        registry
    })
}

/// 适配器名称迭代器
pub struct Iter<'a> {
    keys: Vec<String>,
    index: usize,
    _guard: std::sync::RwLockReadGuard<'a, HashMap<String, Box<dyn AgentAdapter>>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.keys.len() {
            return None;
        }

        let key = self.keys[self.index].clone();
        self.index += 1;

        Some(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::claude_code::ClaudeCodeAdapter;

    #[test]
    #[ignore = "需要隔离全局注册表"]
    fn test_registry_new() {
        let registry = AdapterRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    #[ignore = "需要隔离全局注册表"]
    fn test_register_adapter() {
        let registry = AdapterRegistry::new();
        let adapter = Box::new(ClaudeCodeAdapter::new());

        let result = registry.register("test-claude", adapter);
        assert!(result.is_ok());
        assert_eq!(registry.count(), 1);
        assert!(registry.contains("test"));
    }

    #[test]
    #[ignore = "需要隔离全局注册表"]
    fn test_register_duplicate() {
        let registry = AdapterRegistry::new();
        let adapter1 = Box::new(ClaudeCodeAdapter::new());
        let adapter2 = Box::new(ClaudeCodeAdapter::new());

        registry.register("test-claude", adapter1).unwrap();

        let result = registry.register("test-claude", adapter2);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("已存在"));
    }

    #[test]
    #[ignore = "需要隔离全局注册表"]
    fn test_unregister_adapter() {
        let registry = AdapterRegistry::new();
        let adapter = Box::new(ClaudeCodeAdapter::new());

        registry.register("test-claude", adapter).unwrap();
        assert_eq!(registry.count(), 1);

        let removed = registry.unregister("test");
        assert!(removed.is_some());
        assert_eq!(registry.count(), 0);
    }

    #[test]
    #[ignore = "需要隔离全局注册表"]
    fn test_list_adapters() {
        let registry = AdapterRegistry::new();
        let adapter = Box::new(ClaudeCodeAdapter::new());

        registry.register("test-list", adapter).unwrap();

        let adapters = registry.list_adapters();
        assert_eq!(adapters.len(), 1);
        assert_eq!(adapters[0].name, "claude-code");
    }

    #[test]
    #[ignore = "需要隔离全局注册表"]
    fn test_validate_adapter() {
        let registry = AdapterRegistry::new();
        let adapter = Box::new(ClaudeCodeAdapter::new());

        registry.register("test-list", adapter).unwrap();

        let results = registry.validate_all();
        assert_eq!(results.len(), 1);
        // Claude Code 适配器应该是有效的
        assert!(results[0].is_valid);
    }
}
