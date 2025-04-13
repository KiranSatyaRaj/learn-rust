use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // mutex in single thread context
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 5;
    }
    println!("m = {m:?}");

    // Sharing a Mutex<T> between multiple threads

    // atomic reference counted type, it's safe to use in concurrent situations
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // this won't compile
    // as the reference to s could be invalidated before the thread completes it's execution
    let s = String::from("Hello world");
    let a = Arc::new(&s);
    let a2 = Arc::clone(&a);
    let t = thread::spawn(move || a2.len());
    let len = t.join().unwrap();
    println!()
}