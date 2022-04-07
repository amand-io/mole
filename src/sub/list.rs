use std::fs;
use std::env;

pub fn main() {
    list_certs();
}

fn list_certs() {
    let path = env::current_dir().unwrap();
    let curr_path = format!("{}/src/store/data",path.display());
    let paths = fs::read_dir(curr_path).unwrap();

    println!("Indentifier: ");
    for path in paths {
        let curr_path = path.unwrap().path().display().to_string();
        let divide = curr_path.split("/");
        println!("::{} -", divide.last().unwrap())
    }
}