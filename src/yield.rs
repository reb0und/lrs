use std::time::Duration;
use std::thread;

fn main() {
    trpl::run(async {
        let a = async {
            println!("a started");
            slow("a", 20);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("a finished");
        };

        let b = async {
            println!("b started");
            slow("b", 200);
            trpl::yield_now().await;
            slow("b", 100);
            trpl::yield_now().await;
            slow("b", 200);
            trpl::yield_now().await;
            println!("b finished");
        };

        trpl::race(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}");
}
