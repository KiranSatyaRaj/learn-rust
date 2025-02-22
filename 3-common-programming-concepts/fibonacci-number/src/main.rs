use std::io;

fn main() {
    println!("Please enter a positive number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim()
        .parse()
        .expect("Enter a positive number");

    println!("The fibonacci number is: {}", fibonacci_sum(number));
}

fn fibonacci_sum(number: u32) -> u32 {
    if number <= 1 {
        return 0;
    }

    if number == 2 {
        return 1;
    }
    fibonacci_sum(number - 1) + fibonacci_sum(number - 2)
}
