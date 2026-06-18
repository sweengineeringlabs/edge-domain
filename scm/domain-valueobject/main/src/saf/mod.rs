//! SAF — value object service facade.

mod value;

pub use value::NonEmptyString;
pub use value::ValueObject;
pub use value::ValueObjectError;
pub use value::ValueObjectFactory;
pub use value::VALUE_OBJECT_FACTORY_SVC;
pub use value::VALUE_OBJECT_SVC;
