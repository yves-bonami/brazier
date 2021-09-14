use brazier::{Mediator, Request, RequestHandler};

pub struct Ping {}

impl Request<String> for Ping {}

#[derive(Debug)]
pub struct PingHandler;

#[async_trait::async_trait]
impl RequestHandler<Ping, String> for PingHandler {
    async fn handle(&mut self, _request: Ping) -> brazier::Result<String> {
        Ok(String::from("pong!"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Mediator
    let mut mediator = Mediator::new();

    // Register the PingHandler
    mediator.register_handler(PingHandler);

    // Send a Ping request
    let result = mediator.send(Ping {}).await?;

    // Print "pong!"
    println!("{}", result);

    Ok(())
}
