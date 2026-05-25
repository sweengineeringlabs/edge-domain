//! Domain error types.

pub mod command_error;
pub mod event;
pub mod handler_error;
pub mod query_error;
pub mod repository_error;
pub mod service_error;

pub use command_error::CommandError;
pub use event::EventError;
pub use event::EventStoreError;
pub use handler_error::HandlerError;
pub use query_error::QueryError;
pub use repository_error::RepositoryError;
pub use service_error::ServiceError;
