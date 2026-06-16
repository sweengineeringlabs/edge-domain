//! `Snapshot` impl for [`NoopSnapshot`].

use crate::api::snapshot::traits::Snapshot;
use crate::api::snapshot::types::NoopSnapshot;

impl Snapshot for NoopSnapshot {
    type AggregateId = String;

    fn aggregate_id(&self) -> &String {
        &self.id
    }

    fn version(&self) -> u64 {
        self.version
    }
}
