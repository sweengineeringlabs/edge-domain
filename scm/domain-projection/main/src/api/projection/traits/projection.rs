use edge_domain_event::DomainEvent;

use crate::api::projection::errors::ProjectionError;
use crate::api::projection::types::{ProjectionApplyRequest, ProjectionReadModelRequest, ProjectionReadModelResponse};

/// Consumes domain events and maintains a read model.
///
/// The read side of CQRS: apply events in stream order to build a
/// denormalised view optimised for queries.
pub trait Projection: Send + Sync {
    /// The event type this projection handles.
    type Event: DomainEvent;

    /// The read model this projection maintains.
    type ReadModel;

    /// Apply one event to update the read model in place.
    fn apply(&mut self, req: ProjectionApplyRequest<'_, Self::Event>) -> Result<(), ProjectionError>;

    /// Return the current read model state.
    fn read_model(
        &self,
        req: ProjectionReadModelRequest,
    ) -> Result<ProjectionReadModelResponse<'_, Self::ReadModel>, ProjectionError>;
}
