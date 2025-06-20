use rovkit::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct AppConfig {
    port: u16,
    name: String,
}

fn main() {
    let f = "build/config.json";
    filekit::write_string(f, "{\"port\":8080,\"name\":\"Rust\"}").expect("write config");

    let config: AppConfig = configkit::from_json_file(f).expect("load config");
    println!("loaded config: {:?}", &config);
    println!(
        "json config: {}",
        jsonkit::to_pretty_json(&config).unwrap()
    );
}
