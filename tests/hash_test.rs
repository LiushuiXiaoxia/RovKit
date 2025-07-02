#[cfg(test)]
mod tests {
    use rovkit::hashkit::*;

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
