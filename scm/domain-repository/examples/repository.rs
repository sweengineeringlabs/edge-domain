//! Basic `Repository` usage example.

use edge_domain_repository::InMemoryRepository;

fn main() {
    let _cfg: InMemoryRepository<String, u32> = InMemoryRepository::new();
    println!("repository sub-crate ready");
}
