use anyhow::{Error, Result};
use chacha20poly1305::aead::AeadMut;
use chacha20poly1305::{ChaCha20Poly1305, Nonce};

pub fn encrypt(private_key: &str, key: [u8; 32], id: &uuid::Uuid) -> Result<String> {
    use chacha20poly1305::aead::NewAead;
    let nonce = Nonce::from_slice(&id.as_bytes()[0..12]);
    let key = chacha20poly1305::Key::from_slice(&key[..]);
    let mut encryptor = ChaCha20Poly1305::new(key);
    let res = encryptor
        .encrypt(nonce, base64::decode(&private_key)?.as_slice())
        .unwrap();

    Ok(base64::encode(res))
}

pub fn decrypt(private_key: &str, key: [u8; 32], id: &uuid::Uuid) -> Result<Vec<u8>> {
    use chacha20poly1305::aead::NewAead;
    let nonce = Nonce::from_slice(&id.as_bytes()[0..12]);
    let key = chacha20poly1305::Key::from_slice(&key[..]);
    let mut decrypter = ChaCha20Poly1305::new(key);
    decrypter
        .decrypt(nonce, base64::decode(private_key)?.as_slice())
        .map_err(Error::msg)
}
