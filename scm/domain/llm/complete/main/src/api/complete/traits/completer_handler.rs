//! `CompleterHandler` — marker supertrait for types that are both a [`Completer`] and a [`Processor`].

use crate::api::complete::traits::{Completer, Processor};

/// Marker: types that implement both [`Completer`] and [`Processor`].
///
/// A `CompleterHandler` is the canonical wiring point — it is the type placed
/// into the `HandlerRegistry` when an agent needs to invoke an LLM backend.
pub trait CompleterHandler: Completer + Processor + Send + Sync {}
