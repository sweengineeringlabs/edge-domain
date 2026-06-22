//! Public trait contracts and error types.

mod error;
mod traits;

pub use error::PipelineError;
pub use traits::{Pipeline, Step};
