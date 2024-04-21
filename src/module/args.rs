use std::env::args;
use std::fs;

pub fn env_args() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];
    // Reading a File
    // fs crate to read the file
    // read_to_string to read the content of the file
    let content = fs::read_to_string(file_path).expect("Failed to read the file");
    println!("{}", content);
}
