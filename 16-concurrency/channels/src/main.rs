use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc: multiple producer single consumer
    // multiple sending ends and one receiver end
    let (tx, rx) = mpsc ::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received =  rx.recv().unwrap(); // recv() method blocks main thread until a msg is sent down the channel
    println!("Got: {received}");

    let (tx, rx) = mpsc ::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap(); // val is moved here, send takes ownership of its params
        // println!("val is {val}"); // so this won't compile
    });

    let received =  rx.recv().unwrap();
    println!("Got: {received}");

    // Sending multiple values and Seeing the Receiver Waiting

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in v{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    for received in rx {
        println!("Got: {received}");
    }

    // Creating multiple producers by transmitter

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}")
    }
}