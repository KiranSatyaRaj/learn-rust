use std::pin::Pin;
use std::task::{Context, Poll};

// Future in Rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// Poll type
enum Poll<T> {
    Ready(T),
    Pending
}

fn main() {
    // what happens when you call .await
    // match hello("async").poll() {
    //     Ready(_) => {
    //         // We're done!
    //     }
    //     Pending => {
    //         // But what goes here?
    //     }
    // }

    let hello_fut = hello("async");
    loop {
        match hello_fut.poll() {
            Ready(_) => {
                break;
            }
            Pending => {
                // continue
            }
        }
    }

    let mut v: Vec<i32> = Vec::new();
    v.push(3);

}