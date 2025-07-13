use std::pin::pin;
use std::time::Duration;
use std::thread;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let intervals = get_intervals()
            .throttle(Duration::from_millis(100))
            .take(10);

        let mut stream = pin!(intervals);

        while let Some(count) = stream.next().await {
            println!("count: {count}");
        }
    });
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("error sending {count} {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
