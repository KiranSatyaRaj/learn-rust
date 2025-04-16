use std::pin::pin;
use std::time::Duration;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    // trpl::run(async {
    //     let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let iter = values.iter().map(|n| n * 2);
    //     let mut stream = trpl::stream_from_iter(iter);
    //
    //     while let Some(value) = stream.next().await {
    //         println!("The value was: {value}");
    //     }
    // });
    //
    // trpl::run(async {
    //     let values = 1..101;
    //     let iter = values.map(|n| n * 2);
    //     let stream = trpl::stream_from_iter(iter);
    //
    //     let mut filtered =
    //         stream.filter(|value| value % 3 == 0 || value % 5 == 0);
    //
    //     while let Some(value) = filtered.next().await {
    //         println!("The value was: {value}");
    //     }
    // });
    //
    // trpl::run(async {
    //     let mut messages = get_messages();
    //
    //     while let Some(message) = messages.next().await {
    //         println!("{message}");
    //     }
    // });
    //
    // trpl::run(async {
    //     let mut messages =
    //     pin!(get_messages().timeout(Duration::from_millis(200)));
    //
    //     while let Some(result) = messages.next().await {
    //         match result {
    //             Ok(message) => println!("{message}"),
    //             Err(reason) => eprintln!("Problem: {reason:?}"),
    //         }
    //     }
    // });

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)

}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task (async  move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep: u64 = if index % 2 == 0 { 200 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: {message}")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

   ReceiverStream::new(rx)
}