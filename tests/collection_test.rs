#[cfg(test)]
mod tests {
    use rovkit::collectionkit::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_array_list() {
        {
            let mut list = ArrayList::<i32>::new();
            list.push(1);
            list.push(2);
            list.push(3);
            assert_eq!(list.len(), 3);
            assert_eq!(list[0], 1);
            assert_eq!(list[1], 2);
            assert_eq!(list[2], 3);
        }
        {
            let list = ArrayList::<i32>::from(vec![1, 2, 3]);
            println!("{:?}", list);
            assert_eq!(list.len(), 3);
        }
        {
            let mut list = ArrayList::<i32>::from([1, 2, 3]);
            list.clear();
            assert_eq!(list.len(), 0)
        }

        {
            let list = ArrayList::<i32>::from([1, 2, 3]);
            assert_eq!(list.contains(&1), true);
        }
    }

    #[test]
    fn test_linked_list() {
        {
            let mut list = LinkedList::<i32>::new();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);
            assert_eq!(list.len(), 3);
        }
        {
            let list = LinkedList::<i32>::from([1, 2, 3]);
            println!("{:?}", list);
            assert_eq!(list.len(), 3);
        }
        {
            let mut list = LinkedList::<i32>::from([1, 2, 3]);
            list.clear();
            assert_eq!(list.len(), 0)
        }

