use std::future::Future;
use std::pin::{pin, Pin};
use std::process::Output;
use std::thread;
use std::time::{Duration, Instant};
use trpl::Either;

fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });


        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });

    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("Hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received: {value}");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // you can add arbitrary # of futures in join macro
        // trpl::join!(tx_fut, tx1_fut, rx_fut);

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, tx1_fut, rx_fut];
        trpl::join_all(futures).await; // dynamic number of futures with same Output future type
    });

    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c); // for different output future type
        println!("{a_result} {b_result} {c_result}");
    });

    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished");
        };

        let fast = async {
            println!("'fast' started");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished");
        };

        trpl::race(slow, fast).await;
    });

    trpl::run(async {
        let one_ms = Duration::from_millis(1);
        let a = async {
            println!("'a' started");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished");
        };

        // this is just a simulation for CPU bound process
        // ideally we want our program to yield control
        // to the runtime instead of sleeping
        let b = async {
            println!("'b' started");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished");
        };

        trpl::race(a, b).await;
    });


    trpl::run(async {
        let one_ms = Duration::from_millis(1);
        let a = async {
            println!("'a' started");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished");
        };

         let b = async {
             println!("'b' started");
             slow("b", 75);
             trpl::yield_now().await;
             slow("b", 10);
             trpl::yield_now().await;
             slow("b", 15);
             trpl::yield_now().await;
             slow("b", 350);
             trpl::yield_now().await;
             println!("'b' finished");
        };

        trpl::race(a, b).await;
    });

    // comparing performance b/w sleep and yield

    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 0..1000 {
                trpl::sleep(one_ns).await;
            }
        }
            .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds",
            time.as_secs_f32(),
        );

        let start = Instant::now();
        async {
            for _ in 0..1000 {
                trpl::yield_now().await;
            }
        }
            .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds",
            time.as_secs_f32(),
        );
    });

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished!"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });


}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(ouput) => Ok(ouput),
        Either::Right(_) => Err(max_time),
    }
}