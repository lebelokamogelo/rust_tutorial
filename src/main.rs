/*
use std::io;
use rand::Rng;

fn main() {

    println!("Please input your guess (1-10).");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let secret = generate_secret();
    let guess: i32 = guess.trim().parse().expect("Failed to convert");

    match guess {

        // values used in match arms must be known at compile time.
        // If a value isn't known at compile time, it can't be used directly in a match arm.
        // Instead, you'll need to bind the value to a variable using a pattern.

        x if x == secret => println!("You won"),
        _ => println!("You lost, try again")
    }
}

fn generate_secret() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    secret_number
}
*/

// Managing Growing Projects with Packages, Crates, and Modules

/*
   Start from the crate root: Compiler begins in the crate root file.
   Declare modules: Use module keyword in root file or other files.
   Declare submodules: Also using module keyword in non-root files.
   Paths to code: Referencing module code within the crate.
   Public vs. private: Control visibility with pub keyword.
   Use keyword: Create shortcuts within scopes for concise code.
*/

mod module;

// use module::fibonacci::fibonacci;
// use module::collections::hashmap;

// use module::error::errors;
// use module::generics::generic;

// use module::traits::{notify, Summary, Tweet};

//use module::lifetime::lifetime;

fn main() {
    // fibonacci();
    /*
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    */

    module::refcell::refcell();

    module::console::print();
}
