//! Value-object theme — port contracts.

pub mod value_object;
pub mod value_object_factory;

pub use value_object::ValueObject;
pub use value_object_factory::ValueObjectFactory;

pub use crate::api::valueobject::types::NonEmptyString;
