//! `ServiceRegistryBootstrap` trait impls for the canonical factory types.

use crate::api::ServiceRegistryBootstrap;
use crate::api::StdServiceRegistryFactory;

impl ServiceRegistryBootstrap for StdServiceRegistryFactory {}

impl ServiceRegistryBootstrap for () {}
