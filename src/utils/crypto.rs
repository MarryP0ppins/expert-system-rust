use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Error, Key, Nonce,
};

pub fn encrypt_data(key: &[u8], nonce_key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256GcmSiv>::from_slice(key);

    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(nonce_key); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;

    Ok(ciphertext)
}

pub fn decrypt_data(key: &[u8], nonce_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256GcmSiv>::from_slice(key);

    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(nonce_key); // 96-bits; unique per message
    let plaintext = cipher.decrypt(&nonce, ciphertext)?;

    Ok(plaintext)
}
