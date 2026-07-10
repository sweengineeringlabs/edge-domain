//! `OutboundRegistry` impl for [`InMemoryOutboundRegistry`].

use std::collections::HashMap;

use parking_lot::RwLock;

use crate::api::DomainError;
use crate::api::InMemoryOutboundRegistry;
use crate::api::OutboundRegistry;
use crate::api::{OutboundDeregisterRequest, OutboundDeregisterResponse};
use crate::api::{OutboundGetRequest, OutboundGetResponse};
use crate::api::{OutboundIsEmptyRequest, OutboundIsEmptyResponse};
use crate::api::{OutboundLenRequest, OutboundLenResponse};
use crate::api::{OutboundNamesRequest, OutboundNamesResponse};
use crate::api::{OutboundRegisterRequest, OutboundRegisterResponse};

impl<H: Clone + Send + Sync> InMemoryOutboundRegistry<H> {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            handles: RwLock::new(HashMap::new()),
        }
    }
}

impl<H: Clone + Send + Sync> Default for InMemoryOutboundRegistry<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: Clone + Send + Sync> OutboundRegistry for InMemoryOutboundRegistry<H> {
    type Handle = H;

    fn register(
        &self,
        req: OutboundRegisterRequest<H>,
    ) -> Result<OutboundRegisterResponse, DomainError> {
        self.handles.write().insert(req.name, req.handle);
        Ok(OutboundRegisterResponse)
    }

    fn deregister(
        &self,
        req: OutboundDeregisterRequest,
    ) -> Result<OutboundDeregisterResponse, DomainError> {
        let removed = self.handles.write().remove(&req.name).is_some();
        Ok(OutboundDeregisterResponse { removed })
    }

    fn get(&self, req: OutboundGetRequest) -> Result<OutboundGetResponse<H>, DomainError> {
        let handle = self.handles.read().get(&req.name).cloned();
        Ok(OutboundGetResponse { handle })
    }

    fn names(&self, _req: OutboundNamesRequest) -> Result<OutboundNamesResponse, DomainError> {
        let names = self.handles.read().keys().cloned().collect();
        Ok(OutboundNamesResponse { names })
    }

    fn len(&self, _req: OutboundLenRequest) -> Result<OutboundLenResponse, DomainError> {
        Ok(OutboundLenResponse {
            count: self.handles.read().len(),
        })
    }

    fn is_empty(
        &self,
        _req: OutboundIsEmptyRequest,
    ) -> Result<OutboundIsEmptyResponse, DomainError> {
        Ok(OutboundIsEmptyResponse {
            empty: self.handles.read().is_empty(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_registry_happy() {
        let reg: InMemoryOutboundRegistry<String> = InMemoryOutboundRegistry::new();
        assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
        assert_eq!(reg.len(OutboundLenRequest).unwrap().count, 0);
    }

    #[test]
    fn test_register_then_get_returns_handle_happy() {
        let reg: InMemoryOutboundRegistry<String> = InMemoryOutboundRegistry::new();
        reg.register(OutboundRegisterRequest {
            name: "svc".into(),
            handle: "url".to_string(),
        })
        .unwrap();
        let handle = reg
            .get(OutboundGetRequest { name: "svc".into() })
            .unwrap()
            .handle;
        assert_eq!(handle.as_deref(), Some("url"));
    }

    #[test]
    fn test_deregister_missing_name_returns_false_error() {
        let reg: InMemoryOutboundRegistry<String> = InMemoryOutboundRegistry::new();
        let removed = reg
            .deregister(OutboundDeregisterRequest {
                name: "missing".into(),
            })
            .unwrap()
            .removed;
        assert!(!removed);
    }

    #[test]
    fn test_names_returns_all_registered_edge() {
        let reg: InMemoryOutboundRegistry<u32> = InMemoryOutboundRegistry::new();
        reg.register(OutboundRegisterRequest {
            name: "a".into(),
            handle: 1,
        })
        .unwrap();
        reg.register(OutboundRegisterRequest {
            name: "b".into(),
            handle: 2,
        })
        .unwrap();
        let mut names = reg.names(OutboundNamesRequest).unwrap().names;
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
    }
}
