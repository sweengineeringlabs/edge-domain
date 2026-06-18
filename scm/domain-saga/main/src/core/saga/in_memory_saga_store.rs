use std::fmt::Display;

use crate::api::SagaError;
use crate::api::{Saga, SagaStore};
use crate::api::InMemorySagaStore;

impl<S> SagaStore for InMemorySagaStore<S>
where
    S: Saga,
    S::SagaId: Display,
{
    type SagaInstance = S;

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
    struct InMemorySagaStoreTestSaga {
        done: bool,
    }

    #[derive(Clone)]
    struct InMemorySagaStoreTestSagaSignal;

    impl DomainEvent for InMemorySagaStoreTestSagaSignal {
        fn aggregate_id(&self) -> &str {
            "saga-test"
        }
    }

    impl Command for InMemorySagaStoreTestSagaSignal {
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async move { Ok(()) })
        }
    }

    impl Saga for InMemorySagaStoreTestSaga {
        type SagaId = String;
        type Event = InMemorySagaStoreTestSagaSignal;
        type Command = InMemorySagaStoreTestSagaSignal;

        fn handle(&mut self, _event: &Self::Event) -> Vec<Self::Command> {
            self.done = true;
            vec![]
        }

        fn is_complete(&self) -> bool {
            self.done
        }
    }

    type TestStore = InMemorySagaStore<InMemorySagaStoreTestSaga>;

    #[test]
    fn test_register_then_get_returns_ok() {
        let mut store = TestStore::new();
        store.register("s1".to_string(), InMemorySagaStoreTestSaga::default())
            .ok();
        assert!(store.get(&"s1".to_string()).is_ok());
    }

    #[test]
    fn test_register_duplicate_returns_already_registered() {
        let mut store = TestStore::new();
        store.register("s1".to_string(), InMemorySagaStoreTestSaga::default())
            .ok();
        let err = store
            .register("s1".to_string(), InMemorySagaStoreTestSaga::default())
            .unwrap_err();
        assert_eq!(err, SagaError::AlreadyRegistered("s1".to_string()));
    }

    #[test]
    fn test_get_unknown_id_returns_not_found() {
        let store = TestStore::new();
        let err = store.get(&"ghost".to_string()).unwrap_err();
        assert_eq!(err, SagaError::NotFound("ghost".to_string()));
    }
}
