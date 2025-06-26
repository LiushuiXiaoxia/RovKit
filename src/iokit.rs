use rand::{distributions::Alphanumeric, Rng};
use std::env::temp_dir;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// 读取文件所有内容，返回字符串
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// 读取文件所有内容，返回字节数组
pub fn read_file_to_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// 写入字符串到文件（覆盖）
pub fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// 写入字节数组到文件（覆盖）
pub fn write_bytes_to_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    fs::write(path, data)
}

/// 追加字符串写入文件
pub fn append_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(content.as_bytes())
}

/// 复制文件
pub fn copy_file<P: AsRef<Path>>(src: P, dst: P) -> io::Result<u64> {
    fs::copy(src, dst)
}

/// 判断路径是否存在（文件或目录）
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// 判断是否是文件
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// 判断是否是目录
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// 创建目录（包括所有父目录）
/// 返回 Ok(()) 或 io::Error
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// 删除文件或目录（目录递归删除）
/// 返回 Ok(()) 或 io::Error
pub fn remove_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(());
    }
    if path.is_file() {
        fs::remove_file(path)
    } else {
        fs::remove_dir_all(path)
    }
}

/// 获取文件大小，单位字节
pub fn get_file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// 创建临时文件，文件关闭后不自动删除，返回路径和文件句柄
pub fn create_temp_file() -> io::Result<(PathBuf, File)> {
    let mut rng = rand::thread_rng();
    let filename: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(12)
        .map(char::from)
        .collect();

    let mut path = temp_dir();
    path.push(filename);

    // 以读写方式创建文件，如果文件存在会返回错误
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)?;

    Ok((path, file))
}

/// 创建临时目录，返回 TempDir
/// TempDir 关闭时自动删除目录
pub fn create_temp_dir() -> io::Result<TempDir> {
    TempDir::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        write_string_to_file(&file_path, "hello world").unwrap();
        assert_eq!(read_file_to_string(&file_path).unwrap(), "hello world");

        append_string_to_file(&file_path, "!!!").unwrap();
        assert_eq!(read_file_to_string(&file_path).unwrap(), "hello world!!!");

        let bytes = read_file_to_bytes(&file_path).unwrap();
        assert_eq!(bytes, b"hello world!!!");
    }

    #[test]
    fn test_copy_and_remove() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        let dst = dir.path().join("dst.txt");

        write_string_to_file(&src, "copy me").unwrap();
        copy_file(&src, &dst).unwrap();
        assert!(path_exists(&dst));
        assert_eq!(read_file_to_string(&dst).unwrap(), "copy me");

        remove_all(&src).unwrap();
        assert!(!path_exists(&src));

        remove_all(&dst).unwrap();
        assert!(!path_exists(&dst));
    }

    #[test]
    fn test_create_dir_all() {
        let dir = tempdir().unwrap();
        println!("test_create_dir_all: dir = {:?}", dir);
        let nested = dir.path().join("a/b/c");
        create_dir_all(&nested).unwrap();
        assert!(is_dir(&nested));
    }

    #[test]
    fn test_get_file_size() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("size.txt");
        write_string_to_file(&file, "12345").unwrap();
        assert_eq!(get_file_size(&file).unwrap(), 5);
    }

    #[test]
    fn test_temp_file_and_dir() {
        let (path, file) = create_temp_file().unwrap();
        assert!(path.exists());
        drop(file); // 关闭文件，但文件仍存在
        assert!(path.exists());

        let temp_dir = create_temp_dir().unwrap();
        assert!(temp_dir.path().exists());
        // TempDir drop时自动删除
    }
}
