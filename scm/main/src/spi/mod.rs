//! SPI layer — extension hooks and external-library implementations.
//!
//! Mirrors the `api/` theme structure. External-library-backed implementations
//! (e.g. the tokio broadcast event bus) live under `spi/{theme}/{technology}/`
//! and are exported solely through `saf/`.

pub(crate) mod event;

mod noop_domain_extension;
