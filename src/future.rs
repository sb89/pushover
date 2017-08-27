use futures::{Future, Poll};
use error::Error;

/// A future that resolves into a Pushover response.
pub struct PushoverFuture<T> {
    inner: Box<Future<Item = T, Error = Error>>,
}

impl<T> PushoverFuture<T> {
    pub(crate) fn new(inner: Box<Future<Item = T, Error = Error>>) -> Self {
        Self { inner: inner }
    }
}

impl<T> Future for PushoverFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}
