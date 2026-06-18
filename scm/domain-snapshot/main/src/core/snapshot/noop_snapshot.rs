//! `Snapshot` impl for [`NoopSnapshot`].

use crate::api::Snapshot;
use crate::api::NoopSnapshot;

impl Snapshot for NoopSnapshot {
    type AggregateId = String;

    fn aggregate_id(&self) -> &String {
        &self.id
    }

    fn version(&self) -> u64 {
        self.version
    }
}