        {
            let list = LinkedList::<i32>::from([1, 2, 3]);
            assert_eq!(list.contains(&1), true);
        }
    }

    #[test]
    fn test_hash_set() {
        {
            let mut set = HashSet::new();
            assert!(set.insert(1));
            assert!(set.insert(2));
            assert!(set.contains(&1));
            assert!(set.contains(&2));
            assert!(!set.contains(&3));
        }
        {
            let mut set = HashSet::new();
            assert!(set.insert(10)); // 第一次插入成功
            assert!(!set.insert(10)); // 第二次插入返回 false
            assert_eq!(set.len(), 1);
        }
        {
            let mut set = HashSet::new();
            set.insert(100);
            assert!(set.remove(&100));
            assert!(!set.contains(&100));
            assert!(!set.remove(&100)); // 再次移除失败
        }
        {
            let mut set = HashSet::new();
            set.insert(1);
            set.insert(2);
            set.clear();
            assert!(set.is_empty());
        }

        {
            let set = HashSet::from([1, 2, 3]);
            assert_eq!(set.len(), 3);
            assert!(set.contains(&1));
            assert!(set.contains(&2));
            assert!(set.contains(&3));
        }
    }

    #[test]
    fn test_concurrent_set() {
        pub fn create_set() -> ConcurrentSet<i32> {
            let set = ConcurrentSet::new();
            set.insert(1);
            set.insert(2);
            set.insert(3);
            set
        }
        {
            let set = ConcurrentSet::new();
            assert!(set.insert(10));
            assert!(set.contains(&10));
            assert!(!set.insert(10)); // 重复插入
            assert_eq!(set.len(), 1);
            assert_eq!(set.remove(&10).unwrap(), 10);
            assert!(!set.contains(&10));
        }
        {
            let set = create_set();
            let expected: HashSet<_> = [1, 2, 3].iter().cloned().collect();
            let actual: HashSet<_> = set.iter().map(|v| *v).collect();
            assert_eq!(expected, actual);
        }
        {
            let set = Arc::new(ConcurrentSet::new());
            let mut handles = vec![];

            for i in 0..10 {
                let set = Arc::clone(&set);
                handles.push(thread::spawn(move || {
                    set.insert(i);
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            for i in 0..10 {
                assert!(set.contains(&i));
            }

            assert_eq!(set.len(), 10);
        }
        {
            let set = create_set();
            assert!(!set.is_empty());
            set.clear();
            assert!(set.is_empty());
        }
    }

    mod hash_map {
        use rovkit::collectionkit::*;

        pub fn create_map() -> HashMap<String, i32> {
            let mut map = HashMap::new();
            map.insert("apple".to_string(), 3);
            map.insert("banana".to_string(), 5);
            map.insert("orange".to_string(), 2);
            map
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_insert_and_get() {
                let mut map = HashMap::new();
                map.insert("key1", 100);
                map.insert("key2", 200);

                assert_eq!(map.get("key1"), Some(&100));
                assert_eq!(map.get("key2"), Some(&200));
                assert_eq!(map.get("key3"), None);
            }

            #[test]
            fn test_update_value() {
                let mut map = HashMap::new();
                map.insert("count", 1);
                assert_eq!(map.get("count"), Some(&1));

                map.insert("count", 42); // 更新值
                assert_eq!(map.get("count"), Some(&42));
            }

            #[test]
            fn test_remove_key() {
                let mut map = HashMap::new();
                map.insert("temp", 999);
                assert_eq!(map.remove("temp"), Some(999));
                assert_eq!(map.get("temp"), None);
            }

            #[test]
            fn test_clear_map() {
                let mut map = create_map();
                assert_eq!(map.len(), 3);
                map.clear();
                assert!(map.is_empty());
            }

            #[test]
            fn test_iterate_keys_and_values() {
                let map = create_map();
                let mut keys: Vec<&str> = vec![];
                let mut values: Vec<i32> = vec![];

                for (k, v) in &map {
                    keys.push(k);
                    values.push(*v);
                }

                assert!(keys.contains(&"apple"));
                assert!(values.contains(&3));
            }

            #[test]
            fn test_compare_with_expected() {
                let map = create_map();
                let mut expected = HashMap::new();
                expected.insert("apple".to_string(), 3);
                expected.insert("banana".to_string(), 5);
                expected.insert("orange".to_string(), 2);

                assert_eq!(map, expected);
            }
        }
    }

    mod linked_hash_map {
        use rovkit::collectionkit::*;

        pub fn create_map() -> LinkedHashMap<String, i32> {
            let mut map = LinkedHashMap::new();
            map.insert("apple".to_string(), 10);
            map.insert("banana".to_string(), 20);
            map.insert("orange".to_string(), 15);
            map
        }

        #[test]
        fn test_insert_and_get() {
            let mut map = LinkedHashMap::new();
            map.insert("key1", 100);
            map.insert("key2", 200);

            assert_eq!(map.get("key1"), Some(&100));
            assert_eq!(map.get("key2"), Some(&200));
            assert_eq!(map.get("key3"), None);
        }

        #[test]
        fn test_update_value() {
            let mut map = LinkedHashMap::new();
            map.insert("count", 1);
            assert_eq!(map.get("count"), Some(&1));

            map.insert("count", 42);
            assert_eq!(map.get("count"), Some(&42));
        }

        #[test]
        fn test_remove_key() {
            let mut map = LinkedHashMap::new();
            map.insert("temp", 999);
            assert_eq!(map.remove("temp"), Some(999));
            assert_eq!(map.get("temp"), None);
        }

        #[test]
        fn test_clear_map() {
            let mut map = create_map();
            assert_eq!(map.len(), 3);
            map.clear();
            assert!(map.is_empty());
        }

        #[test]
        fn test_iterate_sorted_keys() {
            let map = create_map();
            let keys: Vec<&String> = map.keys().collect();
            assert_eq!(
                keys,
                vec![
                    &"apple".to_string(),
                    &"banana".to_string(),
                    &"orange".to_string()
                ]
            );
        }

        #[test]
        fn test_compare_with_expected() {
            let map = create_map();

            let mut expected = LinkedHashMap::new();
            expected.insert("apple".to_string(), 10);
            expected.insert("banana".to_string(), 20);
            expected.insert("orange".to_string(), 15);

            assert_eq!(map, expected);
        }
    }

    mod concurrent_map {
        use rovkit::collectionkit::*;
        use std::sync::Arc;
        use std::thread;

        pub fn create_map() -> ConcurrentMap<String, i32> {
            let map = ConcurrentMap::new();
            map.insert("apple".into(), 1);
            map.insert("banana".into(), 2);
            map.insert("orange".into(), 3);
            map
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_basic_operations() {
                let map = ConcurrentMap::new();
                map.insert("key1", 10);
                assert_eq!(*map.get("key1").unwrap(), 10);
                assert!(map.contains_key("key1"));

                map.insert("key1", 20); // 更新
                assert_eq!(*map.get("key1").unwrap(), 20);

                assert_eq!(map.remove("key1"), Some(("key1", 20)));
                assert!(!map.contains_key("key1"));
            }

            #[test]
            fn test_clear_and_is_empty() {
                let map = create_map();
                assert!(!map.is_empty());
                map.clear();
                assert!(map.is_empty());
            }

            #[test]
            fn test_iter_and_compare() {
                let map = create_map();
                let mut expected = HashMap::new();
                expected.insert("apple".to_string(), 1);
                expected.insert("banana".to_string(), 2);
                expected.insert("orange".to_string(), 3);

                let actual: HashMap<_, _> = map
                    .iter()
                    .map(|kv| (kv.key().clone(), *kv.value()))
                    .collect();
                assert_eq!(expected, actual);
            }

            #[test]
            fn test_concurrent_access() {
                let map = Arc::new(ConcurrentMap::new());
                let mut handles = vec![];

                for i in 0..10 {
                    let map = Arc::clone(&map);
                    handles.push(thread::spawn(move || {
                        map.insert(format!("key{}", i), i);
                    }));
                }

                for handle in handles {
                    handle.join().unwrap();
                }

                for i in 0..10 {
                    let key = format!("key{}", i);
                    assert_eq!(*map.get(&key).unwrap(), i);
                }

                assert_eq!(map.len(), 10);
            }
        }
    }

    mod queue {
        use rovkit::collectionkit::*;

        pub fn create_deque() -> Queue<i32> {
            let mut q = Queue::new();
            q.push_back(1);
            q.push_back(2);
            q.push_back(3);
            q
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use rovkit::collectionkit::Queue;

            #[test]
            fn test_push_and_pop() {
                let mut q = Queue::new();
                q.push_back(10);
                q.push_front(5);
                assert_eq!(q.front(), Some(&5));
                assert_eq!(q.back(), Some(&10));

                assert_eq!(q.pop_front(), Some(5));
                assert_eq!(q.pop_back(), Some(10));
                assert!(q.is_empty());
            }

            #[test]
            fn test_index_access() {
                let q = create_deque(); // [1, 2, 3]
                assert_eq!(q[0], 1);
                assert_eq!(q[1], 2);
                assert_eq!(q[2], 3);
            }

            #[test]
            fn test_iterate_elements() {
                let q = create_deque();
                let collected: Vec<_> = q.iter().cloned().collect();
                assert_eq!(collected, vec![1, 2, 3]);
            }

            #[test]
            fn test_len_and_clear() {
                let mut q = create_deque();
                assert_eq!(q.len(), 3);
                q.clear();
                assert!(q.is_empty());
            }

            #[test]
            fn test_extend_and_contains() {
                let mut q = Queue::from(vec![1, 2]);
                q.extend([3, 4]);
                assert_eq!(q.contains(&3), true);
                assert_eq!(q.contains(&5), false);
                assert_eq!(q.len(), 4);
            }
        }
    }
}
