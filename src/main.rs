use std::io;
use rand::Rng;

fn main() {

    println!("Please input your guess (1-10).");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed: {guess}");

    let secret = generate_secret();
    println!("The secret number is: {secret}");
}

fn generate_secret() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    secret_number
}
