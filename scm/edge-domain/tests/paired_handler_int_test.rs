//! Integration tests for `Domain::paired` — shared backend construction.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain::{Domain, HandlerContext, HandlerError, Repository};
use edge_domain_observe::StdObserveFactory;
use edge_domain_security::SecurityContext;

struct WriteHandler {
    repo: Arc<dyn Repository<Entity = String, Id = String>>,
}

struct ReadHandler {
    repo: Arc<dyn Repository<Entity = String, Id = String>>,
}

/// @covers: Domain::paired — both handlers share one Arc<Backend>
#[tokio::test]
async fn test_paired_write_is_visible_to_read() {
    let (writer, reader) = Domain::paired(
        Domain::new_in_memory_repository::<String, String>(),
        |repo| WriteHandler { repo },
        |repo| ReadHandler { repo },
    );

    writer
        .repo
        .save("k".to_string(), "v".to_string())
        .await
        .unwrap();
    let found = reader.repo.find(&"k".to_string()).await.unwrap();
    assert_eq!(found, Some("v".to_string()));
}

/// @covers: Domain::paired — independent from_config() calls use different backends
#[tokio::test]
async fn test_independent_backends_do_not_share_state() {
    let repo_a = Domain::new_in_memory_repository::<String, String>();
    let repo_b = Domain::new_in_memory_repository::<String, String>();

    repo_a.save("k".to_string(), "v".to_string()).await.unwrap();

    let found = repo_b.find(&"k".to_string()).await.unwrap();
    assert_eq!(found, None, "separate instances must not share state");
}

/// @covers: Domain::paired — different handler types allowed
#[test]
fn test_paired_accepts_heterogeneous_handler_types() {
    struct CmdHandler {
        _repo: Arc<dyn Repository<Entity = u32, Id = u32>>,
    }
    struct QryHandler {
        _repo: Arc<dyn Repository<Entity = u32, Id = u32>>,
    }

    let (cmd, qry) = Domain::paired(
        Domain::new_in_memory_repository::<u32, u32>(),
        |repo| CmdHandler { _repo: repo },
        |repo| QryHandler { _repo: repo },
    );
    drop((cmd, qry));
}

// ── EchoHandler via Domain factory (backward compat) ─────────────────────────

/// @covers: Domain::echo_handler delegates to Dispatch::echo_handler
#[tokio::test]
async fn test_domain_echo_handler_returns_input_unchanged() {
    let h = Domain::echo_handler::<String>("e", "/e");
    let security = SecurityContext::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, bus.as_ref(), observer.as_ref());
    let result = h.execute("hello".to_string(), ctx).await;
    assert_eq!(result.unwrap(), "hello");
}

/// @covers: Domain::new_handler_registry delegates to Dispatch::new_handler_registry
#[test]
fn test_domain_new_handler_registry_is_empty() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}

/// @covers: HandlerError::internal
#[test]
fn test_handler_error_internal_helper() {
    let e = HandlerError::internal("db timeout");
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
}
