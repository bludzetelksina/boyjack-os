// userland/lib/config.rs

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::io;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Возможные ошибки загрузки конфигурации
#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    ParseTomlError(toml::de::Error),
    ParseJsonError(serde_json::Error),
    UnsupportedFormat,
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseTomlError(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::ParseJsonError(err)
    }
}

/// Загружает конфигурацию из файла по пути `path`
/// Автоматически определяет формат по расширению (.toml или .json)
/// Возвращает десериализованную структуру `T`
pub fn load_config<T>(path: &Path) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path)?;

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("toml") => {
            let config = toml::from_str(&content)?;
            Ok(config)
        }
        Some("json") => {
            let config = serde_json::from_str(&content)?;
            Ok(config)
        }
        _ => Err(ConfigError::UnsupportedFormat),
    }
}

/// Пример структуры конфигурации
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub username: String,
    pub timeout: u64,
    pub debug: bool,
    pub servers: Vec<String>,
}

/// Пример использования
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_toml_config() {
        let toml_content = r#"
            username = "denis"
            timeout = 30
            debug = true
            servers = ["server1.local", "server2.local"]
        "#;

        let tmp_file = "/tmp/test_config.toml";
        std::fs::write(tmp_file, toml_content).unwrap();

        let config: AppConfig = load_config(Path::new(tmp_file)).unwrap();
        assert_eq!(config.username, "denis");
        assert_eq!(config.timeout, 30);
        assert_eq!(config.debug, true);
        assert_eq!(config.servers.len(), 2);
    }

    #[test]
    fn test_load_json_config() {
        let json_content = r#"
            {
                "username": "denis",
                "timeout": 45,
                "debug": false,
                "servers": ["json1.local", "json2.local"]
            }
        "#;

        let tmp_file = "/tmp/test_config.json";
        std::fs::write(tmp_file, json_content).unwrap();

        let config: AppConfig = load_config(Path::new(tmp_file)).unwrap();
        assert_eq!(config.username, "denis");
        assert_eq!(config.timeout, 45);
        assert_eq!(config.debug, false);
        assert_eq!(config.servers.len(), 2);
    }
}
