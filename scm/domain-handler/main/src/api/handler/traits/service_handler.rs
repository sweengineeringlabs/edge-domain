//! [`ServiceHandler`] — combined marker trait for the service-to-handler bridge.

use crate::api::handler::traits::service_bridge::ServiceBridge;
use crate::api::handler::traits::validator::Validator;

/// Marker trait that combines [`ServiceBridge`] and [`Validator`].
///
/// Implemented by the concrete adapter in `core/`. Consumers receive an opaque
/// `impl Handler + ServiceHandler` from [`IntoHandler::into_handler`].
pub trait ServiceHandler: ServiceBridge + Validator {}
