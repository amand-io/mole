extern crate getopts;
mod certificates;
mod store;
use getopts::Options;
use std::env;
use certificates::self_cert as self_cert;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] NAME", program);
    print!("{}", opts.usage(&brief));
}

fn print_self(out: Option<String>) {
        match out {
            Some(x) => { println!("Generating a self-signed certificate and linking with name: {} ...", x);
                       self_cert::gen_self_cert(Some(x)); 
            } ,
            None => println!("Generating a self-signed certificate  with no name linked..."),
        }

}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n", "name", "name of your certificate", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s", "self", "create a self-signed certificate");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    let output = matches.opt_str("n");

    if matches.opt_present("s") {
        print_self(output);
    }


}