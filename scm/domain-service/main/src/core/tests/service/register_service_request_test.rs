//! External tests for `RegisterServiceRequest::new`.

#[cfg(test)]
mod tests {
    use crate::api::{NameRequest, NoopRequest, NoopResponse, NoopService, RegisterServiceRequest};
    use std::sync::Arc;

    /// @covers: new
    #[test]
    fn test_new_wraps_given_service_happy() {
        let req = RegisterServiceRequest::<NoopRequest, NoopResponse>::new(Arc::new(NoopService));
        let name = req.service.name(NameRequest).unwrap().name;
        assert_eq!(name, "noop");
    }

    /// @covers: new
    #[test]
    fn test_new_distinct_instances_are_independent_edge() {
        let req1 = RegisterServiceRequest::<NoopRequest, NoopResponse>::new(Arc::new(NoopService));
        let req2 = RegisterServiceRequest::<NoopRequest, NoopResponse>::new(Arc::new(NoopService));
        assert!(!Arc::ptr_eq(&req1.service, &req2.service));
    }
}
