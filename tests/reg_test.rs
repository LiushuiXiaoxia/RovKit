#[cfg(test)]
mod tests {
    use rovkit::regkit::*;

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
