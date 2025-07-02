#[cfg(test)]
mod tests {
    use rovkit::cachekit::time_cache;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_cache() {
        let mut cache = time_cache(2);
        cache.put("hello", "world");
        println!("{:?}", cache.get(&"hello")); // Some("world")

        sleep(Duration::from_secs(3));
        println!("{:?}", cache.get(&"hello")); // None，已超时
        assert_eq!(cache.get(&"hello"), None);
    }
}