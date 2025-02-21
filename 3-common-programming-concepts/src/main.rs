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

    // 3.3 Functions

    another_function(32);

    print_label_measurement(32, 'h');

    // Statements and Expressions

    // This line results in error
    // let x = (let y = 6); // This is a statement and statements don't return a value.

    // This is an expression
    let y = {
        let x = 3;
        x + 1 // This an expression and it evaluates to 4 and is returned to y, also has no semicolon
    };

    println!("The value of y is : {y}");

    // Functions with return values
    let y = five();
    println!("The value of y is: {y}");

    let x = plus_one(7);
    println!("The value of x is: {x}");

    println!("{}", plus_one({

        let y = 1;

        y + 1

    }));

    // 3.4 Comments

    // This is a comment

    /*
        This is a longer comment than the previous comment
        Also a multi-line comment
     */

    // 3.5 Control-Flow

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loop {
    //     println!("again");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the element is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x that is passed is: {x}");
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 { // we don't name return values, but we must declare its type
    5 // implicit return
}

fn plus_one(x: i32) -> i32 {
    x + 1
}