//! SAF — value-object sub-module facades.

pub mod value_object_bootstrap_svc;
pub mod value_object_svc;

pub use value_object_bootstrap_svc::ValueObjectBootstrap;
pub use value_object_bootstrap_svc::VALUE_OBJECT_FACTORY_SVC;
pub use value_object_svc::NonEmptyString;
pub use value_object_svc::ValueObject;
pub use value_object_svc::ValueObjectError;
pub use value_object_svc::VALUE_OBJECT_SVC;
