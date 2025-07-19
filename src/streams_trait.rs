use std::pin::Pin;
use std::task{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;
}

trait StreamExt {
    async fn next(&mut self) -> Option<Self::Item>
    where 
        Self: Unpin;
}
