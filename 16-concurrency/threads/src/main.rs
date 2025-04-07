use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // this will make sure the thread finishes before main exits
    // it blocks the current thread running and runs until the
    // handle terminates, here handle thread will be executed first
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // error! Rust doesn't know for how long this thread will run
    // vec can be dropped, in that case v is no longer valid
    // we can use move keyword for closure to take ownership of
    // values its using
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    drop(v);

    handle.join().unwrap();
}