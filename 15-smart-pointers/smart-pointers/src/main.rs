use crate::List::{Cons, Nil};

// Recursive type
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Non-Recursive Type
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // storing data on heap
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}