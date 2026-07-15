//! SAF facade tests — `DomainRuntime` trait.
#![cfg(all(feature = "event", feature = "command"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application::{CommandBus, Domain, DomainError, DomainRuntime, EventBus, EventPublisher};
use edge_application::{
    DirectCommandBusRequest, DirectCommandBusResponse, InProcessEventBusRequest,
    InProcessEventBusResponse, NoopEventBusRequest, NoopEventBusResponse,
    NoopEventPublisherRequest, NoopEventPublisherResponse,
};

/// Real, minimal delegating implementor — forwards to `Domain`'s own impl.
struct TestRuntime;

impl DomainRuntime for TestRuntime {
    fn direct_command_bus(
        &self,
        req: DirectCommandBusRequest,
    ) -> Result<DirectCommandBusResponse, DomainError> {
        Domain.direct_command_bus(req)
    }

    fn noop_event_publisher(
        &self,
        req: NoopEventPublisherRequest,
    ) -> Result<NoopEventPublisherResponse, DomainError> {
        Domain.noop_event_publisher(req)
    }

    fn in_process_event_bus(
        &self,
        req: InProcessEventBusRequest,
    ) -> Result<InProcessEventBusResponse, DomainError> {
        Domain.in_process_event_bus(req)
    }

    fn noop_event_bus(
        &self,
        req: NoopEventBusRequest,
    ) -> Result<NoopEventBusResponse, DomainError> {
        Domain.noop_event_bus(req)
    }
}

struct FailingRuntime;

impl DomainRuntime for FailingRuntime {
    fn direct_command_bus(
        &self,
        _req: DirectCommandBusRequest,
    ) -> Result<DirectCommandBusResponse, DomainError> {
        Err(DomainError::Unavailable("runtime offline".into()))
    }

    fn noop_event_publisher(
        &self,
        _req: NoopEventPublisherRequest,
    ) -> Result<NoopEventPublisherResponse, DomainError> {
        Err(DomainError::Unavailable("runtime offline".into()))
    }

    fn in_process_event_bus(
        &self,
        _req: InProcessEventBusRequest,
    ) -> Result<InProcessEventBusResponse, DomainError> {
        Err(DomainError::Unavailable("runtime offline".into()))
    }

    fn noop_event_bus(
        &self,
        _req: NoopEventBusRequest,
    ) -> Result<NoopEventBusResponse, DomainError> {
        Err(DomainError::Unavailable("runtime offline".into()))
    }
}

fn as_dyn(runtime: &impl DomainRuntime) -> &dyn DomainRuntime {
    runtime
}

/// @covers: DomainRuntime::direct_command_bus — success
#[test]
fn test_direct_command_bus_ok_runtime_returns_bus_happy() {
    let runtime = as_dyn(&TestRuntime);
    let resp = runtime.direct_command_bus(DirectCommandBusRequest).unwrap();
    let _bus: Arc<dyn CommandBus> = resp.bus;
}

/// @covers: DomainRuntime::direct_command_bus — failure propagates
#[test]
fn test_direct_command_bus_failing_runtime_returns_err_error() {
    let runtime = as_dyn(&FailingRuntime);
    assert!(runtime.direct_command_bus(DirectCommandBusRequest).is_err());
}

/// @covers: DomainRuntime::direct_command_bus — each call yields an independent bus
#[test]
fn test_direct_command_bus_repeated_calls_yield_independent_instances_edge() {
    let runtime = as_dyn(&TestRuntime);
    let a = runtime
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let b = runtime
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(!Arc::ptr_eq(&a, &b));
}

/// @covers: DomainRuntime::noop_event_publisher — success
#[test]
fn test_noop_event_publisher_ok_runtime_returns_publisher_happy() {
    let runtime = as_dyn(&TestRuntime);
    let resp = runtime
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap();
    let _publisher: Arc<dyn EventPublisher> = resp.publisher;
}

/// @covers: DomainRuntime::noop_event_publisher — failure propagates
#[test]
fn test_noop_event_publisher_failing_runtime_returns_err_error() {
    let runtime = as_dyn(&FailingRuntime);
    assert!(runtime
        .noop_event_publisher(NoopEventPublisherRequest)
        .is_err());
}

/// @covers: DomainRuntime::noop_event_publisher — each call yields an independent publisher
#[test]
fn test_noop_event_publisher_repeated_calls_yield_independent_instances_edge() {
    let runtime = as_dyn(&TestRuntime);
    let a = runtime
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap()
        .publisher;
    let b = runtime
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap()
        .publisher;
    assert!(!Arc::ptr_eq(&a, &b));
}

/// @covers: DomainRuntime::in_process_event_bus — success
#[test]
fn test_in_process_event_bus_ok_runtime_returns_bus_happy() {
    let runtime = as_dyn(&TestRuntime);
    let resp = runtime
        .in_process_event_bus(InProcessEventBusRequest {
            config: edge_application::EventBusConfig { capacity: 4 },
        })
        .unwrap();
    let _bus: Arc<dyn EventBus> = resp.bus;
}

/// @covers: DomainRuntime::in_process_event_bus — failure propagates
#[test]
fn test_in_process_event_bus_failing_runtime_returns_err_error() {
    let runtime = as_dyn(&FailingRuntime);
    assert!(runtime
        .in_process_event_bus(InProcessEventBusRequest {
            config: edge_application::EventBusConfig { capacity: 4 },
        })
        .is_err());
}

/// @covers: DomainRuntime::in_process_event_bus — minimum valid capacity is accepted
#[test]
fn test_in_process_event_bus_minimum_capacity_config_still_constructs_edge() {
    let runtime = as_dyn(&TestRuntime);
    let resp = runtime
        .in_process_event_bus(InProcessEventBusRequest {
            config: edge_application::EventBusConfig { capacity: 1 },
        })
        .unwrap();
    assert_eq!(Arc::strong_count(&resp.bus), 1);
}

/// @covers: DomainRuntime::noop_event_bus — success
#[test]
fn test_noop_event_bus_ok_runtime_returns_bus_happy() {
    let runtime = as_dyn(&TestRuntime);
    let resp = runtime.noop_event_bus(NoopEventBusRequest).unwrap();
    let _bus: Arc<dyn EventBus> = resp.bus;
}

/// @covers: DomainRuntime::noop_event_bus — failure propagates
#[test]
fn test_noop_event_bus_failing_runtime_returns_err_error() {
    let runtime = as_dyn(&FailingRuntime);
    assert!(runtime.noop_event_bus(NoopEventBusRequest).is_err());
}

/// @covers: DomainRuntime::noop_event_bus — each call yields an independent bus
#[test]
fn test_noop_event_bus_repeated_calls_yield_independent_instances_edge() {
    let runtime = as_dyn(&TestRuntime);
    let a = runtime.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    let b = runtime.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    assert!(!Arc::ptr_eq(&a, &b));
}

/// @covers: DomainRuntime — `Domain` is a real trait implementor
#[test]
fn test_domain_implements_domain_runtime_as_dyn_trait_happy() {
    let runtime: &dyn DomainRuntime = &Domain;
    let resp = runtime.direct_command_bus(DirectCommandBusRequest).unwrap();
    assert_eq!(Arc::strong_count(&resp.bus), 1);
}
