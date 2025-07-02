use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::time::Duration;

/// 执行 GET 请求
pub fn get(url: &str, headers: Option<HeaderMap>) -> reqwest::Result<String> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let mut req = client.get(url);
    if let Some(h) = headers {
        req = req.headers(h);
    }

    let resp = req.send()?.text()?;
    Ok(resp)
}

/// 执行 POST 请求（form 表单）
pub fn post_form(
    url: &str,
    form_data: &HashMap<&str, &str>,
    headers: Option<HeaderMap>,
) -> reqwest::Result<String> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let mut req = client.post(url).form(form_data);
    if let Some(h) = headers {
        req = req.headers(h);
    }

    let resp = req.send()?.text()?;
    Ok(resp)
}

/// 执行 POST 请求（application/json）
pub fn post_json(
    url: &str,
    json_body: &str,
    headers: Option<HeaderMap>,
) -> reqwest::Result<String> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let mut req = client.post(url).body(json_body.to_string());
    if let Some(h) = headers {
        req = req.headers(h);
    }

    let resp = req
        .header("Content-Type", "application/json")
        .send()?
        .text()?;
    Ok(resp)
}
