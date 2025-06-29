use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use crate::cachekit::Cache;

pub struct FifoCache<K, V> {
    map: HashMap<K, V>,
    order: VecDeque<K>,
    cap: usize,
}

impl<K: Eq + Hash + Clone, V> FifoCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        FifoCache {
            map: HashMap::new(),
            order: VecDeque::new(),
            cap: capacity,
        }
    }
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> for FifoCache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            return;
        }

        if self.map.len() >= self.cap {
            if let Some(oldest) = self.order.pop_front() {
                self.map.remove(&oldest);
            }
        }

        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.order.retain(|k| k != key);
        self.map.remove(key)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn capacity(&self) -> usize {
        self.cap
    }
}
