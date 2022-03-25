
use crate::store::store_cert;

pub fn main(out: String) {

    pair(out);
}

fn pair(out: String) {
    println!("Getting your certificate, wait a minute...");
    let _ = store_cert::get_cert(out);
}
