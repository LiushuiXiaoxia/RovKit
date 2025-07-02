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
