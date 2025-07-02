#[cfg(test)]
mod tests {
    use rovkit::cachekit::fifo_cache;
    use rovkit::cachekit::lru_cache;
    #[test]

    fn test_cache() {
        {
            let mut fifo_cache = fifo_cache::<i32, &'static str>(2);
            fifo_cache.put(1, "one");
            fifo_cache.put(2, "two");
            fifo_cache.put(3, "three"); // 淘汰 key=1
            println!("{:?}", fifo_cache.get(&1)); // None
            assert_eq!(fifo_cache.get(&1), None);
            assert_eq!(fifo_cache.get(&2), Some(&"two"));
            assert_eq!(fifo_cache.get(&3), Some(&"three"))
        }

        {
            let mut lru_cache = lru_cache::<i32, &'static str>(2);
            lru_cache.put(1, "one");
            lru_cache.put(2, "two");
            lru_cache.get(&1); // 使用 key=1
            lru_cache.put(3, "three"); // 淘汰 key=2
            println!("{:?}", lru_cache.get(&2)); // None

            assert_eq!(lru_cache.get(&1), Some(&"one"));
            assert_eq!(lru_cache.get(&2), None);
            assert_eq!(lru_cache.get(&3), Some(&"three"))
        }
    }
}
