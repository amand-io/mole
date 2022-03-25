use std::fs;
use std::io::prelude::*;
use std::env;
use promptly::prompt;
use std::fs::OpenOptions;

fn verify() -> bool {
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
        println!("Saving file created on {}/src/store/data{}/",path.display(), name);
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
    Ok(())
}