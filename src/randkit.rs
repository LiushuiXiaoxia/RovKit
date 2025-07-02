use rand::{random, Rng};

/// 生成随机整数 [min, max]
pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

/// 生成随机浮点数 [min, max)
pub fn random_float(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

/// 生成随机布尔值
pub fn random_bool() -> bool {
    random()
}

/// 生成随机字符串（仅包含字母数字）
pub fn random_string(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
