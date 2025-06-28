use std::sync::{Mutex, OnceLock};

/// 通用单例包装器
pub struct Single<T> {
    instance: OnceLock<Mutex<T>>,
}

impl<T> Single<T> {
    /// 创建新的单例容器
    pub const fn new() -> Self {
        Self {
            instance: OnceLock::new(),
        }
    }

    /// 获取单例引用，如果未初始化则使用提供的闭包初始化
    pub fn get_or_init<F>(&'static self, init: F) -> &'static Mutex<T>
    where
        F: FnOnce() -> T,
    {
        self.instance.get_or_init(|| Mutex::new(init()))
    }

    /// 获取单例引用，必须已经初始化
    pub fn get(&'static self) -> Option<&'static Mutex<T>> {
        self.instance.get()
    }
}

// 为所有T实现Default
impl<T> Default for Single<T> {
    fn default() -> Self {
        Self::new()
    }
}

// 线程安全标记
unsafe impl<T> Sync for Single<T> where T: Sync {}
/// 通用单例包装器
/// ```
///  singleton!(pub CONFIG_INSTANCE:Config = Config {
///     name: "test".to_string(),
///  });
///
///
#[macro_export]
macro_rules! singleton {
    ($vis:vis $name:ident: $ty:ty = $expr:expr) => {
        use once_cell::sync::Lazy;
        use std::sync::Mutex;

        $vis static $name: Lazy<Mutex<$ty>> = Lazy::new(|| {
            Mutex::new($expr)
        });

        impl $ty {
            $vis fn single() -> &'static Mutex<Self> {
                &$name
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Config {
        pub name: String,
    }
    #[test]
    fn test_single() {
        use crate::singlekit::Single;
        static CONFIG: Single<Config> = Single::new();
        CONFIG.get_or_init(|| Config {
            name: "Single-config".to_string(),
        });
        {
            let mut c = CONFIG.get().unwrap().lock().unwrap();
            println!("config name = {}", c.name);
            c.name = "update".to_string();
            println!("config name = {}", c.name);
        }
        {
            let mut c = CONFIG.get().unwrap().lock().unwrap();
            println!("config name = {}", c.name);
            c.name = "new Name ".to_string();
            println!("config name = {}", c.name);
        }
    }

    singleton!(pub CONFIG_INSTANCE:Config = Config {
        name: "test".to_string(),
    });
    #[test]
    fn test_single2() {
        let mut config = Config::single().lock().unwrap();
        println!("{:?}", config);
        config.name = "test2".to_string();
        println!("{:?}", config);

        let mut c = CONFIG_INSTANCE.lock().unwrap();
        c.name = "test3".to_string();
        print!("config: {:?}", c);
    }
}
