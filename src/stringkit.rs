use regex::Regex;

pub fn trim(s: &str) -> &str {
    s.trim()
}

pub fn to_lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn contains(s: &str, pat: &str) -> bool {
    s.contains(pat)
}

pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replacen(from, to, 1)
}

pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn regex_replace(s: &str, pattern: &str, repl: &str) -> Result<String, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.replace_all(s, repl).to_string())
}

pub fn split(s: &str, sep: &str) -> Vec<String> {
    s.split(sep).map(|v| v.to_string()).collect()
}

pub fn join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut uppercase_next = false;
    for c in s.chars() {
        if c == '_' {
            uppercase_next = true;
        } else if uppercase_next {
            result.extend(c.to_uppercase());
            uppercase_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
    }

    #[test]
    fn test_case() {
        assert_eq!(to_lower("ABC"), "abc");
        assert_eq!(to_upper("abc"), "ABC");
    }

    #[test]
    fn test_contains_prefix_suffix() {
        let s = "hello_world";
        assert!(contains(s, "_w"));
        assert!(starts_with(s, "hello"));
        assert!(ends_with(s, "world"));
    }

    #[test]
    fn test_replace() {
        assert_eq!(replace("a-b-c", "-", "+"), "a+b-c");
        assert_eq!(replace_all("a-b-c", "-", "+"), "a+b+c");
    }

    #[test]
    fn test_regex_replace() {
        let result = regex_replace("a  b   c", r"\s+", " ").unwrap();
        assert_eq!(result, "a b c");
    }

    #[test]
    fn test_split_join() {
        assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
        assert_eq!(join(&["a", "b", "c"], ","), "a,b,c");
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
    }

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(remove_whitespace(" a b\tc\n"), "abc");
    }
}
