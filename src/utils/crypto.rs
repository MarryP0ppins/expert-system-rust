use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Error, Key,
};

pub fn encrypt_data(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, plaintext);
    ciphertext
}

pub fn decrypt_data(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.decrypt(&nonce, ciphertext);
    ciphertext
}
