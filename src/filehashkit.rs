use md5::Context;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// 计算文件的 MD5 哈希，返回16进制字符串
pub fn hash_md5<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut context = Context::new();
    let mut buffer = [0u8; 8192];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.write(&buffer[..count])?;
    }
    Ok(format!("{:x}", context.finalize()))
}

/// 计算文件的 SHA256 哈希，返回16进制字符串
pub fn hash_sha256<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut context = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(hex::encode(context.finalize()))
}

/// 计算文件的 SHA512 哈希，返回16进制字符串
pub fn hash_sha512<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut context: Sha512 = Sha512::new();
    let mut buffer = [0u8; 8192];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(hex::encode(context.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_hashes() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, b"hello world").unwrap();

        let md5 = hash_md5(&file_path).unwrap();
        assert_eq!(md5, "5eb63bbbe01eeed093cb22bb8f5acdc3");

        let sha256 = hash_sha256(&file_path).unwrap();
        assert_eq!(
            sha256,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9" // 这里只写部分，测试时请用完整值
        );

        let sha512 = hash_sha512(&file_path).unwrap();
        assert!(sha512.starts_with("309ecc489c12d6eb4cc40f50c902f2b4"));
    }
}
