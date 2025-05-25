use lrs::{Summary, NewsArticle};
use std::fmt::Display;

pub fn notify(item: &(impl Summary + Display)) {
    println!("news");
}


//fn some_function<T, U>(t: &T, u: &U) -> i32 
//where
//    T: Display + Clone,
//    U: Clone + Debug,
//{}

//fn returns_summarizable() -> impl Summary {}

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        println!("{}", self.x);
    }
}


fn main() {
    let article = NewsArticle {
        headline: String::from("abc"),
        author: String::from("abc"),
        content: String::from("asdsad"),
    };

    println!("{}", article.summarize());
}
