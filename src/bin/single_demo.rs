use rovkit::singleton;

#[allow(dead_code)]
fn main() {
    let mut config = CONFIG_INSTANCE.lock().unwrap();
    println!("config: {:?}", config);
    config.name = "test".to_string();
    println!("config: {:?}", config);
}

#[derive(Debug)]
struct Config {
    name: String,
}

// 一行生成全套接口
singleton!(pub CONFIG_INSTANCE:Config = Config {
    name: "test".to_string(),
});
