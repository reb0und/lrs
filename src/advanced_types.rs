use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

type Thunk = Box<dyn Fn() + Send + 'static>;

type Kilometers = i32;

fn bar() -> ! {
    loop {}
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}
