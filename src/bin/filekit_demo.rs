fn main() {}

#[cfg(test)]
mod tests {
    use rovkit::filekit;

    #[test]
    fn test_filekit() {
        let path = "build/test.txt";
        let parent = filekit::parent_dir(path).unwrap();
        if !filekit::exists(&parent) {
            filekit::create_dir_all(&parent).unwrap();
        }

        filekit::write_string(path, "Hello, RovKit!").unwrap();

        let content = filekit::read_string(path).unwrap();
        println!("内容: {}", content);

        println!("是否存在: {}", filekit::exists(path));
        println!("文件大小: {} bytes", filekit::file_size(path).unwrap());

        filekit::remove_file(path).unwrap();
    }

    #[test]
    fn test_copy_file() -> Result<(), Box<dyn std::error::Error>> {
        let f1 = "build/test.txt";
        let f2 = "build/test2.txt";

        filekit::write_data(f1, b"hello world")?;
        filekit::copy_file(f1, f2)?;

        let data = filekit::read_data(f2)?;
        println!("data = {}", String::from_utf8_lossy(&data));

        Ok(())
    }
}
