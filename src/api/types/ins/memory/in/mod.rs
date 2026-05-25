//! In-memory storage for events and repositories.

pub mod in_memory_event_store;
pub mod in_memory_repository;

pub use in_memory_event_store::InMemoryEventStore;
pub use in_memory_repository::InMemoryRepository;
