use std::collections::HashMap;
use url::{ParseError, Url as InnerUrl};

pub type Url = InnerUrl;

/// 解析 URL 并返回 Url 对象
pub fn parse_url(input: &str) -> Result<Url, ParseError> {
    Url::parse(input)
}

/// 获取 query 参数（? 后的键值对）
pub fn get_query_params(url: &Url) -> HashMap<String, String> {
    url.query_pairs().into_owned().collect::<HashMap<_, _>>()
}

/// 获取 host（不含 schema）
pub fn get_host(url: &Url) -> Option<String> {
    url.host_str().map(|s| s.to_string())
}

/// 获取 scheme（http/https 等）
pub fn get_scheme(url: &Url) -> String {
    url.scheme().to_string()
}

/// 构造 URL（传入 base、路径、query）
pub fn build_url(base: &str, path: &str, query: &HashMap<&str, &str>) -> Result<Url, ParseError> {
    let mut url = Url::parse(base)?;
    url.set_path(path);
    url.query_pairs_mut().clear();
    for (k, v) in query {
        url.query_pairs_mut().append_pair(k, v);
    }
    Ok(url)
}

/// 追加 query 参数
pub fn append_query_param(url: &mut Url, key: &str, value: &str) {
    url.query_pairs_mut().append_pair(key, value);
}

/// 移除 query 参数
pub fn remove_query_param(url: &mut Url, key: &str) {
    let pairs: Vec<(String, String)> = url.query_pairs().into_owned().collect();
    url.query_pairs_mut().clear();
    for (k, v) in pairs {
        if k != key {
            url.query_pairs_mut().append_pair(&k, &v);
        }
    }
}

/// 转换为字符串
pub fn to_string(url: &Url) -> String {
    url.as_str().to_string()
}

/// 获取路径 segments
pub fn get_path_segments(url: &Url) -> Vec<String> {
    url.path_segments()
        .map(|segments| segments.map(|s| s.to_string()).collect())
        .unwrap_or_else(Vec::new)
}

/// 获取 fragment（#后部分）
pub fn get_fragment(url: &Url) -> Option<String> {
    url.fragment().map(|s| s.to_string())
}

/// 设置 fragment（#部分）
pub fn set_fragment(url: &mut Url, fragment: &str) {
    url.set_fragment(Some(fragment));
}

/// 清除 fragment
pub fn clear_fragment(url: &mut Url) {
    url.set_fragment(None);
}

/// 设置 path（替换整个路径）
pub fn set_path(url: &mut Url, path: &str) {
    url.set_path(path);
}

/// 清除 path
pub fn clear_path(url: &mut Url) {
    url.set_path("");
}

/// 清除所有 query 参数
pub fn clear_query(url: &mut Url) {
    url.set_query(None);
}

/// 追加 path 片段
pub fn append_path_segment(url: &mut Url, segment: &str) {
    let mut segments: Vec<String> = get_path_segments(url);
    segments.push(segment.to_string());
    let new_path = format!("/{}", segments.join("/"));
    url.set_path(&new_path);
}

/// 设置用户名和密码（userinfo）
pub fn set_userinfo(url: &mut Url, username: &str, password: Option<&str>) {
    url.set_username(username).ok();
    if let Some(pw) = password {
        url.set_password(Some(pw)).ok();
    } else {
        url.set_password(None).ok();
    }
}

/// 设置端口号
pub fn set_port(url: &mut Url, port: Option<u16>) {
    url.set_port(port).ok();
}

/// 获取用户名和密码（userinfo）
pub fn get_userinfo(url: &Url) -> Option<(String, Option<String>)> {
    let username = url.username();
    if username.is_empty() {
        return None;
    }
    Some((username.to_string(), url.password().map(|s| s.to_string())))
}

/// 获取端口号
pub fn get_port(url: &Url) -> Option<u16> {
    url.port_or_known_default()
}
