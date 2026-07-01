//! [`NameResponse`] — wrapper for `Service::name()` return value.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameResponse {
    pub name: String,
}
