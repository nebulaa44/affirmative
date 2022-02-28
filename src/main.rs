use std::env;
use std::process;

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    print_ver_if_necessary(&args);
    
    let repeat_this = string_to_repeat(&args);
    loop {
        println!("{repeat_this}");
    }
}

fn print_ver_if_necessary(args: &Vec<String>) {
    if args.contains(&String::from("--version")) {
        println!("{VERSION}");
        process::exit(0);
    }
}

fn string_to_repeat(args: &Vec<String>) -> String {
    // The first argument is always the path of the binary being run.
    match args.get(1) {
        None      => String::from("y"),
        Some(arg) => arg.to_owned()
    }
}