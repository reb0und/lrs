// use proc_macro;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("hello");
//     }
// }

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };

    Pancakes::hello_macro();
}

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

fn main() {}
