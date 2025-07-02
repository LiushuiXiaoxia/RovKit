pub mod aeskit {
    use aes::Aes256;
    use cbc::{Decryptor, Encryptor};
    use cipher::block_padding::Pkcs7;
    use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
    use generic_array::GenericArray;
    use rand::Rng;

    type Aes256CbcEnc = Encryptor<Aes256>;
    type Aes256CbcDec = Decryptor<Aes256>;

    // AES 加密
    pub fn aes_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);

        let mut buf = [0u8; 48];
        let pt_len = plaintext.len();
        buf[..pt_len].copy_from_slice(&plaintext);

        let ct = Aes256CbcEnc::new(key, iv)
            .encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut buf)
            .unwrap();

        ct.to_vec()
    }

    // AES 解密
    pub fn aes_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut buf = [0u8; 48];
        let pt = Aes256CbcDec::new(key, iv)
            .decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf)
            .unwrap();

        pt.to_vec()
    }

    // 生成随机密钥和 IV
    pub fn generate_aes_key_iv() -> (Vec<u8>, Vec<u8>) {
        let mut rng = rand::thread_rng();

        let key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();

        (key, iv)
    }
}

pub mod rsakit {
    use rand::prelude::ThreadRng;
    use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
    // 生成 RSA 密钥对
    pub fn generate_rsa_keys() -> (ThreadRng, RsaPublicKey, RsaPrivateKey) {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let pri_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, bits).expect("generate key fail");
        let pub_key: RsaPublicKey = RsaPublicKey::from(&pri_key);

        (rng, pub_key, pri_key)
    }

    // RSA 加密
    pub fn rsa_encrypt(rng: &mut ThreadRng, key: &RsaPublicKey, plaintext: &[u8]) -> Vec<u8> {
        key.encrypt(rng, Pkcs1v15Encrypt, &plaintext[..])
            .expect("failed to encrypt")
    }

    // RSA 解密
    pub fn rsa_decrypt(key: &RsaPrivateKey, ciphertext: &[u8]) -> Vec<u8> {
        key.decrypt(Pkcs1v15Encrypt, ciphertext).unwrap()
    }
}
