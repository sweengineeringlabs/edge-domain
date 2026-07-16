//! # edge-domain-base
//!
//! The `Request`/`Response` marker-trait contract shared by `edge-domain-handler` and
//! `edge-domain-service`.
//!
//! `Send + 'static` alone is not a contract — any type satisfies it. `Request` and `Response`
//! give `Handler`/`Service` implementors a real, checkable bound for "valid request" and
//! "valid response" instead of an unconstrained associated type. A shared crate (rather than
//! each of `domain-handler`/`domain-service` declaring its own local marker trait) means a type
//! crossing the `Service`→`Handler` bridge only ever needs to satisfy one `Request`/`Response`
//! trait, not two independently-declared mirrors.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::Request;
pub use api::RequestError;
pub use api::Response;
pub use api::ResponseError;
pub use api::ValidationRequest;
pub use api::ValidationResponse;

pub use saf::REQUEST_SVC;
pub use saf::REQUEST_SVC_FACTORY;
pub use saf::RESPONSE_SVC;
pub use saf::RESPONSE_SVC_FACTORY;
