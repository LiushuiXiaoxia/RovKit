#[cfg(test)]
mod tests {
    use rovkit::filehashkit::*;
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
