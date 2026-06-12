//! In-memory saga registry — stores sagas in a `HashMap` keyed by `SagaId`.
//!
//! A reference [`SagaRegistry`] for development and testing.  State lives in
//! process memory and is lost when the process stops.

use std::collections::HashMap;
use std::fmt::Display;

use crate::api::saga::errors::SagaError;
use crate::api::saga::traits::Saga;
use crate::api::saga::traits::SagaRegistry;

pub(crate) struct InMemorySagaRegistry<S: Saga> {
    sagas: HashMap<S::SagaId, S>,
}

impl<S: Saga> InMemorySagaRegistry<S> {
    pub(crate) fn new() -> Self {
        Self {
            sagas: HashMap::new(),
        }
    }
}

// impl SagaRegistry for InMemorySagaRegistry
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
    use crate::api::command::Command;
    use crate::api::command::CommandError;
    use crate::api::event::DomainEvent;
    use futures::future::BoxFuture;

    #[derive(Clone)]
    struct InMemorySagaRegistryTestSignal;
    impl DomainEvent for InMemorySagaRegistryTestSignal {
        fn aggregate_id(&self) -> &str {
            "saga-test"
        }
    }
    impl Command for InMemorySagaRegistryTestSignal {
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async move { Ok(()) })
        }
    }

    #[derive(Debug, Default)]
    struct InMemorySagaRegistryTestSaga {
        done: bool,
    }
    impl Saga for InMemorySagaRegistryTestSaga {
        type SagaId = String;
        type Event = InMemorySagaRegistryTestSignal;
        type Command = InMemorySagaRegistryTestSignal;
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
    fn test_new_creates_empty_registry() {
        let reg = TestRegistry::new();
        assert!(reg.get(&"missing".to_string()).is_err());
    }

    #[test]
    fn test_register_then_get_returns_saga() {
        let mut reg = TestRegistry::new();
        reg.register("s1".to_string(), InMemorySagaRegistryTestSaga::default())
            .unwrap();
        assert!(reg.get(&"s1".to_string()).is_ok());
    }

    #[test]
    fn test_register_duplicate_id_returns_already_registered() {
        let mut reg = TestRegistry::new();
        reg.register("s1".to_string(), InMemorySagaRegistryTestSaga::default())
            .unwrap();
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
