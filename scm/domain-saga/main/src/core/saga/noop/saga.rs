use crate::api::saga::traits::Saga;
use crate::api::saga::types::{NoopSaga, NoopSagaCommand, NoopSagaEvent};

impl Saga for NoopSaga {
    type SagaId = u64;
    type Event = NoopSagaEvent;
    type Command = NoopSagaCommand;

    fn handle(&mut self, _event: &Self::Event) -> Vec<Self::Command> {
        vec![]
    }

    fn is_complete(&self) -> bool {
        self.complete
    }
}
