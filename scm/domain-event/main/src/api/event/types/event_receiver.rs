//! [`EventReceiver`] — owned handle over an [`EventSource`].

use crate::api::event::traits::EventSource;

/// An owned, type-erased handle over an [`EventSource`].
///
/// Obtained from [`EventBus::subscribe`](super::super::traits::EventBus::subscribe).
/// Call [`recv`](EventReceiver::recv) to pull the next event from the underlying source.
pub struct EventReceiver(pub(crate) Box<dyn EventSource>);
