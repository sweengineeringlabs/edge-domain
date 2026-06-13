//! Basic `Query` usage example.

use edge_domain_query::{DirectQueryBus, Query, QueryBusFactory, QueryError};
use futures::future::BoxFuture;

struct Buses;
impl QueryBusFactory for Buses {}

struct Ping;
impl Query for Ping {
    type Result = String;

    fn name(&self) -> &str { "ping" }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Ok("pong".into()) })
    }
}

fn main() {
    let bus: DirectQueryBus<String> = Buses::direct();
    let _query: Box<dyn Query<Result = String>> = Box::new(Ping);
    let _ = &bus;
    println!("query sub-crate ready");
}
