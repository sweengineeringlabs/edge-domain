//! Basic `Command` usage example.

use edge_domain_command::{
    Command, CommandBusBootstrap, CommandDispatchRequest, CommandError, ExecutionRequest,
    NameRequest, NameResponse,
};
use futures::future::BoxFuture;

struct Buses;
impl CommandBusBootstrap for Buses {}

struct Ping;
impl Command for Ping {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "ping".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::main]
async fn main() {
    let bus = Buses::direct();
    let result = edge_domain_command::CommandBus::dispatch(
        &bus,
        CommandDispatchRequest {
            command: Box::new(Ping),
        },
    )
    .await;
    println!("{:?}", result);
}
