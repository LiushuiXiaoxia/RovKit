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
