//! Public trait contracts, types, and error types.

mod error;
mod traits;
pub mod types;

pub use error::PipelineError;
pub use traits::{Pipeline, Step};
pub use types::PipelineConfig;
