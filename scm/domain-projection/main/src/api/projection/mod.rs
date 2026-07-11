pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ProjectionError;
pub use traits::Projection;
pub use traits::ProjectionEvent;
pub use types::{
    InMemoryProjection, ProjectionApplyRequest, ProjectionEventDescribeRequest,
    ProjectionEventDescribeResponse, ProjectionReadModelRequest, ProjectionReadModelResponse,
    TryDrainRequest, TryDrainResponse,
};
