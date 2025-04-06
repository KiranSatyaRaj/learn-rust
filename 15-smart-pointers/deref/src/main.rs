// Defining our own smart pointer

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implementing Deref trait on MyBox
impl<T> Deref for MyBox<T> {
    type Target = T; // associated type for Deref trait

    fn deref(&self) -> &Self::Target {
        &self.0 // because MyBox is a tuple struct
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    // MyBox can't be dereferenced yet that ability for MyBox hasn't been implemented yet
    // so it won't compile yet
    assert_eq!(5, *y); // internally *(y.deref()) is executed

    let m = MyBox(String::from("Rust"));
    // here &String will be passed into hello
    // since String implements deref trait
    // implicit deref coercion converts it to &str
    // deref on &String to turn it into &str
    hello(&m);

    // without implicit deref coercion
    hello(&(*m)[..]);


}

fn hello(name: &str) {
    println!("Hello {name}");
}