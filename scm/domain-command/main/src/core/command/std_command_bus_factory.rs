//! `CommandBusBootstrap` and `CommandBootstrap` impls for [`StdCommandBusFactory`].

use crate::api::CommandBusBootstrap;
use crate::api::CommandBootstrap;
use crate::api::StdCommandBusFactory;

impl CommandBusBootstrap for StdCommandBusFactory {}

impl CommandBootstrap for StdCommandBusFactory {}
