mod tests {
    use rovkit::cryptokit::*;

    #[test]
    fn test_aes() {
        println!("=== AES 加解密示例 ===");

        // 生成 AES 密钥和 IV
        let (aes_key, aes_iv) = aeskit::generate_aes_key_iv();
        println!("AES Key: {}", hex::encode(&aes_key));
        println!("AES IV: {}", hex::encode(&aes_iv));

        let plaintext = "Hello, AES encryption!";
        println!("原始文本: {}", plaintext);

        // AES 加密
        let encrypted = aeskit::aes_encrypt(plaintext.as_bytes(), &aes_key, &aes_iv);
        println!("加密结果: {}", hex::encode(&encrypted));

        // AES 解密
        let decrypted = aeskit::aes_decrypt(&encrypted, &aes_key, &aes_iv);
        println!("解密结果: {}", String::from_utf8(decrypted).unwrap());
    }

    #[test]
    fn test_rsa() {
        println!("=== RSA 加解密示例 ===");

        // 生成 RSA 密钥对
        let (mut rng, public_key, private_key) = rsakit::generate_rsa_keys();
        println!("RSA rng:\n{:?}", rng);
        println!("RSA 公钥:\n{:?}", public_key);
        println!("RSA 私钥:\n{:?}", private_key);
        //
        let plaintext = "Hello, RSA encryption!";
        println!("原始文本: {}", plaintext);

        // RSA 加密
        let encrypted = rsakit::rsa_encrypt(&mut rng, &public_key, plaintext.as_bytes());
        println!("加密结果: {}", hex::encode(&encrypted));

        // RSA 解密
        let decrypted = rsakit::rsa_decrypt(&private_key, &encrypted);
        let s2 = String::from_utf8(decrypted).unwrap();
        println!("解密结果: {}", &s2);

        assert_eq!(plaintext, s2);
    }
}
