use std::time::Duration;

fn main() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("{i} from spawned task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("{i} from main thread");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}
