use std::future::Future;
use crate::error::Result;

pub trait Sendable {
    type Output;

    fn send(self) -> impl Future<Output=Result<Self::Output>> + Send + 'static;
}