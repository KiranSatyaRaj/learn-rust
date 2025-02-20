use std::io;
fn main() {

    // 3.1 Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The number of seconds in three hours: {THREE_HOURS_IN_SECONDS}");
    // Shadowing

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // let mut spaces = "    ";
    // spaces = spaces.len();

    // 3.2 Data Types
    // Integer Types
    let guess: u32 = "42".parse().expect("Not a number!");

    // Floating-Point Types

    let x = 3.0; // f64

    let y: f32 = 51.14; // f32

    // Basic Numeric Operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean Type
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Character Type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let mut x: (i32, i32) = (1, 2);
    x.0 = 42;
    x.1 += 11;

    // Array Type

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 3]; // is the same as [3, 3, 3]

    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at the index: {index} is {element}");
}