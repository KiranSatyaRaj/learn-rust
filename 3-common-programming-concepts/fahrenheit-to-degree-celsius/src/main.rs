use std::io;

fn main() {

    println!("Please enter degree fahrenheit");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read temperature");

    let degree_fahrenheit: f32 = fahrenheit.trim()
        .parse()
        .expect("Please type a number!");

    println!("Temperature in celsius is {}", fahrenheit_to_celsius(degree_fahrenheit));
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
