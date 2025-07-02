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

/// 判断路径是否为软链接（符号链接）
/// 返回 Ok(true) 表示是软链，Ok(false) 不是软链，Err 出错
pub fn is_symlink<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let metadata = fs::symlink_metadata(path)?;
    Ok(metadata.file_type().is_symlink())
}

#[cfg(unix)]
use std::os::unix::fs::symlink as symlink_unix;

#[cfg(windows)]
use std::os::windows::fs::{
    symlink_dir as symlink_dir_windows, symlink_file as symlink_file_windows,
};

/// 创建软链接，支持文件和目录，跨平台
pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();

    if original.is_dir() {
        #[cfg(unix)]
        {
            symlink_unix(original, link)
        }
        #[cfg(windows)]
        {
            symlink_dir_windows(original, link)
        }
    } else {
        #[cfg(unix)]
        {
            symlink_unix(original, link)
        }
        #[cfg(windows)]
        {
            symlink_file_windows(original, link)
        }
    }
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
    if is_file(path) {
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
