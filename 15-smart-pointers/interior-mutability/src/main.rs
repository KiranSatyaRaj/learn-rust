use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let mut b = Box::new(String::from("Hello "));
    let string_ref = &mut *b; // you cannot borrow a mutable reference from an immutable type
    string_ref.push_str("world!");
    println!("{b}");

    // RefCell<T> is one way to get interior mutability
    // the check for RefCell<T> happens at runtime and
    // if it doesn't follow the borrowing rules we'll
    // get a panic at runtime

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}