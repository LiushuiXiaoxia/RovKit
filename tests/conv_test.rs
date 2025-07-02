#[cfg(test)]
mod tests {
    use super::*;
    use rovkit::convkit::*;

    #[test]
    fn test_str_to_number() {
        assert_eq!(str_to_number::<i32>("123"), Some(123));
        assert_eq!(str_to_number::<f64>("3.14"), Some(3.14));
        assert_eq!(str_to_number::<i32>("abc"), None);
    }

    #[test]
    fn test_number_to_str() {
        assert_eq!(number_to_str(456), "456");
        assert_eq!(number_to_str(7.89), "7.89");
    }

    #[test]
    fn test_bool_conversion() {
        assert_eq!(bool_to_str(true), "true");
        assert_eq!(str_to_bool("yes"), Some(true));
        assert_eq!(str_to_bool("0"), Some(false));
        assert_eq!(str_to_bool("maybe"), None);
    }
}
