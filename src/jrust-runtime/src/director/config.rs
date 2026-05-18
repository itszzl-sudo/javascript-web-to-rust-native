use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildConfig {
    pub project: ProjectConfig,
    pub output: OutputConfig,
    pub communication: CommunicationConfig,
    pub resources: ResourceConfig,
    pub build: BuildSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildSettings {
    pub mode: BuildMode,
    pub compile_lib: bool,
    pub compile_exe: bool,
    pub generate_snap: bool,
    pub split_code: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BuildMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "code_only")]
    CodeOnly,
}

impl Default for BuildSettings {
    fn default() -> Self {
        Self {
            mode: BuildMode::Full,
            compile_lib: true,
            compile_exe: true,
            generate_snap: true,
            split_code: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    pub base_dir: PathBuf,
    pub source_dir: String,
    pub lib_dir: String,
    pub final_dir: String,
    pub exe_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunicationConfig {
    pub mode: String,
    pub server_addr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceConfig {
    pub favicon: Option<String>,
    pub icon: Option<String>,
    pub resource_extensions: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            project: ProjectConfig {
                name: "jrust-app".to_string(),
                title: "JRust Application".to_string(),
                description: None,
                version: "0.1.0".to_string(),
            },
            output: OutputConfig {
                base_dir: PathBuf::from("dist"),
                source_dir: "1_source".to_string(),
                lib_dir: "2_lib".to_string(),
                final_dir: "3_final".to_string(),
                exe_name: None,
            },
            communication: CommunicationConfig {
                mode: "direct".to_string(),
                server_addr: Some("127.0.0.1:8080".to_string()),
            },
            resources: ResourceConfig {
                favicon: None,
                icon: None,
                resource_extensions: vec![
                    "png".to_string(),
                    "jpg".to_string(),
                    "jpeg".to_string(),
                    "gif".to_string(),
                    "svg".to_string(),
                    "woff".to_string(),
                    "woff2".to_string(),
                    "ttf".to_string(),
                    "eot".to_string(),
                    "css".to_string(),
                ],
            },
            build: BuildSettings::default(),
        }
    }
}

impl BuildConfig {
    pub fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }
    
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }
    
    pub fn get_icon_path(&self) -> Option<String> {
        self.resources.icon.clone().or_else(|| self.resources.favicon.clone())
    }
    
    pub fn get_final_exe_name(&self) -> String {
        self.output.exe_name.clone().unwrap_or_else(|| self.project.name.clone())
    }
}

use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = BuildConfig::default();
        assert_eq!(config.project.name, "jrust-app");
        assert_eq!(config.project.title, "JRust Application");
        assert_eq!(config.output.source_dir, "1_source");
    }
    
    #[test]
    fn test_get_icon_path() {
        let mut config = BuildConfig::default();
        assert!(config.get_icon_path().is_none());
        
        config.resources.favicon = Some("favicon.ico".to_string());
        assert_eq!(config.get_icon_path(), Some("favicon.ico".to_string()));
        
        config.resources.icon = Some("icon.png".to_string());
        assert_eq!(config.get_icon_path(), Some("icon.png".to_string()));
    }
}
