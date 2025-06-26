// use aes_gcm::aead::Aead;
// use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
// use base64::{decode as b64decode, encode as b64encode};
// use rand::RngCore;
//
// /// AES-256-GCM 加密，返回 base64 编码的 nonce + 密文
// pub fn aes_gcm_encrypt(
//     key_bytes: &[u8; 32],
//     plaintext: &[u8],
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let key = Key::from_slice(key_bytes);
//     let cipher = Aes256Gcm::from(key);
//
//     let mut nonce_bytes = [0u8; 12];
//     rand::thread_rng().fill_bytes(&mut nonce_bytes);
//     let nonce = Nonce::from_slice(&nonce_bytes);
//
//     let ciphertext = cipher.encrypt(nonce, plaintext).unwrap();
//     let mut out = Vec::new();
//     out.extend_from_slice(&nonce_bytes);
//     out.extend_from_slice(&ciphertext);
//     Ok(b64encode(&out))
// }
//
// /// AES-256-GCM 解密，接收 base64 编码的 nonce + 密文
// pub fn aes_gcm_decrypt(
//     key_bytes: &[u8; 32],
//     b64_ciphertext: &str,
// ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let data = b64decode(b64_ciphertext)?;
//     if data.len() < 12 {
//         return Err("Ciphertext too short".into());
//     }
//     let (nonce_bytes, ciphertext) = data.split_at(12);
//     let key = Key::from_slice(key_bytes);
//     let cipher = Aes256Gcm::new(key);
//     let nonce = Nonce::from_slice(nonce_bytes);
//     let plaintext = cipher.decrypt(nonce, ciphertext)?;
//     Ok(plaintext)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_aes_gcm() {
//         let key = [0x11u8; 32];
//         let data = b"hello aes gcm!";
//         let encrypted = aes_gcm_encrypt(&key, data).unwrap();
//         let decrypted = aes_gcm_decrypt(&key, &encrypted).unwrap();
//         assert_eq!(decrypted, data);
//     }
// }
