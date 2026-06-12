use std::fmt::Display;

use crate::api::saga::errors::SagaError;
use crate::api::saga::traits::{Saga, SagaRegistry};
use crate::api::saga::types::InMemorySagaRegistry;

impl<S> SagaRegistry<S> for InMemorySagaRegistry<S>
where
    S: Saga,
    S::SagaId: Display,
{
    fn register(&mut self, id: S::SagaId, saga: S) -> Result<(), SagaError> {
        if self.sagas.contains_key(&id) {
            return Err(SagaError::AlreadyRegistered(id.to_string()));
        }
        self.sagas.insert(id, saga);
        Ok(())
    }

    fn get(&self, id: &S::SagaId) -> Result<&S, SagaError> {
        self.sagas
            .get(id)
            .ok_or_else(|| SagaError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{Command, CommandError};
    use edge_domain_event::DomainEvent;
    use futures::future::BoxFuture;

    #[derive(Debug, Default)]
    struct InMemorySagaRegistryTestSaga {
        done: bool,
    }

    #[derive(Clone)]
    struct InMemorySagaRegistryTestSagaSignal;

    impl DomainEvent for InMemorySagaRegistryTestSagaSignal {
        fn aggregate_id(&self) -> &str {
            "saga-test"
        }
    }

    impl Command for InMemorySagaRegistryTestSagaSignal {
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async move { Ok(()) })
        }
    }

    impl Saga for InMemorySagaRegistryTestSaga {
        type SagaId = String;
        type Event = InMemorySagaRegistryTestSagaSignal;
        type Command = InMemorySagaRegistryTestSagaSignal;

        fn handle(&mut self, _event: &Self::Event) -> Vec<Self::Command> {
            self.done = true;
            vec![]
        }

        fn is_complete(&self) -> bool {
            self.done
        }
    }

    type TestRegistry = InMemorySagaRegistry<InMemorySagaRegistryTestSaga>;

    #[test]
    fn test_register_then_get_returns_ok() {
        let mut reg = TestRegistry::new();
        reg.register("s1".to_string(), InMemorySagaRegistryTestSaga::default())
            .ok();
        assert!(reg.get(&"s1".to_string()).is_ok());
    }

    #[test]
    fn test_register_duplicate_returns_already_registered() {
        let mut reg = TestRegistry::new();
        reg.register("s1".to_string(), InMemorySagaRegistryTestSaga::default())
            .ok();
        let err = reg
            .register("s1".to_string(), InMemorySagaRegistryTestSaga::default())
            .unwrap_err();
        assert_eq!(err, SagaError::AlreadyRegistered("s1".to_string()));
    }

    #[test]
    fn test_get_unknown_id_returns_not_found() {
        let reg = TestRegistry::new();
        let err = reg.get(&"ghost".to_string()).unwrap_err();
        assert_eq!(err, SagaError::NotFound("ghost".to_string()));
    }
}
