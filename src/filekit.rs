use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// 读取整个文件内容为字符串
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// 将字符串内容写入文件（覆盖写入）
pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// 拷贝文件到目标路径
pub fn copy_file<P: AsRef<Path>>(from: P, to: P) -> io::Result<u64> {
    fs::copy(from, to)
}

/// 删除指定文件
pub fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path)
}

/// 创建文件夹（可递归创建）
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// 判断文件是否存在
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

/// 获取文件大小（以字节为单位）
pub fn file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

/// 列出目录下所有文件（非递归）
pub fn list_files<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            files.push(entry.path());
        }
    }
    Ok(files)
}

/// 递归列出所有文件
pub fn list_files_recursive<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    visit_dirs(dir.as_ref(), &mut files)?;
    Ok(files)
}

fn visit_dirs(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else {
                files.push(path);
            }
        }
    }
    Ok(())
}
