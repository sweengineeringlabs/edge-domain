//! [`LenResponse`] — wrapper for registry length.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LenResponse {
    pub count: usize,
}
