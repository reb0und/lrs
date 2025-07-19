use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

fn main() {
    loop {
        match page_title(url).poll() {
            Ready(page_title) => match page_title {
                Some(title) => println!("title for {url} was {title}"),
                None => println!("{url} had no title"),
            },
            Pending => {
                // ?
            }
        }
    }
}
