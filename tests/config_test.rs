#[cfg(test)]
mod tests {
    use rovkit::configkit::from_json_file;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[derive(Debug, Deserialize, PartialEq)]
    struct AppConfig {
        name: String,
        debug: bool,
    }

    #[test]
    fn test_load_json_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{{\"name\":\"test\",\"debug\":true}}").unwrap();

        let config: AppConfig = from_json_file(&file_path).unwrap();
        assert_eq!(config.name, "test");
        assert!(config.debug);
    }
}

// 示例：
/*

*/
