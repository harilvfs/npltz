use crate::error::NpltzError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let cfg = Config::default();
        assert_eq!(cfg.theme, None);
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let cfg = Config { theme: Some("dracula".into()) };
        let toml_str = toml::to_string(&cfg).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.theme.unwrap(), "dracula");
    }

    #[test]
    fn test_config_none_serializes() {
        let cfg = Config { theme: None };
        let toml_str = toml::to_string(&cfg).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.theme, None);
    }

    #[test]
    fn test_config_path_ends_correctly() {
        let path = Config::path();
        assert!(path.ends_with("npltz/config.toml"));
    }

    #[test]
    fn test_save_and_load() {
        let dir = std::env::temp_dir().join(format!("npltz_test_{}", std::process::id()));
        let test_path = dir.join("config.toml");
        let cfg = Config { theme: Some("nord".into()) };
        let parent = test_path.parent().unwrap();
        let _ = fs::create_dir_all(parent);
        let content = toml::to_string(&cfg).unwrap();
        fs::write(&test_path, &content).unwrap();
        let loaded: Config = fs::read_to_string(&test_path)
            .ok()
            .and_then(|c| toml::from_str(&c).ok())
            .unwrap_or_default();
        assert_eq!(loaded.theme.unwrap(), "nord");
        let _ = fs::remove_dir_all(&dir);
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: Option<String>,
}

impl Config {
    pub fn path() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        base.join("npltz").join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::path();
        fs::read_to_string(&path).ok().and_then(|c| toml::from_str(&c).ok()).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), NpltzError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string(self).map_err(|e| NpltzError::Config(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }
}
