//! SPI hook for domain assembly extension consumers.

/// Extension-point marker — implement to register custom domain assembly hooks.
///
/// Downstream consumers implement this on zero-size types to participate in
/// the domain assembly lifecycle without altering any public port contracts.
pub trait DomainSpi: Send + Sync {}
