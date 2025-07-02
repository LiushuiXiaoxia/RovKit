use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 结构体：包含值和插入时间
struct CacheEntry<V> {
    value: V,
    inserted: Instant,
}

pub struct TimedCache<K, V> {
    map: HashMap<K, CacheEntry<V>>,
    timeout: Duration,
}

impl<K: std::cmp::Eq + std::hash::Hash + Clone, V> TimedCache<K, V> {
    pub fn new(timeout: Duration) -> Self {
        Self {
            map: HashMap::new(),
            timeout,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let entry = CacheEntry {
            value,
            inserted: Instant::now(),
        };
        self.map.insert(key, entry);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let expired = {
            if let Some(entry) = self.map.get(key) {
                entry.inserted.elapsed() >= self.timeout
            } else {
                return None;
            }
        };

        if expired {
            self.map.remove(key);
            None
        } else {
            // 安全再次访问
            self.map.get(key).map(|entry| &entry.value)
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|e| e.value)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }
}
