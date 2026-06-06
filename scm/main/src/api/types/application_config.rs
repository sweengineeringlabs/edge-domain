//! Application configuration for the domain layer.

/// Runtime configuration for edge-domain.
///
/// Currently empty as the domain layer requires no runtime tuning for a pure library.
#[derive(Debug, Clone, Default)]
pub struct ApplicationConfig;

impl ApplicationConfig {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self
    }
}
