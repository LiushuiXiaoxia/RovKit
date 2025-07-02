use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ConcurrentVec<T> {
    inner: Arc<Mutex<Vec<T>>>,
}

impl<T> ConcurrentVec<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut vec = self.inner.lock().unwrap();
        vec.push(value);
    }

    pub fn get_snapshot(&self) -> Vec<T>
    where
        T: Clone,
    {
        let vec = self.inner.lock().unwrap();
        vec.clone()
    }

    pub fn arc(&self) -> Arc<Mutex<Vec<T>>> {
        Arc::clone(&self.inner)
    }
}

// 必须实现 Clone，才能在多线程中共享 ConcurrentVec
impl<T> Clone for ConcurrentVec<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
