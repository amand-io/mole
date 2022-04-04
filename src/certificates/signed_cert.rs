extern crate openssl;

use std::fs;
use std::env;
use openssl::x509::{X509, X509Name};
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use openssl::rsa::Rsa;
use openssl::nid::Nid;
use std::fs::OpenOptions;
use std::io::Write;
use openssl::pkey::Private;
use crate::store::store_cert;

pub fn gen_cert(ca_name: String, out_name: String) -> std::io::Result<()> {
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    // CA Zone
    let path = env::current_dir()?;

    let curr_path = format!("{}/src/store/data/{}",path.display() , ca_name);
    if std::path::Path::new(&curr_path).exists() {
        fs::create_dir_all(curr_path)?;
        println!("Catching file from {}/src/store/data/{}/",path.display(), ca_name);
    } else {  return Ok(()); }

    let key_path = format!("{}/src/store/data/{}/cert.key",path.display(), ca_name);
    let contents_k = fs::read_to_string(key_path)
        .expect("Something went wrong reading the file");

    println!("With cert:\n{}", contents_k);
    let ca_rsa = Rsa::private_key_from_pem(contents_k.as_bytes()).unwrap();
    let ca_pkey = PKey::from_rsa(ca_rsa).unwrap();

    // Build Certificate
    let mut name = X509Name::builder().unwrap();
    name.append_entry_by_nid(Nid::COMMONNAME, "foobar.com").unwrap();
    let name = name.build();

    let mut builder = X509::builder().unwrap();
    builder.set_version(2).unwrap();
    builder.set_subject_name(&name).unwrap();
    builder.set_issuer_name(&name).unwrap();
    builder.set_pubkey(&pkey).unwrap();

    builder.sign(&ca_pkey, MessageDigest::sha256()).unwrap();

    let certificate: X509 = builder.build();
    let _ = save_cert(out_name, certificate, pkey);
    Ok(())
}

fn save_cert(name: String, cert: X509, key: PKey<Private>) -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let curr_path = format!("{}/src/store/data/{}",path.display() , name);
    if ! std::path::Path::new(&curr_path).exists() {
        fs::create_dir_all(curr_path)?;
        println!("Saving file created on {}/src/store/data/{}/",path.display(), name);
    } else if !store_cert::verify() {  return Ok(()); }

    let cert_path = format!("{}/src/store/data/{}/cert.pem",path.display(), name);
    let mut cert_file = OpenOptions::new()
        .read(true)
        .write(true) 
        .create(true)
        .truncate(true)
        .open(&cert_path)
        .unwrap();
    
    let cert_arq = cert.to_pem().unwrap();
    let _ = cert_file.write(&cert_arq)?;
    //println!("{}", cert_arq);

    let key_path = format!("{}/src/store/data/{}/cert.key",path.display(), name);
    let mut key_file = OpenOptions::new()
        .read(true)
        .write(true) 
        .create(true)
        .truncate(true)
        .open(&key_path)
        .unwrap();

    let key_arq = key.private_key_to_pem_pkcs8().unwrap();
    let _ = key_file.write(&key_arq)?;

    //println!("{}", key_arq);
    Ok(())
}