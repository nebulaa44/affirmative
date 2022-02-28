use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let repeat_this = string_to_repeat(&args);
    loop {
        println!("{repeat_this}");
    }
}

fn string_to_repeat(args: &Vec<String>) -> String {
    // The first argument is always the path of the binary being run.
    match args.get(1) {
        None      => String::from("y"),
        Some(arg) => arg.to_owned()
    }
}