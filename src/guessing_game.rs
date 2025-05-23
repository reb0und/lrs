use std::io;
use std::cmp::Ordering;

use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("bad value");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let num = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("guess: {guess}");

        let guess: u8 = match guess.trim().parse() {
            Ok(a) => a,
            Err(e) => {
                println!("err: {e}");
                continue
            },
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("good");
                break;
            }
        }
    }

}

