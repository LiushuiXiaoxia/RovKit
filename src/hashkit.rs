use base64::Engine;
use sha2::{Digest, Sha256, Sha512};

/// 计算 SHA-256 哈希
pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

/// 计算 SHA-512 哈希
pub fn sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

/// 计算 MD5 哈希
pub fn md5(input: &str) -> String {
    let hasher = md5::compute(input.as_bytes());
    format!("{:x}", hasher)
}

/// Base64 编码
pub fn base64_encode(input: &str) -> String {
    base64::prelude::BASE64_STANDARD.encode(input.as_bytes())
}

/// Base64 解码
pub fn base64_decode(input: &str) -> Result<String, base64::DecodeError> {
    let ret = base64::prelude::BASE64_STANDARD.decode(input.as_bytes());

    match ret {
        Ok(v) => Ok(String::from_utf8(v).unwrap()),
        Err(e) => Err(e),
    }
}
