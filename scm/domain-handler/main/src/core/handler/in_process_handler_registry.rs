//! `HandlerRegistry` impl for [`InProcessHandlerRegistry`] — RwLock-backed in-process store.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::DeregisterHandlerRequest;
use crate::api::DeregisterHandlerResponse;
use crate::api::Handler;
use crate::api::HandlerError;
use crate::api::HandlerLookupRequest;
use crate::api::HandlerLookupResponse;
use crate::api::HandlerRegistry;
use crate::api::IdRequest;
use crate::api::InProcessHandlerRegistry;
use crate::api::LenRequest;
use crate::api::LenResponse;
use crate::api::ListIdsRequest;
use crate::api::ListIdsResponse;
use crate::api::RegisterHandlerRequest;
use crate::api::RegisterHandlerResponse;

impl<Req, Resp> RegisterHandlerRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Create a new registration request for the given handler.
    pub fn new(handler: Arc<dyn Handler<Request = Req, Response = Resp>>) -> Self {
        Self { handler }
    }
}

impl<Req, Resp> InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }
}

impl<Req, Resp> Default for InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Req: Send + 'static, Resp: Send + 'static> HandlerRegistry
    for InProcessHandlerRegistry<Req, Resp>
{
    type Request = Req;
    type Response = Resp;

    fn register(
        &self,
        req: RegisterHandlerRequest<Req, Resp>,
    ) -> Result<RegisterHandlerResponse, HandlerError> {
        let id = req.handler.id(IdRequest)?.id;
        self.handlers.write().insert(id, req.handler);
        Ok(RegisterHandlerResponse)
    }

    fn deregister(
        &self,
        req: DeregisterHandlerRequest,
    ) -> Result<DeregisterHandlerResponse, HandlerError> {
        let was_present = self.handlers.write().remove(&req.id).is_some();
        Ok(DeregisterHandlerResponse { was_present })
    }

    fn get(
        &self,
        req: HandlerLookupRequest,
    ) -> Result<HandlerLookupResponse<Req, Resp>, HandlerError> {
        let handler = self.handlers.read().get(&req.id).cloned();
        Ok(HandlerLookupResponse { handler })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, HandlerError> {
        let guard = self.handlers.read();
        let mut ids: Vec<String> = guard.keys().cloned().collect();
        ids.sort();
        Ok(ListIdsResponse { ids })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, HandlerError> {
        Ok(LenResponse {
            count: self.handlers.read().len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::EmptinessRequest;

    #[test]
    fn test_new_creates_empty_registry_happy() {
        let reg: InProcessHandlerRegistry<String, String> = InProcessHandlerRegistry::new();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }
}
