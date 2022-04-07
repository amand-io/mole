use std::fs;
use std::io::prelude::*;
use std::env;
use promptly::prompt;
use std::fs::OpenOptions;
use crate::encrypt::encrypt_file;

pub fn verify() -> bool {
    loop {
        let answer: String = prompt("The Name has been used :( Do you want subscribe the files? [Y/ N]");
    
        if answer.to_uppercase().eq("Y") {
            return true;
        } else if answer.to_uppercase().eq("N"){
            return false;
        } 
    }
}

pub fn save_cert(name: String, cert: rcgen::Certificate) -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let curr_path = format!("{}/src/store/data/{}",path.display() , name);
    if ! std::path::Path::new(&curr_path).exists() {
        fs::create_dir_all(curr_path)?;
        println!("Saving file created on {}/src/store/data/{}/",path.display(), name);
    } else if !verify() {  return Ok(()); }

    let cert_path = format!("{}/src/store/data/{}/cert.pem",path.display(), name);
    let mut cert_file = OpenOptions::new()
        .read(true)
        .write(true) 
        .create(true)
        .truncate(true)
        .open(&cert_path)
        .unwrap();
    
    let cert_arq = cert.serialize_pem().unwrap();
    let _ = cert_file.write(cert_arq.as_bytes())?;
    println!("{}", cert_arq);

    let key_path = format!("{}/src/store/data/{}/cert.key",path.display(), name);
    let mut key_file = OpenOptions::new()
        .read(true)
        .write(true) 
        .create(true)
        .truncate(true)
        .open(&key_path)
        .unwrap();

    let key_arq = cert.serialize_private_key_pem();
    let _ = key_file.write(key_arq.as_bytes())?;

    println!("{}", key_arq);

    let _ = encrypt_file::encrypt_small_file(cert_path.clone(), cert_path.clone(), name.clone());
    let _ = encrypt_file::encrypt_small_file(key_path.clone(), key_path.clone(), format!("{}/config_key", name));
    Ok(())
}

pub fn get_cert(name: String) -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let curr_path = format!("{}/src/store/data/{}",path.display() , name);
    if std::path::Path::new(&curr_path).exists() {
        fs::create_dir_all(curr_path)?;
        println!("Catching file from {}/src/store/data/{}/",path.display(), name);
    } else {  return Ok(()); }

    let cert_path = format!("{}/src/store/data/{}/cert.pem",path.display(), name);
    let _ = encrypt_file::decrypt_small_file(cert_path.clone(), cert_path.clone(), name.clone());
    let contents_c = fs::read_to_string(cert_path.clone())
        .expect("Something went wrong reading the file");

    println!("With cert:\n{}", contents_c);

    let key_path = format!("{}/src/store/data/{}/cert.key",path.display(), name.clone());
    let _ = encrypt_file::decrypt_small_file(key_path.clone(), key_path.clone(), format!("{}/config_key", name));
    let contents_k = fs::read_to_string(key_path.clone())
        .expect("Something went wrong reading the file");

    println!("With cert:\n{}", contents_k);

    // Encrypt file again
    let _ = encrypt_file::encrypt_small_file(cert_path.clone(), cert_path.clone(), name.clone());
    let _ = encrypt_file::encrypt_small_file(key_path.clone(), key_path.clone(), format!("{}/config_key", name));
    Ok(())
}