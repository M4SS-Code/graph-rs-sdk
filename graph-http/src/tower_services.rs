use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::future;
use http::StatusCode;
use reqwest::{Request, Response};
use tokio::time::{sleep, Sleep};

#[derive(Clone)]
pub(crate) struct Attempts(pub usize);

impl tower::retry::Policy<Request, Response, Box<dyn std::error::Error + Send + Sync + 'static>>
    for Attempts
{
    type Future = future::Ready<Self>;

    fn retry(
        &self,
        _req: &Request,
        result: Result<&Response, &Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Option<Self::Future> {
        match result {
            Ok(response) => {
                if response.status().is_server_error() && self.0 > 0 {
                    return Some(future::ready(Attempts(self.0 - 1)));
                }
                None
            }
            Err(_) => {
                if self.0 > 0 {
                    Some(future::ready(Attempts(self.0 - 1)))
                } else {
                    None
                }
            }
        }
    }

    fn clone_request(&self, req: &Request) -> Option<Request> {
        req.try_clone()
    }
}

#[derive(Clone)]
pub(crate) struct WaitFor();

impl tower::retry::Policy<Request, Response, Box<dyn std::error::Error + Send + Sync + 'static>>
    for WaitFor
{
    type Future = future::Either<future::Ready<Self>, WaitBeforeRetry<Self>>;

    fn retry(
        &self,
        _req: &Request,
        result: Result<&Response, &Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Option<Self::Future> {
        match result {
            Ok(response) => match response.status() {
                StatusCode::TOO_MANY_REQUESTS
                | StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::SERVICE_UNAVAILABLE
                | StatusCode::GATEWAY_TIMEOUT => match response.headers().get("Retry-After") {
                    Some(retry_after) => match retry_after.to_str() {
                        Ok(ra) => match ra.parse::<u64>() {
                            Ok(retry_after) => {
                                let wait = WaitBeforeRetry::new(
                                    WaitFor(),
                                    Duration::from_secs(retry_after),
                                );
                                Some(future::Either::Right(wait))
                            }
                            Err(_) => None,
                        },
                        Err(_) => None,
                    },
                    None => None,
                },
                _ => None,
            },
            Err(_) => None,
        }
    }

    fn clone_request(&self, req: &Request) -> Option<Request> {
        req.try_clone()
    }
}

pub struct WaitBeforeRetry<T> {
    inner: Option<T>,
    sleep: Pin<Box<Sleep>>,
}

impl<T> WaitBeforeRetry<T> {
    pub fn new(inner: T, duration: Duration) -> Self {
        WaitBeforeRetry {
            inner: Some(inner),
            sleep: Box::pin(sleep(duration)),
        }
    }
}

impl<T: Unpin> Future for WaitBeforeRetry<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        match self.sleep.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => Poll::Ready(self.inner.take().expect("Ready polled after completion")),
        }
    }
}
