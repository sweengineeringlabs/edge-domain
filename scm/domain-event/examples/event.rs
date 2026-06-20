//! Basic `DomainEvent` usage example.

use edge_domain_event::{DomainEvent, EventBootstrap};

struct Events;
impl EventBootstrap for Events {}

struct OrderCreated {
    order_id: String,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self) -> &str { "order.created" }
    fn aggregate_id(&self) -> &str { &self.order_id }
}

fn main() {
    let _bus = Events::noop_bus();
    println!("event sub-crate ready");
}
