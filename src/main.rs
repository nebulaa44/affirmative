use std::env;
use std::process;

const VERSION: &str = "0.1.1";

fn main() {
    let args_vec: Vec<String> = env::args().collect();

    if args_vec.contains(&String::from("--version")) {
        println!("{VERSION}");
        process::exit(0);
    }
    
    let repeat_this = string_to_repeat(&args_vec);
    loop {
        println!("{repeat_this}");
    }
}

fn string_to_repeat(arg_vec: &Vec<String>) -> String {
    let mut output = String::new();
    for (i, el) in arg_vec.into_iter().enumerate() {
        // The first element is always the binary path
        if i == 0 {
            continue;
        }

        if !el.starts_with("-") {
            output.push_str(el);
            output.push(' ');
        } 
    }

    if output.is_empty() {
        output = String::from("y");
    }

    output
}