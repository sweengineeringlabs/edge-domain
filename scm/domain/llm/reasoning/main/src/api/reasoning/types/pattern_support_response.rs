/// Response for [`Reasoning::supports_pattern`](crate::api::reasoning::traits::Reasoning::supports_pattern).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternSupportResponse {
    /// Whether the queried pattern is supported.
    pub supported: bool,
}
