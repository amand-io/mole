use anyhow::anyhow;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
    Key,
    XNonce,
};
use rand::{rngs::OsRng, RngCore};
use std::{
    fs::{self },
};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, ([u8; 32], [u8; 24])>> = Mutex::new(HashMap::new());
}

pub fn encrypt_small_file(
    filepath: String,
    dist: String,
    name: String,
) -> Result<(), anyhow::Error> {

    // Generate key and nonce
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    if HASHMAP.lock().unwrap().contains_key(&name) {
        let key_and_nonce = HASHMAP.lock().unwrap().get(&name).unwrap().clone();
        key = key_and_nonce.0;
        nonce = key_and_nonce.1;
    }

    let key_chacha = Key::from_slice(&key);
    let cipher = XChaCha20Poly1305::new(key_chacha);

    let file_data = fs::read(filepath)?;

    let nonce_chacha = XNonce::from_slice(&nonce);
    let encrypted_file = cipher
        .encrypt(nonce_chacha, file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    HASHMAP.lock().unwrap().insert(
        name,
        (key, nonce),
    );

    Ok(())
}

pub fn decrypt_small_file(
    encrypted_file_path: String,
    dist: String,
    name: String,
) -> Result<(), anyhow::Error> {

    if !HASHMAP.lock().unwrap().contains_key(&name) {
        println!("The name {} not exist in this mole. :(", name);
        return Ok(())
    }
    
    let key_and_nonce = HASHMAP.lock().unwrap().get(&name).unwrap().clone();

    let cipher = XChaCha20Poly1305::new(key_and_nonce.0.as_slice().into());

    let file_data = fs::read(encrypted_file_path)?;

    let decrypted_file = cipher
        .decrypt(key_and_nonce.1.as_slice().into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}

