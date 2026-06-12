//! Basic `Saga` usage example.

use edge_domain_command::{Command, CommandError};
use edge_domain_event::DomainEvent;
use edge_domain_saga::Saga;
use futures::future::BoxFuture;

struct Signal;
impl DomainEvent for Signal {}
impl Command for Signal {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct MySaga {
    done: bool,
}

impl Saga for MySaga {
    type SagaId = String;
    type Event = Signal;
    type Command = Signal;
    fn handle(&mut self, _e: &Signal) -> Vec<Signal> {
        self.done = true;
        vec![]
    }
    fn is_complete(&self) -> bool { self.done }
}

fn main() {
    let mut s = MySaga { done: false };
    s.handle(&Signal);
    println!("complete: {}", s.is_complete());
}
