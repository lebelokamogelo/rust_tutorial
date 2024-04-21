use std::env::args;

pub fn env_args() {
    let args: Vec<String> = args().collect();
    let arg = &args[1];
    println!("{}", arg);
}
