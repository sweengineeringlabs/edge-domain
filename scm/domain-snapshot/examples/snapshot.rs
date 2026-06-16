//! Basic `Snapshot` usage example.

use edge_domain_snapshot::Snapshot;

struct OrderSnap {
    id: String,
    version: u64,
}

impl Snapshot for OrderSnap {
    type AggregateId = String;
    fn aggregate_id(&self) -> &String { &self.id }
    fn version(&self) -> u64 { self.version }
}

fn main() {
    let s = OrderSnap { id: "order-1".into(), version: 5 };
    println!("snapshot v{} for {}", s.version(), s.aggregate_id());
}
