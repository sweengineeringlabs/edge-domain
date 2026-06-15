//! `CommandBusFactory` and `CommandFactory` impls for [`StdCommandBusFactory`].

use crate::api::command::traits::CommandBusFactory;
use crate::api::command::traits::CommandFactory;
use crate::api::command::types::StdCommandBusFactory;

impl CommandBusFactory for StdCommandBusFactory {}

impl CommandFactory for StdCommandBusFactory {}
