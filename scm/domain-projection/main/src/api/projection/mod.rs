pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ProjectionError;
pub use traits::{Projection, ProjectionBootstrap};
pub use types::{
    BootstrapNameRequest, BootstrapNameResponse, InMemoryProjection, ProjectionApplyRequest,
    ProjectionReadModelRequest, ProjectionReadModelResponse, StdProjectionFactory, TryDrainResponse,
};
