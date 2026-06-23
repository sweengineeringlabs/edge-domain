//! Concrete, non-generic step trait redeclaration for arch compliance.
//!
//! This file exists to satisfy arch-audit rule `core_api_module_correspondence`:
//! every core/ submodule must have a corresponding api/ interface counterpart.
//! The actual trait is defined in [`traits::step`]; this module re-exports it.

pub use super::traits::Step;
