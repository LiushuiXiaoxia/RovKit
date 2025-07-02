#[cfg(test)]
mod tests {

    use rovkit::singlekit::*;
    use rovkit::*;
    #[test]
    fn test_single() {
        #[derive(Debug)]
        struct Config {
            pub name: String,
        }
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

    #[test]
    fn test_single2() {
        #[derive(Debug)]
        struct Config {
            pub name: String,
        }

        singleton!(pub CONFIG_INSTANCE:Config = Config {
            name: "test".to_string(),
        });
        {
            let mut config = Config::single().lock().unwrap();
            println!("{:?}", config);
            config.name = "test2".to_string();
            println!("{:?}", config);
        }

        {
            let mut c = CONFIG_INSTANCE.lock().unwrap();
            c.name = "test3".to_string();
            print!("config: {:?}", c);
        }
    }
}
