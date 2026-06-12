//! Basic `Repository` usage example.

use edge_domain_repository::{InMemoryRepository, RepositoryFactory};

struct Repos;
impl RepositoryFactory for Repos {}

fn main() {
    let _cfg: InMemoryRepository<String, u32> = Repos::in_memory();
    println!("repository sub-crate ready");
}
