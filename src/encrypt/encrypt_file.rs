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

use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use std::env;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    key: [u8; 32], 
    nonce: [u8; 24],
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


    let path = env::current_dir()?;
    let config_path = format!("{}/src/store/data/{}/config",path.display(), name);
    if std::path::Path::new(&config_path).exists() {
        let mut f = BufReader::new(File::open(config_path.clone()).unwrap());
        let m: Config = bincode::deserialize_from(&mut f).unwrap();

        key = m.key;
        nonce = m.nonce;
    }

    let key_chacha = Key::from_slice(&key);
    let cipher = XChaCha20Poly1305::new(key_chacha);

    let file_data = fs::read(filepath)?;

    let nonce_chacha = XNonce::from_slice(&nonce);
    let encrypted_file = cipher
        .encrypt(nonce_chacha, file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    let m = Config{ key: key, nonce: nonce};
    let curr_path = format!("{}/src/store/data/{}",path.display() , name);
    fs::create_dir_all(curr_path)?;

    let mut f = BufWriter::new(File::create(config_path.clone()).unwrap());
    bincode::serialize_into(&mut f, &m).unwrap();

    Ok(())
}

pub fn decrypt_small_file(
    encrypted_file_path: String,
    dist: String,
    name: String,
) -> Result<(), anyhow::Error> {

    let path = env::current_dir()?;
    let config_path = format!("{}/src/store/data/{}/config",path.display(), name);

    let mut f = BufReader::new(File::open(config_path).unwrap());
    let m: Config = bincode::deserialize_from(&mut f).unwrap();

    let cipher = XChaCha20Poly1305::new(m.key.as_slice().into());
    
    let file_data = fs::read(encrypted_file_path)?;

    let decrypted_file = cipher
        .decrypt(m.nonce.as_slice().into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}

