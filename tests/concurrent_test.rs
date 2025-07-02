#[cfg(test)]
mod tests {
    use rovkit::internal::concurrent_vec::ConcurrentVec;
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
