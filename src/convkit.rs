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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_number() {
        assert_eq!(str_to_number::<i32>("123"), Some(123));
        assert_eq!(str_to_number::<f64>("3.14"), Some(3.14));
        assert_eq!(str_to_number::<i32>("abc"), None);
    }

    #[test]
    fn test_number_to_str() {
        assert_eq!(number_to_str(456), "456");
        assert_eq!(number_to_str(7.89), "7.89");
    }

    #[test]
    fn test_bool_conversion() {
        assert_eq!(bool_to_str(true), "true");
        assert_eq!(str_to_bool("yes"), Some(true));
        assert_eq!(str_to_bool("0"), Some(false));
        assert_eq!(str_to_bool("maybe"), None);
    }
}
