#[cfg(test)]
mod tests {
    use rovkit::stringkit::*;

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
