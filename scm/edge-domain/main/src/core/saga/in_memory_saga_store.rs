//! In-memory saga store — stores sagas in a `HashMap` keyed by `SagaId`.
//!
//! A reference [`SagaStore`] for development and testing.  State lives in
//! process memory and is lost when the process stops.

use std::collections::HashMap;
use std::fmt::Display;

use crate::api::SagaError;
use crate::api::Saga;
use crate::api::SagaStore;

pub(crate) struct InMemorySagaStore<S: Saga> {
    sagas: HashMap<S::SagaId, S>,
}

impl<S: Saga> InMemorySagaStore<S> {
    pub(crate) fn new() -> Self {
        Self {
            sagas: HashMap::new(),
        }
    }
}

// impl SagaStore for InMemorySagaStore
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
    use crate::api::Command;
    use crate::api::CommandError;
    use crate::api::DomainEvent;
    use futures::future::BoxFuture;

    #[derive(Clone)]
    struct InMemorySagaStoreTestSignal;
    impl DomainEvent for InMemorySagaStoreTestSignal {
        fn aggregate_id(&self) -> &str {
            "saga-test"
        }
    }
    impl Command for InMemorySagaStoreTestSignal {
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async move { Ok(()) })
        }
    }

    #[derive(Debug, Default)]
    struct InMemorySagaStoreTestSaga {
        done: bool,
    }
    impl Saga for InMemorySagaStoreTestSaga {
        type SagaId = String;
        type Event = InMemorySagaStoreTestSignal;
        type Command = InMemorySagaStoreTestSignal;
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
    fn test_new_creates_empty_store() {
        let store = TestStore::new();
        assert!(store.get(&"missing".to_string()).is_err());
    }

    #[test]
    fn test_register_then_get_returns_saga() {
        let mut store = TestStore::new();
        store.register("s1".to_string(), InMemorySagaStoreTestSaga::default())
            .unwrap();
        assert!(store.get(&"s1".to_string()).is_ok());
    }

    #[test]
    fn test_register_duplicate_id_returns_already_registered() {
        let mut store = TestStore::new();
        store.register("s1".to_string(), InMemorySagaStoreTestSaga::default())
            .unwrap();
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
