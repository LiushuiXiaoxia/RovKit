#[cfg(test)]
mod tests {
    use rovkit::collectionkit::HashMap;
    use rovkit::httpkit::*;

    #[test]
    fn test_get() {
        let resp = get("https://httpbin.org/get", None).unwrap();
        assert!(resp.contains("url"));
        println!("test_get: resp = {}", resp);
    }

    #[test]
    fn test_post_form() {
        let mut data = HashMap::new();
        data.insert("key", "value");
        let resp = post_form("https://httpbin.org/post", &data, None).unwrap();
        assert!(resp.contains("form"));

        println!("test_post_form: resp = {}", resp);
    }

    #[test]
    fn test_post_json() {
        let json = r#"{"key":"value"}"#;
        let resp = post_json("https://httpbin.org/post", json, None).unwrap();
        assert!(resp.contains("json"));

        println!("test_post_json: resp = {}", resp);
    }
}
