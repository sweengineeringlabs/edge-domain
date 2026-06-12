//! Basic `Query` usage example.

use edge_domain_query::{Query, QueryBusFactory, QueryError};
use futures::future::BoxFuture;

struct Buses;
impl QueryBusFactory for Buses {}

struct Ping;
impl Query<String> for Ping {
    fn name(&self) -> &str { "ping" }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Ok("pong".into()) })
    }
}

fn main() {
    let bus = Buses::direct();
    let _query: Box<dyn Query<String>> = Box::new(Ping);
    let _ = &bus;
    println!("query sub-crate ready");
}
