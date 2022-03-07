# Brazier

An implemenation of the mediator pattern.

Brazier is heavily inspired by the [.NET MediatR pacakage](https://github.com/jbogard/MediatR).
It allows you to decouple the sending of a message and the handling of it.

## Example
```rust
use brazier::*;

pub struct Ping {}

impl Request<String> for Ping {}

#[derive(Debug)]
pub struct PingHandler;

#[async_trait::async_trait]
impl RequestHandler<Ping, String> for PingHandler {
    async fn handle(&mut self, _request: Ping) -> Result<String> {
        Ok(String::from("pong!"))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut mediator = Mediator::new();
    mediator.register_handler(PingHandler);
    let result = mediator.send(Ping {}).await?;
    println!("{}", result);
    Ok(())
}
```
