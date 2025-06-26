use regex::Regex;

/// 判断字符串是否匹配指定正则
pub fn is_match(pattern: &str, text: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(text))
        .unwrap_or(false)
}

/// 查找所有匹配项
pub fn find_all(pattern: &str, text: &str) -> Vec<String> {
    Regex::new(pattern)
        .map(|re| re.find_iter(text).map(|m| m.as_str().to_string()).collect())
        .unwrap_or_default()
}

/// 查找第一个匹配项
pub fn find_first(pattern: &str, text: &str) -> Option<String> {
    Regex::new(pattern)
        .ok()
        .and_then(|re| re.find(text).map(|m| m.as_str().to_string()))
}

/// 提取第一个匹配项的所有分组（不包含整个匹配）
pub fn find_groups(pattern: &str, text: &str) -> Vec<String> {
    Regex::new(pattern)
        .ok()
        .and_then(|re| re.captures(text))
        .map(|caps| {
            (1..caps.len())
                .filter_map(|i| caps.get(i).map(|m| m.as_str().to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// 查找所有匹配项的分组集合
pub fn find_all_groups(pattern: &str, text: &str) -> Vec<Vec<String>> {
    Regex::new(pattern)
        .map(|re| {
            re.captures_iter(text)
                .map(|caps| {
                    (1..caps.len())
                        .filter_map(|i| caps.get(i).map(|m| m.as_str().to_string()))
                        .collect()
                })
                .collect()
        })
        .unwrap_or_default()
}

/// 替换所有匹配项
pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> String {
    Regex::new(pattern)
        .map(|re| re.replace_all(text, replacement).to_string())
        .unwrap_or_else(|_| text.to_string())
}

/// 替换第一个匹配项
pub fn replace_first(pattern: &str, text: &str, replacement: &str) -> String {
    Regex::new(pattern)
        .map(|re| re.replace(text, replacement).to_string())
        .unwrap_or_else(|_| text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        let text = "hello@example.com";
        let pattern = r"\w+@\w+\.\w+";
        assert!(is_match(pattern, text));
        assert!(!is_match(r"\d+", text));
    }

    #[test]
    fn test_find_all() {
        let text = "Emails: a@a.com, b@b.com";
        let pattern = r"\w+@\w+\.\w+";
        let result = find_all(pattern, text);
        assert_eq!(result, vec!["a@a.com", "b@b.com"]);
    }

    #[test]
    fn test_find_first() {
        let text = "Emails: a@a.com, b@b.com";
        let pattern = r"\w+@\w+\.\w+";
        let result = find_first(pattern, text);
        assert_eq!(result, Some("a@a.com".to_string()));
    }

    #[test]
    fn test_find_groups() {
        let text = "Email: user@domain.com";
        let pattern = r"(\w+)@(\w+)\.com";
        let result = find_groups(pattern, text);
        assert_eq!(result, vec!["user", "domain"]);
    }

    #[test]
    fn test_find_all_groups() {
        let text = "a@x.com b@y.com";
        let pattern = r"(\w+)@(\w+)\.com";
        let result = find_all_groups(pattern, text);
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "x".to_string()],
                vec!["b".to_string(), "y".to_string()]
            ]
        );
    }

    #[test]
    fn test_replace_all() {
        let text = "Replace a@a.com and b@b.com";
        let pattern = r"\w+@\w+\.\w+";
        let result = replace_all(pattern, text, "[email]");
        assert_eq!(result, "Replace [email] and [email]");
    }

    #[test]
    fn test_replace_first() {
        let text = "Replace a@a.com and b@b.com";
        let pattern = r"\w+@\w+\.\w+";
        let result = replace_first(pattern, text, "[email]");
        assert_eq!(result, "Replace [email] and b@b.com");
    }
}
