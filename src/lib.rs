//! An implemenation of the mediator pattern.
//!
//! Brazier is heavily inspired by the [.NET MediatR pacakage](https://github.com/jbogard/MediatR).
//! It allows you to decouple the sending of a message and the handling of it.
//!
//! # Example
//!
//! ```rust
//! use brazier::*;
//!
//! pub struct Ping {}
//!
//! impl Request<String> for Ping {}
//!
//! #[derive(Debug)]
//! pub struct PingHandler;
//!
//! #[async_trait::async_trait]
//! impl RequestHandler<Ping, String> for PingHandler {
//!     async fn handle(&mut self, _request: Ping) -> Result<String> {
//!         Ok(String::from("pong!"))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut mediator = Mediator::new();
//!     mediator.register_handler(PingHandler);
//!     let result = mediator.send(Ping {}).await?;
//!     println!("{}", result);
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![deny(unsafe_code)]

use async_trait::async_trait;
use std::fmt::Debug;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
};

mod error;
pub use self::error::MediatorError;

/// The result type returned by the RequestHandler.
pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

/// The request trait.
pub trait Request<TResponse>: 'static {}

/// The request handler trait. The handler is responsible for handling the request.
#[async_trait]
pub trait RequestHandler<TRequest, TResponse>
where
    TRequest: Request<TResponse>,
{
    /// The method that handles the request.
    async fn handle(&mut self, request: TRequest) -> Result<TResponse>;
}

/// The mediator trait.
#[derive(Debug)]
pub struct Mediator(TypeMap);

impl Mediator {
    /// Creates a new mediator.
    pub fn new() -> Self {
        Mediator(TypeMap::new())
    }

    /// Registers a request handler.
    pub fn register_handler<TRequest, TRequestHandler, TResponse>(
        &mut self,
        handler: TRequestHandler,
    ) -> &mut Self
    where
        TRequest: Request<TResponse>,
        TRequestHandler: RequestHandler<TRequest, TResponse> + 'static,
        TResponse: 'static,
    {
        self.0
            .set::<TRequest, Box<dyn RequestHandler<TRequest, TResponse>>>(Box::new(handler));
        self
    }

    /// Send a request to the mediator.
    pub async fn send<TRequest, TResponse>(&mut self, request: TRequest) -> Result<TResponse>
    where
        TRequest: Request<TResponse>,
        TResponse: 'static,
    {
        match self
            .0
            .get_mut::<TRequest, Box<dyn RequestHandler<TRequest, TResponse>>>()
        {
            Some(h) => h.handle(request).await,
            None => Err(Box::new(error::MediatorError::HandlerNotRegisteredError)),
        }
    }
}

#[derive(Debug)]
struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

impl TypeMap {
    fn new() -> Self {
        TypeMap(HashMap::new())
    }

    pub fn set<TKey: 'static, TValue: Any + 'static>(&mut self, value: TValue) {
        self.0.insert(TypeId::of::<TKey>(), Box::new(value));
    }

    pub fn get_mut<TKey: 'static, TValue: Any + 'static>(&mut self) -> Option<&mut TValue> {
        self.0
            .get_mut(&TypeId::of::<TKey>())
            .and_then(|v| v.downcast_mut::<TValue>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    pub struct TestRequest {}

    #[derive(Debug)]
    pub struct TestRequestHandler;

    impl Request<i64> for TestRequest {}

    #[async_trait]
    impl RequestHandler<TestRequest, i64> for TestRequestHandler {
        async fn handle(&mut self, _request: TestRequest) -> Result<i64> {
            Ok(42)
        }
    }

    #[tokio::test]
    async fn test_mediator_register_handler() {
        let mut m = Mediator::new();
        m.register_handler(TestRequestHandler);
        assert_eq!(m.send(TestRequest {}).await.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_mediator_no_handler_registered() {
        let mut m = Mediator::new();
        match m.send(TestRequest {}).await {
            Ok(_) => assert!(false),
            Err(err) => {
                if let Some(e) = err.downcast_ref::<MediatorError>() {
                    assert_eq!(e, &error::MediatorError::HandlerNotRegisteredError);
                } else {
                    assert!(false);
                }
            }
        }
    }
}
