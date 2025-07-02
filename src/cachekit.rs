use crate::cache::cache_fifo::FifoCache;
use crate::cache::cache_lru::LruCache;
use crate::cache::cache_time::TimedCache;
use std::boxed::Box;
use std::time::Duration;

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

pub fn time_cache<K, V>(timeout_secs: u64) -> TimedCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: 'static,
{
    TimedCache::new(Duration::from_secs(timeout_secs))
}
