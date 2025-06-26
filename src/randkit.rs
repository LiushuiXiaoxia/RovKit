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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_int() {
        for _ in 0..100 {
            let v = random_int(1, 10);
            assert!(v > 0 && v < 100);
            println!("test_random_int = {}", v)
        }
    }

    #[test]
    fn test_random_float() {
        for _ in 0..100 {
            let v = random_float(0.0, 1.0);
            assert!(v >= 0.0 && v < 1.0);
            println!("random_float = {}", v)
        }
    }

    #[test]
    fn test_random_bool() {
        let _v = random_bool(); // no assertion, just ensure no panic
        for _ in 0..100 {
            let s = random_bool();
            println!("random_bool = {}", s)
        }
    }

    #[test]
    fn test_random_string() {
        for _ in 0..100 {
            let s = random_string(20);
            assert_eq!(s.len(), 20);
            println!("random_string = {}", s)
        }
    }
}
