//use std::{cmp::Ordering, io};
use std::io::{self, Write};

mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_bad_order() {
        super::deliver_order()
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_rest() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    //meal.seasonal_fruit = String::from("ax");
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;

    front_of_house::hosting::add_to_waitlist();
}
