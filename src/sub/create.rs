use crate::SELF;
use crate::CA;

use crate::certificates::self_cert as self_cert;
use crate::certificates::signed_cert as signed_cert;

pub fn main(out: String) {
    
    if SELF.flag {
        self_cert(out);
    } else if CA.is_present() {
        let ca_name = CA.flag;
        normal(ca_name.to_string(), out)
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


fn normal(ca: String, name: String) {
    if !name.trim().is_empty() {
        println!("Generating a signed certificate and linking with name: {} ...", name);
        let _ = signed_cert::gen_cert(ca, name); 
    } else {
        println!("Generating a signed certificate with no name linked...");
    }
}

