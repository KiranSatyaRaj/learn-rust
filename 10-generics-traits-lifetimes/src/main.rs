fn main() {
    let number_list = vec![10, 20, 30, 40, 50];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let result = largest(number_list);
    println!("The largest number is {result}");

    // assume that we've been tasked with finding the large number
    // in two different list
    // we could do the same thing as we did above
    // code duplication s*cks/tedious and error-prone

    let number_list = vec![1000, 24254, 6, 35, 152, 2515];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");

    let result = largest(number_list);
    println!("The largest number is {result}");

    // summary
    // identify duplicated code
    // extract that into a function, and specify inputs and return value of that code in function signature
    // update the instance of duplicated code to use the function instead
}

// Do not Repeat yourself
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}