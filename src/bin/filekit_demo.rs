use rovkit::filekit;

fn main() {
    let path = "test.txt";

    filekit::write_string(path, "Hello, RovKit!").unwrap();

    let content = filekit::read_to_string(path).unwrap();
    println!("内容: {}", content);

    println!("是否存在: {}", filekit::exists(path));
    println!("文件大小: {} bytes", filekit::file_size(path).unwrap());

    filekit::remove_file(path).unwrap();
}
