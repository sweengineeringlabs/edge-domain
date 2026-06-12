#[allow(clippy::module_inception)]
pub mod queryable_repository;
pub mod repository;
pub mod repository_factory;

pub use queryable_repository::QueryableRepository;
pub use repository::Repository;
pub use repository_factory::RepositoryFactory;
