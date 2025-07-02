#[cfg(test)]
mod tests {
    use rovkit::envkit::*;
    use std::env;

    #[test]
    fn test_env_var() {
        unsafe {
            env::set_var("TEST_KEY", "value");
        }
        assert_eq!(get_var("TEST_KEY"), Some("value".to_string()));
        assert!(contains_var("TEST_KEY"));
    }

    #[test]
    fn test_get_all_vars() {
        let all = get_all_vars();
        assert!(all.contains_key("PATH") || all.contains_key("Path"));
    }

    #[test]
    fn test_var_as() {
        unsafe {
            env::set_var("TEST_PORT", "8080");
        }
        let port: Option<u16> = get_var_as("TEST_PORT");
        assert_eq!(port, Some(8080));
    }
}
