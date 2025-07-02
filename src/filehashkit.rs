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
