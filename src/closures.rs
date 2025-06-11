use std::thread;
use std::time;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref = Some(ShirtColor::Blue);
    let giveaway = store.giveaway(user_pref);

    println!("user with preference: {:?} gets {:?}", user_pref, giveaway);

    let user_pref = None;
    let giveaway = store.giveaway(user_pref);

    println!("user with preferece: {:?} gets {:?}", user_pref, giveaway);

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating...");
        thread::sleep(time::Duration::from_secs(2));
        num
    };

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let two = add_one_v3(1);
    let two = add_one_v4(1);

    //let example_closure = |x| x;
    //
    //let s = example_closure(String::from("hello"));
    //let n = example_closure(5);
}

fn add_one_v1(x: u32) -> u32 { x + 1 }
