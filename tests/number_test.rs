#[cfg(test)]
mod tests {
    use rovkit::numberkit::*;

    #[test]
    fn test_basic_checks() {
        assert!(is_integer(3.0));
        assert!(is_float(3.14));
        assert!(is_positive(1.0));
        assert!(is_negative(-1.0));
        assert!(is_zero(0.0));
        assert!(is_even(2));
        assert!(is_odd(3));
    }

    #[test]
    fn test_rounding() {
        assert_eq!(round(3.14159, 2), 3.14);
        assert_eq!(ceil(2.3), 3.0);
        assert_eq!(floor(2.9), 2.0);
        assert_eq!(abs(-5.0), 5.0);
    }

    #[test]
    fn test_formatting() {
        assert_eq!(format_float(3.14159, 3), "3.142");
        assert_eq!(format_percent(0.1234, 1), "12.3%");
        assert_eq!(format_thousands(1234567.89), "1,234,567.89");
    }

    #[test]
    fn test_parsing() {
        assert_eq!(parse_f64("3.14").unwrap(), 3.14);
        assert_eq!(parse_i64("42").unwrap(), 42);
        assert_eq!(to_string(123), "123");
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10.0, 2.0), Some(5.0));
        assert_eq!(safe_div(10.0, 0.0), None);
    }
}
