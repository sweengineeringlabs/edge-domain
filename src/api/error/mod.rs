//! Domain error types.

pub mod command_error;
pub mod event_error;
pub mod event_store_error;
pub mod handler_error;
pub mod query_error;
pub mod repository_error;
pub mod service_error;

pub use command_error::CommandError;
pub use event_error::EventError;
pub use event_store_error::EventStoreError;
pub use handler_error::HandlerError;
pub use query_error::QueryError;
pub use repository_error::RepositoryError;
pub use service_error::ServiceError;
