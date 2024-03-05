use std::io::stdin;

fn main() {
    println!("Please enter your name ");

    let mut name = String::new();

    stdin() // stdin function allow us to handle user input
    .read_line(&mut name)
    .expect("Failed to read line");

    print!("Your name is {}", name);
}
