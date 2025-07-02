use serde::de::DeserializeOwned;
use std::fs;
use std::io::{self};
use std::path::Path;

/// 从 JSON 文件加载配置结构体
pub fn from_json_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let contents = fs::read_to_string(path)?;
    let config = serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(config)
}

/// 从 YAML 文件加载配置结构体（需要启用 serde_yaml）
#[cfg(feature = "yaml")]
pub fn from_yaml_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let contents = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(config)
}
