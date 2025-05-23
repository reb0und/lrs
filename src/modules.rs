use crate::garden::vegetables::Carrot;
use std::collections::HashMap;

pub mod garden;

use std::fmt::Result;
use std::io::Result as IoResult;

//fn function1() -> Result {}
//fn function2() -> IoResult<()> {}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let plant = Carrot {};
    println!("{plant:?}");
}
