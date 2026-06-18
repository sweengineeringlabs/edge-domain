//! `CommandBusFactory` and `CommandFactory` impls for [`StdCommandBusFactory`].

use crate::api::CommandBusFactory;
use crate::api::CommandFactory;
use crate::api::StdCommandBusFactory;

impl CommandBusFactory for StdCommandBusFactory {}

impl CommandFactory for StdCommandBusFactory {}
