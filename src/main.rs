use std::env;
use std::process;

const VERSION: &str = "0.1.0";

fn main() {
    let args_vec: Vec<String> = env::args().collect();

    print_ver_if_necessary(&args_vec);
    
    let repeat_this = string_to_repeat(&args_vec);
    loop {
        println!("{repeat_this}");
    }
}

fn print_ver_if_necessary(arg_vec: &Vec<String>) {

    if arg_vec.contains(&String::from("--version")) {
        println!("{VERSION}");
        process::exit(0);
    }
}

fn string_to_repeat(arg_vec: &Vec<String>) -> String {
    // The first argument is always the path of the binary being run.
    match arg_vec.get(1) {
        None      => String::from("y"),
        Some(arg) => arg.to_owned()
    }
}