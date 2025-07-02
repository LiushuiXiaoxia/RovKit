use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

/// 加载 .env 文件（默认从当前目录）
pub fn load_dotenv() {
    let _ = dotenv();
}

/// 获取指定环境变量
pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// 获取环境变量并转换为指定类型
pub fn get_var_as<T: FromStr>(key: &str) -> Option<T> {
    env::var(key).ok()?.parse::<T>().ok()
}

/// 判断环境变量是否存在
pub fn contains_var(key: &str) -> bool {
    env::var(key).is_ok()
}

/// 获取所有环境变量为 HashMap
pub fn get_all_vars() -> HashMap<String, String> {
    env::vars().collect()
}
