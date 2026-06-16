//! `ServiceRegistryFactory` trait impls for the canonical factory types.

use crate::api::service::traits::service_registry_factory::ServiceRegistryFactory;
use crate::api::service::types::StdServiceRegistryFactory;

impl ServiceRegistryFactory for StdServiceRegistryFactory {}

impl ServiceRegistryFactory for () {}
