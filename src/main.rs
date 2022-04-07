#[macro_use]
extern crate serde_derive;
extern crate bincode;


mod certificates;
mod store;
mod sub;
mod encrypt;

extern crate gflags;
use sub::create as create;
use sub::show as show;
use sub::list as list;

gflags::define! {
    -s, --self = false
}

gflags::define! {
    -c, --ca <NAME> = "Root"
}

gflags::define! {
    -h, --help = false
}

fn main() {
    let args = gflags::parse();

    if HELP.flag {
        gflags::print_help_and_exit(0);
    }

    match args[0] {
        "create" => create::main(args[1].to_string()),
        "show" => show::main(args[1].to_string()),
        "list" => list::main(),

        // Handle the rest of cases
        _ => println!("Don't have this options"),
    }


}