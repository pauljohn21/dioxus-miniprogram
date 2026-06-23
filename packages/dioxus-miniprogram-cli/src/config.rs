//! Configuration for Dioxus Mini Program projects

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project metadata
    pub project: ProjectConfig,
    
    /// Build configuration
    pub build: BuildConfig,
    
    /// Mini Program specific configuration
    pub miniprogram: MiniProgramConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    
    /// Project version
    pub version: String,
    
    /// Project description
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Target directory
    pub target_dir: Option<String>,
    
    /// Enable release mode
    pub release: bool,
    
    /// Enable Worker mode
    pub worker: bool,
    
    /// Enable source maps
    pub sourcemap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramConfig {
    /// App ID (optional, for production)
    pub app_id: Option<String>,
    
    /// Pages configuration
    pub pages: Vec<String>,
    
    /// Window configuration
    pub window: WindowConfig,
    
    /// Tab bar configuration
    pub tab_bar: Option<TabBarConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Navigation bar style
    pub navigation_bar_style: Option<String>,
    
    /// Background color
    pub background_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabBarConfig {
    /// Tab bar color
    pub color: String,
    
    /// Selected tab color
    pub selected_color: String,
    
    /// Tab bar list
    pub list: Vec<TabBarItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabBarItem {
    /// Page path
    pub page_path: String,
    
    /// Tab text
    pub text: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                name: "my-app".to_string(),
                version: "0.1.0".to_string(),
                description: Some("A Dioxus Mini Program app".to_string()),
            },
            build: BuildConfig {
                target_dir: None,
                release: false,
                worker: true,
                sourcemap: true,
            },
            miniprogram: MiniProgramConfig {
                app_id: None,
                pages: vec!["pages/index/index".to_string()],
                window: WindowConfig {
                    navigation_bar_style: Some("black".to_string()),
                    background_color: Some("#ffffff".to_string()),
                },
                tab_bar: None,
            },
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
