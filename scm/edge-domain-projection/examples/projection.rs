//! Basic `Projection` usage example.

use edge_domain_event::DomainEvent;
use edge_domain_projection::Projection;

struct Credited {
    amount: u64,
}
impl DomainEvent for Credited {}

struct Balance {
    total: u64,
}

impl Projection for Balance {
    type Event = Credited;
    type ReadModel = u64;
    fn apply(&mut self, e: &Credited) { self.total += e.amount; }
    fn read_model(&self) -> &u64 { &self.total }
}

fn main() {
    let mut b = Balance { total: 0 };
    b.apply(&Credited { amount: 42 });
    println!("balance: {}", b.read_model());
}
