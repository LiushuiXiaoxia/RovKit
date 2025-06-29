use crate::cachekit::Cache;
use std::collections::{BTreeMap, VecDeque};

pub struct LruCache<K, V> {
    map: BTreeMap<K, V>,
    order: VecDeque<K>,
    cap: usize,
}

impl<K: Ord + Clone + PartialEq, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: BTreeMap::new(),
            order: VecDeque::new(),
            cap: capacity,
        }
    }

    fn update_order(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.order.push_back(key.clone());
    }
}

impl<K: Ord + Clone + PartialEq + std::hash::Hash, V> Cache<K, V> for LruCache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.update_order(key);
        }
        self.map.get(key)
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.update_order(&key);
            return;
        }

        if self.map.len() >= self.cap {
            if let Some(lru_key) = self.order.pop_front() {
                self.map.remove(&lru_key);
            }
        }

        self.map.insert(key.clone(), value);
        self.order.push_back(key);
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
