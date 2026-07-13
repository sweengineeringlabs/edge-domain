//! Basic `Repository` usage example.

use edge_domain_repository::MemoryRepository;

fn main() {
    let _cfg: MemoryRepository<String, u32> = MemoryRepository::new();
    println!("repository sub-crate ready");
}
