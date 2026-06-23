//! SAF — entity service facade.

mod entity_svc;

pub use entity_svc::{Entity, ENTITY_SVC};
pub(crate) use entity_svc::EntityError;
