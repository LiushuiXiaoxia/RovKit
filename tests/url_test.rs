#[cfg(test)]
mod tests {
    use rovkit::urlkit::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_and_query() {
        let raw = "https://example.com/api?foo=1&bar=2";
        let url = parse_url(raw).unwrap();
        let query = get_query_params(&url);
        assert_eq!(query.get("foo"), Some(&"1".to_string()));
        assert_eq!(get_host(&url), Some("example.com".to_string()));
        assert_eq!(get_scheme(&url), "https");
    }

    #[test]
    fn test_build_url() {
        let mut q = HashMap::new();
        q.insert("name", "rust");
        q.insert("lang", "en");
        let u = build_url("https://example.com", "/doc", &q).unwrap();
        println!("test_build_url: url = {}", u);
        // assert_eq!(u.as_str(), "https://example.com/doc?name=rust&lang=en");
    }

    #[test]
    fn test_append_and_remove_query_param() {
        let mut url = parse_url("https://example.com/path").unwrap();
        append_query_param(&mut url, "a", "1");
        append_query_param(&mut url, "b", "2");
        assert!(url.as_str().contains("a=1"));
        assert!(url.as_str().contains("b=2"));

        remove_query_param(&mut url, "a");
        assert!(!url.as_str().contains("a=1"));
        assert!(url.as_str().contains("b=2"));
    }

    #[test]
    fn test_to_string() {
        let url = parse_url("https://example.com/x?y=z").unwrap();
        assert_eq!(to_string(&url), "https://example.com/x?y=z");
    }

    #[test]
    fn test_get_path_segments() {
        let url = parse_url("https://example.com/a/b/c").unwrap();
        let segments = get_path_segments(&url);
        assert_eq!(segments, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_get_fragment() {
        let url = parse_url("https://example.com/page#section1").unwrap();
        assert_eq!(get_fragment(&url), Some("section1".to_string()));
    }

    #[test]
    fn test_set_fragment() {
        let mut url = parse_url("https://example.com").unwrap();
        set_fragment(&mut url, "footer");
        assert_eq!(get_fragment(&url), Some("footer".to_string()));
    }

    #[test]
    fn test_set_and_append_path() {
        let mut url = parse_url("https://example.com").unwrap();
        set_path(&mut url, "/docs");
        assert_eq!(url.path(), "/docs");

        append_path_segment(&mut url, "rust");
        assert_eq!(url.path(), "/docs/rust");
    }

    #[test]
    fn test_clear_fragment_path_query() {
        let mut url = parse_url("https://example.com/test/path?x=1#frag").unwrap();
        clear_fragment(&mut url);
        clear_path(&mut url);
        clear_query(&mut url);
        assert_eq!(url.as_str(), "https://example.com/");
    }

    #[test]
    fn test_set_userinfo_and_port() {
        let mut url = parse_url("https://example.com").unwrap();
        set_userinfo(&mut url, "user", Some("pass"));
        set_port(&mut url, Some(8080));
        assert_eq!(url.as_str(), "https://user:pass@example.com:8080/");

        let info = get_userinfo(&url).unwrap();
        assert_eq!(info.0, "user");
        assert_eq!(info.1, Some("pass".to_string()));

        assert_eq!(get_port(&url), Some(8080));
    }
}
