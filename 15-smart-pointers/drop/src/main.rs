struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // this method will be called automatically once the instances of the type go out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // can't call drop method manually Rust doesn't allow you to do that
    // as the type goes out scope rust will again call drop on the instance
    // which causes double-free
    // c.drop();

    drop(c);
    println!("CustomSmartPointer dropped before end of main.");

}