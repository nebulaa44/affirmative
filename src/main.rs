use std::env;

fn main() {
    let repeat_this = string_to_repeat();
    loop {
        println!("{repeat_this}");
    }
}

fn string_to_repeat() -> String {
    let args: Vec<String> = env::args().collect();
    
    // The first argument is always the path of the binary being run.
    match args.get(1) {
        None      => String::from("y"),
        Some(arg) => arg.to_owned()
    }
}