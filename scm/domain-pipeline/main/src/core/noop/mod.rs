//! No-operation step implementations.

pub(crate) mod concrete_step;
pub(crate) mod noop_step;

pub(crate) use concrete_step::ConcreteStep;
pub(crate) use noop_step::NoopStep;
