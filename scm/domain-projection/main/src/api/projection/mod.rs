pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ProjectionError;
pub use traits::Projection;
pub use types::{
    InMemoryProjection, ProjectionApplyRequest, ProjectionReadModelRequest,
    ProjectionReadModelResponse, TryDrainRequest, TryDrainResponse,
};
