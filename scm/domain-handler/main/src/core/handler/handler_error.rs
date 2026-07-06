//! `From<ServiceError>` impl and helper constructors for [`HandlerError`].

use crate::api::HandlerError;

impl From<edge_domain_service::ServiceError> for HandlerError {
    fn from(e: edge_domain_service::ServiceError) -> Self {
        match e {
            edge_domain_service::ServiceError::InvalidRequest(msg) => {
                HandlerError::InvalidRequest(msg)
            }
            edge_domain_service::ServiceError::RuleViolation(msg) => {
                HandlerError::FailedPrecondition(msg)
            }
            edge_domain_service::ServiceError::NotFound(msg) => HandlerError::NotFound(msg),
            edge_domain_service::ServiceError::Unavailable(msg) => {
                HandlerError::ExecutionFailed(msg)
            }
            edge_domain_service::ServiceError::Internal(msg) => HandlerError::ExecutionFailed(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_service::ServiceError;

    #[test]
    fn test_from_invalid_request_maps_correctly_happy() {
        let e: HandlerError = ServiceError::InvalidRequest("bad".into()).into();
        assert_eq!(e, HandlerError::InvalidRequest("bad".into()));
    }

    #[test]
    fn test_from_not_found_maps_correctly_happy() {
        let e: HandlerError = ServiceError::NotFound("gone".into()).into();
        assert_eq!(e, HandlerError::NotFound("gone".into()));
    }

    #[test]
    fn test_from_internal_maps_to_execution_failed_edge() {
        let e: HandlerError = ServiceError::Internal("boom".into()).into();
        assert_eq!(e, HandlerError::ExecutionFailed("boom".into()));
    }
}
