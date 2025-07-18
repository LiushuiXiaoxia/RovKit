#[cfg(test)]
mod tests {
    use rovkit::randkit::*;

    #[test]
    fn test_random_int() {
        for _ in 0..100 {
            let v = random_int(1, 10);
            assert!(v > 0 && v < 100);
            println!("test_random_int = {}", v)
        }
    }

    #[test]
    fn test_random_float() {
        for _ in 0..100 {
            let v = random_float(0.0, 1.0);
            assert!(v >= 0.0 && v < 1.0);
            println!("random_float = {}", v)
        }
    }

    #[test]
    fn test_random_bool() {
        let _v = random_bool(); // no assertion, just ensure no panic
        for _ in 0..100 {
            let s = random_bool();
            println!("random_bool = {}", s)
        }
    }

    #[test]
    fn test_random_string() {
        for _ in 0..100 {
            let s = random_string(20);
            assert_eq!(s.len(), 20);
            println!("random_string = {}", s)
        }
    }
}
