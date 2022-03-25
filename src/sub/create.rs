use crate::SELF;

use crate::certificates::self_cert as self_cert;

pub fn main(out: String) {
    
    if SELF.flag {
        self_cert(out);
    } else {
        panic!("Don't have this option, look to -h, --help to know more about");
    }
}
fn self_cert(out: String) {
    if !out.trim().is_empty() {
        println!("Generating a self-signed certificate and linking with name: {} ...", out);
        self_cert::gen_self_cert(out); 
    } else {
        println!("Generating a self-signed certificate  with no name linked...");
    }

}

#[warn(dead_code)]
fn normal() {}

