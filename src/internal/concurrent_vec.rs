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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_push() {
        let concurrent_vec = ConcurrentVec::new();

        let mut handles = vec![];

        for i in 0..10 {
            let cv = concurrent_vec.clone();
            let handle = thread::spawn(move || {
                cv.push(i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snapshot = concurrent_vec.get_snapshot();
        assert_eq!(snapshot.len(), 10);

        // 检查是否包含所有 0..10 的元素（顺序不保证）
        for i in 0..10 {
            assert!(snapshot.contains(&i));
        }

        println!("concurrent_vec = {:?}", concurrent_vec)
    }
}
