//! `ServiceRegistryBootstrap` implementations (crate-private).

use crate::api::ServiceRegistryBootstrap;
use crate::api::StdServiceRegistryFactory;

impl ServiceRegistryBootstrap for StdServiceRegistryFactory {}
impl ServiceRegistryBootstrap for () {}
