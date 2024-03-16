// This program convert the fahrenheit_to_celsius
// The user is prompted to enter the temperature in Fahrenheit

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter temperature in Fahrenheit");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let temperature: f32 = input.trim().parse().expect("Failed to parse the input");

    let celsius = fahrenheit_to_celsius(temperature);
    println!("Fahrenheit to Celsius = {}", celsius);
}

fn fahrenheit_to_celsius (temperature: f32) -> f32 {
    (temperature - 32.0) * 5.0/9.0
}
