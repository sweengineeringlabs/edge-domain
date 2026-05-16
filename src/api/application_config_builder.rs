//! Builder types for domain configuration.
//!
//! Impl blocks live in the `saf` layer. Struct shapes are declared here so
//! types are anchored in the interface layer per SEA rule 160.

/// Builder for domain application configuration.
///
/// Construct via [`crate::saf::builder`]. Finalize with [`ApplicationConfigBuilder::build`].
pub struct ApplicationConfigBuilder {
    _private: (),
}

impl ApplicationConfigBuilder {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_builder_constructs() {
        let _b = ApplicationConfigBuilder::new();
    }
}
