use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// 读取整个文件内容为字符串
pub fn read_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// 读取整个文件内容为字节数组
pub fn read_data<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// 将字符串内容写入文件（覆盖写入）
pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    // 确保父目录存在
    if let Some(parent) = Path::new(path.as_ref()).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

/// 将二进制数据写入文件（覆盖写入）
pub fn write_data<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    if let Some(parent) = Path::new(path.as_ref()).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(data)
}

/// 拷贝文件到目标路径
pub fn copy_file<P: AsRef<Path>>(from: P, to: P) -> io::Result<u64> {
    // 确保目标父目录存在
    if let Some(parent) = Path::new(to.as_ref()).parent() {
        fs::create_dir_all(parent)?;
    }
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

/// 判断文件或目录是否存在
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

/// 获取路径的文件名称（文件名含扩展）
pub fn file_name<P: AsRef<Path>>(path: P) -> Option<String> {
    Path::new(path.as_ref())
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.to_string())
}

/// 获取路径的父目录
pub fn parent_dir<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    Path::new(path.as_ref()).parent().map(|p| p.to_path_buf())
}

/// 创建文件所在的父目录（如果不存在）
pub fn create_parent_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if let Some(parent) = Path::new(path.as_ref()).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// 获取文件扩展名（不含点）
pub fn extension<P: AsRef<Path>>(path: P) -> Option<String> {
    Path::new(path.as_ref())
        .extension()
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.to_string())
}

/// 拼接多个路径片段
pub fn join_paths<P: AsRef<Path>>(segments: &[P]) -> PathBuf {
    let mut iter = segments.iter();
    let mut path = if let Some(first) = iter.next() {
        PathBuf::from(first.as_ref())
    } else {
        PathBuf::new()
    };
    for seg in iter {
        path.push(seg.as_ref());
    }
    path
}

/// 获取绝对路径
pub fn absolute_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    // path.as_ref().canonicalize()
    std::path::absolute(path)
}
