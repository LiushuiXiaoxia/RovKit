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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        assert_eq!(md5("abc"), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(
            sha256("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha512("abc"),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a".to_string()
                + "2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
    }

    #[test]
    fn test_base64() {
        let encoded = base64_encode("hello");
        assert_eq!(encoded, "aGVsbG8=");
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, "hello");

        let ret = base64_decode("abc123");

        println!("{:?}", ret)
    }
}
