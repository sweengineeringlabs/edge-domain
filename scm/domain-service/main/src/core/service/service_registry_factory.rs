//! `ServiceRegistryFactory` trait impls for the canonical factory types.

use crate::api::ServiceRegistryFactory;
use crate::api::StdServiceRegistryFactory;

impl ServiceRegistryFactory for StdServiceRegistryFactory {}

impl ServiceRegistryFactory for () {}
