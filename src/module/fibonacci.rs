// This program calculates the Fibonacci number at a given position.
//
// The user is prompted to enter the position of the Fibonacci number they want to calculate.
// The program then calculates the Fibonacci number at the given position using a recursive
// function `fibonacci_number`.
//
// # Example
//
// Enter the position of the Fibonacci sequence:
// 6
// The Fibonacci number at position 6 is: 8

use std::io;

pub fn fibonacci() {
    let mut input = String::new();
    println!("Enter the position of the Fibonacci sequence:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let fibonacci_pos: i32 = input.trim().parse().expect("Failed to parse input");

    let fibonacci_result = fibonacci_number(fibonacci_pos);

    println!("The Fibonacci number at position {} is: {}", fibonacci_pos, fibonacci_result);
}

fn fibonacci_number(position: i32) -> i32 {
    if position < 2 {
        position
    } else {
        fibonacci_number(position - 1) + fibonacci_number(position - 2)
    }
}
