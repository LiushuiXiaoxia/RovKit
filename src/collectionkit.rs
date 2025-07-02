///
/// 适用于 Java 体质开发同学,常见的集合对象
///
///

/// 普通的 List
pub type ArrayList<T> = Vec<T>;

/// 普通链表 List
pub type LinkedList<T> = std::collections::LinkedList<T>;

/// 线程安全的 List
pub type ConcurrentList<T> = crate::internal::concurrent_vec::ConcurrentVec<T>;

/// 普通的set
pub type HashSet<T> = std::collections::HashSet<T>;

/// 线程安全的 Set
pub type ConcurrentSet<T> = dashmap::DashSet<T>;

/// 带排序的 Set
pub type LinkedHashSet<T> = std::collections::BTreeSet<T>;

/// 普通的 HashMap
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

/// 带排序的 HashMap
pub type LinkedHashMap<K, V> = std::collections::BTreeMap<K, V>;

/// 线程安全的 HashMap
pub type ConcurrentMap<K, V> = dashmap::DashMap<K, V>;

///  队列
pub type Queue<T> = std::collections::VecDeque<T>;
