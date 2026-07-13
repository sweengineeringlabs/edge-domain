//! Domain theme — cross-cutting domain types and the extension hook contract.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::DomainError;
pub use traits::{
    Domain, DomainExtension, DomainExtensionHealthRequest, DomainRuntime, MemoryOutboundRegistry,
    NoopDomainExtension, OutboundRegistry,
};
#[cfg(feature = "command")]
pub use types::{DirectCommandBusRequest, DirectCommandBusResponse};
#[cfg(feature = "event")]
pub use types::{
    InProcessEventBusRequest, InProcessEventBusResponse, NoopEventBusRequest, NoopEventBusResponse,
    NoopEventPublisherRequest, NoopEventPublisherResponse,
};
pub use types::{
    OutboundDeregisterRequest, OutboundDeregisterResponse, OutboundGetRequest, OutboundGetResponse,
    OutboundIsEmptyRequest, OutboundIsEmptyResponse, OutboundLenRequest, OutboundLenResponse,
    OutboundNamesRequest, OutboundNamesResponse, OutboundRegisterRequest, OutboundRegisterResponse,
};
