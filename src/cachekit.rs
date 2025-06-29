use crate::internal::cache_fifo::FifoCache;
use crate::internal::cache_lru::LruCache;
use std::boxed::Box;

pub trait Cache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}

pub fn fifo_cache<K, V>(capacity: usize) -> Box<dyn Cache<K, V>>
where
    K: 'static + Clone + std::hash::Hash + Eq,
    V: 'static,
{
    Box::new(FifoCache::new(capacity))
}

pub fn lru_cache<K, V>(capacity: usize) -> Box<dyn Cache<K, V>>
where
    K: 'static + Clone + std::hash::Hash + Ord + PartialEq,
    V: 'static,
{
    Box::new(LruCache::new(capacity))
}

#[cfg(test)]
mod tests {
    use super::*;

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
