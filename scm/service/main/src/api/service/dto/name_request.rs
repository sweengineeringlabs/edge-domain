//! [`NameRequest`] — zero-sized marker for `Service::name()`.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NameRequest;
