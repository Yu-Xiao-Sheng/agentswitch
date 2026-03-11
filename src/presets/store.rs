//! 预设存储模块
//!
//! 本模块提供预设的持久化存储功能。

use crate::presets::preset::Preset;
use anyhow::Result;

/// 预设集合
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PresetCollection {
    /// 版本号
    pub version: String,
    /// 所有预设
    #[serde(default)]
    pub presets: std::collections::HashMap<String, Preset>,
}

/// 预设存储
pub struct PresetStore {
    /// 配置目录
    config_dir: std::path::PathBuf,
    /// 预设文件路径
    presets_file: std::path::PathBuf,
}

impl PresetStore {
    /// 创建新的预设存储
    pub fn new() -> Result<Self> {
        let config_dir = dirs::home_dir()
            .map(|p| p.join(".agentswitch"))
            .ok_or_else(|| anyhow::anyhow!("无法获取主目录"))?;

        // 创建配置目录（如果不存在）
        std::fs::create_dir_all(&config_dir)?;

        let presets_file = config_dir.join("presets.toml");

        Ok(Self {
            config_dir,
            presets_file,
        })
    }

    /// 加载预设集合
    pub fn load(&self) -> Result<PresetCollection> {
        if !self.presets_file.exists() {
            // 返回空的预设集合
            return Ok(PresetCollection {
                version: "1.0.0".to_string(),
                presets: std::collections::HashMap::new(),
            });
        }

        let content = std::fs::read_to_string(&self.presets_file)?;
        let collection: PresetCollection = toml::from_str(&content)?;
        Ok(collection)
    }

    /// 保存预设集合
    pub fn save(&self, collection: &PresetCollection) -> Result<()> {
        // 原子写入：先写临时文件，然后重命名
        let tmp_file = self.presets_file.with_extension("tmp.toml");
        let content = toml::to_string_pretty(collection)?;

        std::fs::write(&tmp_file, content)?;

        // 在支持 atomic rename 的平台上，persist 会原子性地替换文件
        if cfg!(not(windows)) {
            std::fs::rename(&tmp_file, &self.presets_file)?;
        } else {
            // Windows 平台：先删除目标文件，然后重命名
            if self.presets_file.exists() {
                std::fs::remove_file(&self.presets_file)?;
            }
            std::fs::rename(&tmp_file, &self.presets_file)?;
        }

        Ok(())
    }

    /// 添加预设
    pub fn add_preset(&mut self, preset: Preset) -> Result<()> {
        let mut collection = self.load()?;

        if collection.presets.contains_key(&preset.name) {
            anyhow::bail!("预设名称已存在: {}", preset.name);
        }

        collection.presets.insert(preset.name.clone(), preset);
        self.save(&collection)?;
        Ok(())
    }

    /// 获取预设
    pub fn get_preset(&self, name: &str) -> Result<Preset> {
        let collection = self.load()?;
        collection
            .presets
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("预设不存在: {}", name))
    }

    /// 列出所有预设
    pub fn list_presets(&self) -> Result<Vec<Preset>> {
        let collection = self.load()?;
        let mut presets: Vec<_> = collection.presets.values().cloned().collect();
        presets.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(presets)
    }

    /// 更新预设
    pub fn update_preset(&mut self, preset: Preset) -> Result<()> {
        let mut collection = self.load()?;

        if !collection.presets.contains_key(&preset.name) {
            anyhow::bail!("预设不存在: {}", preset.name);
        }

        collection.presets.insert(preset.name.clone(), preset);
        self.save(&collection)?;
        Ok(())
    }

    /// 删除预设
    pub fn remove_preset(&mut self, name: &str) -> Result<()> {
        let mut collection = self.load()?;

        if collection.presets.remove(name).is_none() {
            anyhow::bail!("预设不存在: {}", name);
        }

        self.save(&collection)?;
        Ok(())
    }

    /// 按标签查找预设
    pub fn find_by_tag(&self, tag: &str) -> Result<Vec<Preset>> {
        let collection = self.load()?;
        let presets: Vec<_> = collection
            .presets
            .values()
            .filter(|p| p.tags.contains(&tag.to_string()))
            .cloned()
            .collect();
        Ok(presets)
    }
}
