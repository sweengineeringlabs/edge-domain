//! SAF facade tests — `CommandBus` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{Command, CommandBus, CommandDispatchRequest, CommandExecutionRequest, HandlerError};
use futures::executor::block_on;

struct OkCmd;
impl Command for OkCmd {
    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    }
}

struct EchoBus;
impl CommandBus for EchoBus {
    fn dispatch(
        &self,
        req: CommandDispatchRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>> {
        Box::pin(async move { req.command.execute(CommandExecutionRequest).await })
    }
}

struct FailingBus;
impl CommandBus for FailingBus {
    fn dispatch(
        &self,
        _req: CommandDispatchRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>> {
        Box::pin(async { Err(HandlerError::ExecutionFailed("bus down".into())) })
    }
}

/// @covers: CommandBus::dispatch — success
#[test]
fn test_dispatch_ok_bus_returns_ok_happy() {
    let result = block_on(EchoBus.dispatch(CommandDispatchRequest {
        command: Box::new(OkCmd),
    }));
    assert_eq!(result, Ok(()));
}

/// @covers: CommandBus::dispatch — failure propagates
#[test]
fn test_dispatch_failing_bus_returns_err_error() {
    let result = block_on(FailingBus.dispatch(CommandDispatchRequest {
        command: Box::new(OkCmd),
    }));
    assert!(result.is_err());
}

/// @covers: CommandBus::dispatch — repeated dispatch is independent
#[test]
fn test_dispatch_repeated_calls_are_independent_edge() {
    let bus = EchoBus;
    assert_eq!(
        block_on(bus.dispatch(CommandDispatchRequest { command: Box::new(OkCmd) })),
        Ok(())
    );
    assert_eq!(
        block_on(bus.dispatch(CommandDispatchRequest { command: Box::new(OkCmd) })),
        Ok(())
    );
}

/// @covers: CommandBus::wrap — wraps an already type-erased real `CommandBus`
#[test]
fn test_wrap_erased_reference_dispatches_happy() {
    let bus = edge_domain_command::DirectCommandBus;
    let adapter = EchoBus::wrap(&bus as &dyn edge_domain_command::CommandBus);
    let result = block_on(adapter.dispatch(CommandDispatchRequest {
        command: Box::new(OkCmd),
    }));
    assert_eq!(result, Ok(()));
}

/// @covers: CommandBus::wrap — wrapped erased bus propagates real errors
#[test]
fn test_wrap_erased_reference_propagates_errors_error() {
    let bus = edge_domain_command::DirectCommandBus;
    let adapter = EchoBus::wrap(&bus as &dyn edge_domain_command::CommandBus);
    struct DenyCmd;
    impl Command for DenyCmd {
        fn execute(
            &self,
            _req: CommandExecutionRequest,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>>
        {
            Box::pin(async { Err(HandlerError::ExecutionFailed("denied".into())) })
        }
    }
    let result = block_on(adapter.dispatch(CommandDispatchRequest {
        command: Box::new(DenyCmd),
    }));
    assert!(result.is_err());
}

/// @covers: CommandBus::wrap — adapter reusable across multiple dispatches
#[test]
fn test_wrap_adapter_reusable_edge() {
    let bus = edge_domain_command::DirectCommandBus;
    let adapter = EchoBus::wrap(&bus as &dyn edge_domain_command::CommandBus);
    assert_eq!(
        block_on(adapter.dispatch(CommandDispatchRequest { command: Box::new(OkCmd) })),
        Ok(())
    );
    assert_eq!(
        block_on(adapter.dispatch(CommandDispatchRequest { command: Box::new(OkCmd) })),
        Ok(())
    );
}
