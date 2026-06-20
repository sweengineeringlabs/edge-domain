//! Basic `Command` usage example.

use edge_domain_command::{Command, CommandBusBootstrap, CommandError};
use futures::future::BoxFuture;

struct Buses;
impl CommandBusBootstrap for Buses {}

struct Ping;
impl Command for Ping {
    fn name(&self) -> &str { "ping" }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::main]
async fn main() {
    let bus = Buses::direct();
    let result = edge_domain_command::CommandBus::dispatch(&bus, Box::new(Ping)).await;
    println!("{:?}", result);
}
