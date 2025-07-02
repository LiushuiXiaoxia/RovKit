use std::str::FromStr;

/// 字符串转数字（支持整数和浮点）
pub fn str_to_number<T: FromStr>(s: &str) -> Option<T> {
    s.parse::<T>().ok()
}

/// 数字转字符串
pub fn number_to_str<T: ToString>(n: T) -> String {
    n.to_string()
}

/// 布尔转字符串
pub fn bool_to_str(b: bool) -> String {
    b.to_string()
}

/// 字符串转布尔
pub fn str_to_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None,
    }
}
