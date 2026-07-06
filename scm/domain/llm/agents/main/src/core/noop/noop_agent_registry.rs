//! No-op [`AgentRegistry`] implementation for testing the contract.

use crate::api::NoopAgentRegistry;
use crate::api::{Agent, AgentError, AgentRegistry};
use crate::api::{AgentMetadataLookupRequest, AgentMetadataLookupResponse};
use edge_domain_registry::{
    DeregisterRequest, DeregisterResponse, EmptinessRequest, EmptinessResponse, LenRequest,
    LenResponse, ListIdsRequest, ListIdsResponse, RegisterRequest, RegisterResponse, Registry,
    RegistryError, RegistryLookupRequest, RegistryLookupResponse, TryRegisterRequest,
    TryRegisterResponse,
};

impl Registry for NoopAgentRegistry {
    type Value = dyn Agent;

    fn register(&self, req: RegisterRequest<Self::Value>) -> Result<RegisterResponse, RegistryError> {
        // No-op registry: the entry is intentionally discarded.
        let _ = req;
        Ok(RegisterResponse)
    }

    fn try_register(
        &self,
        req: TryRegisterRequest<Self::Value>,
    ) -> Result<TryRegisterResponse, RegistryError> {
        let _ = req;
        Ok(TryRegisterResponse)
    }

    fn deregister(&self, req: DeregisterRequest) -> Result<DeregisterResponse, RegistryError> {
        let _ = req;
        Ok(DeregisterResponse {
            was_present: false,
        })
    }

    fn get(
        &self,
        req: RegistryLookupRequest,
    ) -> Result<RegistryLookupResponse<Self::Value>, RegistryError> {
        let _ = req;
        Ok(RegistryLookupResponse { entry: None })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, RegistryError> {
        Ok(ListIdsResponse { ids: vec![] })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, RegistryError> {
        Ok(LenResponse { count: 0 })
    }

    fn is_empty(&self, _req: EmptinessRequest) -> Result<EmptinessResponse, RegistryError> {
        Ok(EmptinessResponse { empty: true })
    }
}

impl AgentRegistry for NoopAgentRegistry {
    fn metadata(
        &self,
        req: AgentMetadataLookupRequest<'_>,
    ) -> Result<AgentMetadataLookupResponse, AgentError> {
        Err(AgentError::NotFound(req.id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_agent_registry_happy_list_ids_returns_empty() {
        assert_eq!(
            NoopAgentRegistry.list_ids(ListIdsRequest).unwrap().ids.len(),
            0
        );
    }

    #[test]
    fn test_noop_agent_registry_happy_len_returns_zero() {
        assert_eq!(NoopAgentRegistry.len(LenRequest).unwrap().count, 0);
    }

    #[test]
    fn test_noop_agent_registry_error_metadata_returns_not_found() {
        let result = NoopAgentRegistry.metadata(AgentMetadataLookupRequest { id: "any" });
        assert!(matches!(result, Err(AgentError::NotFound(_))));
    }

    #[test]
    fn test_noop_agent_registry_edge_get_returns_none() {
        assert!(NoopAgentRegistry
            .get(RegistryLookupRequest {
                id: "any".to_string()
            })
            .unwrap()
            .entry
            .is_none());
    }
}
