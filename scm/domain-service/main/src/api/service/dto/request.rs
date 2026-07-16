//! [`Request`] — re-export of the shared request contract
//! [`Service::Request`](crate::api::Service::Request) is bound against.
//!
//! `Service` is generic over its request payload — there is no single concrete request struct
//! to file here per-implementor. What every implementor's `Self::Request` actually resolves to
//! is this shared contract, so that's what this file names.

pub use edge_application_base::Request;
