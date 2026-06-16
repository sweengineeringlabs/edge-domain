//! Basic `Entity` usage example.

use edge_domain_entity::Entity;

struct OrderLine {
    id: u64,
    #[allow(dead_code)]
    quantity: u32,
}

impl Entity for OrderLine {
    type Id = u64;
    fn id(&self) -> &u64 {
        &self.id
    }
}

fn main() {
    let line = OrderLine {
        id: 1,
        quantity: 10,
    };
    println!("order line id: {}", line.id());
}
