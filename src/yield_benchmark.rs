use std::time::{Instant, Duration};

fn main() {
    trpl::run(async {
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(Duration::from_nanos(1)).await;
            }
        }.await;

        let time = Instant::now() - start;
        println!(
            "sleep version finished after {} secs", 
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }.await;
        let time = Instant::now() - start;
        println!(
            "yield version finished in {}", 
            time.as_secs_f32()
        );
    });
}
