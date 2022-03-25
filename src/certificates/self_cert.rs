extern crate rcgen;
use rcgen::generate_simple_self_signed;

use crate::store::store_cert;

pub fn gen_self_cert(out: String) {

    let name =  out;
    let subject_alt_names = vec!["hello.world.example".to_string(),
        "localhost".to_string()];

    let cert = generate_simple_self_signed(subject_alt_names).unwrap();
    // The certificate is now valid for localhost and the domain "hello.world.example"

    let _ = store_cert::save_cert(name, cert);
}