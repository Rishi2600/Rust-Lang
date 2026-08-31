use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CustomFuture {
    count: u32,
}

impl Future for CustomFuture {
    type Output = u32;

    // The 'self: Pin<&mut Self>' receiver enforces that this future 
    // cannot move in memory while it is actively being polled.
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        if self.count >= 3 {
            Poll::Ready(self.count)
        } else {
            Poll::Pending
        }
    }
}