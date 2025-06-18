use std::ops::Deref;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}", self.data);
    }
}

struct LinkedList<T> (T, Option<Box<LinkedList<T>>>);

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}
