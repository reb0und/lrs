use std::thread;
use std::time::Duration; 

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vec {v:?}");
    });

    handle.join().unwrap();
}
