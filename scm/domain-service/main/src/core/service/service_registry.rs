//! `ServiceRegistry` trait impl for the [`ServiceRegistry`] struct — in-process registry.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::ServiceRegistryTrait as ServiceRegistry;
use crate::api::ServiceRegistry as ServiceRegistryStore;
use crate::api::Service;

impl<Req, Resp> Default for ServiceRegistryStore<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn default() -> Self {
        ServiceRegistryStore {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

impl<Req, Resp> ServiceRegistry for ServiceRegistryStore<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    type Request = Req;
    type Response = Resp;

    fn register(&self, service: Arc<dyn Service<Request = Req, Response = Resp>>) {
        self.inner.write().insert(service.name().to_owned(), service);
    }

    fn deregister(&self, name: &str) -> bool {
        self.inner.write().remove(name).is_some()
    }

    fn get(&self, name: &str) -> Option<Arc<dyn Service<Request = Req, Response = Resp>>> {
        self.inner.read().get(name).cloned()
    }

    fn list_names(&self) -> Vec<String> {
        self.inner.read().keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.inner.read().len()
    }
}
