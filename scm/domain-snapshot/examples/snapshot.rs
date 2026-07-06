//! Basic `Snapshot` usage example.

use edge_domain_snapshot::{Snapshot, SnapshotAggregateIdRequest, SnapshotVersionRequest};

struct OrderSnap {
    id: String,
    version: u64,
}

impl Snapshot for OrderSnap {
    type AggregateId = String;
    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<
        edge_domain_snapshot::SnapshotAggregateIdResponse<'_, String>,
        edge_domain_snapshot::SnapshotError,
    > {
        Ok(edge_domain_snapshot::SnapshotAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
    fn version(
        &self,
        _req: SnapshotVersionRequest,
    ) -> Result<edge_domain_snapshot::SnapshotVersionResponse, edge_domain_snapshot::SnapshotError>
    {
        Ok(edge_domain_snapshot::SnapshotVersionResponse {
            version: self.version,
        })
    }
}

fn main() {
    let s = OrderSnap {
        id: "order-1".into(),
        version: 5,
    };
    match (
        s.version(SnapshotVersionRequest),
        s.aggregate_id(SnapshotAggregateIdRequest),
    ) {
        (Ok(v), Ok(id)) => println!("snapshot v{} for {}", v.version, id.aggregate_id),
        _ => eprintln!("failed to read snapshot"),
    }
}
