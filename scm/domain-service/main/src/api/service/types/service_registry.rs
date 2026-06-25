//! API-layer type for the in-process service registry.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::service::traits::service::Service;

/// A thread-safe, in-process registry mapping service names to [`Service`] implementations.
///
/// Construction and inherent methods live in `core::service::service_registry` — api/ is a
/// declaration layer only.
pub struct ServiceRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    pub(crate) inner: RwLock<HashMap<String, Arc<dyn Service<Request = Req, Response = Resp>>>>,
}
