//! [`Response`] — re-export of the shared response contract
//! [`Service::Response`](crate::api::Service::Response) is bound against.
//!
//! `Service` is generic over its response payload — there is no single concrete response
//! struct to file here per-implementor. What every implementor's `Self::Response` actually
//! resolves to is this shared contract, so that's what this file names.

pub use edge_application_base::Response;
