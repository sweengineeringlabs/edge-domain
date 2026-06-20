//! Basic `Repository` usage example.

use edge_domain_repository::{InMemoryRepository, RepositoryBootstrap};

struct Repos;
impl RepositoryBootstrap for Repos {}

fn main() {
    let _cfg: InMemoryRepository<String, u32> = Repos::in_memory();
    println!("repository sub-crate ready");
}
