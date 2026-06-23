use edge_domain_projection::{DomainEvent, ProjectionError, ProjectionBootstrap, StdProjectionFactory, Projection};

#[derive(Clone)]
struct TickEvt;

impl DomainEvent for TickEvt {
    fn aggregate_id(&self) -> &str {
        "counter-1"
    }
}

#[test]
fn test_std_factory_in_memory_creates_projection_with_seed_happy() {
    let p = StdProjectionFactory::in_memory(0u32, |n: &mut u32, _e: &TickEvt| *n += 1);
    assert_eq!(*p.read_model(), 0);
}

#[test]
fn test_std_factory_try_drain_empty_slice_returns_empty_stream_error() {
    let mut p = StdProjectionFactory::in_memory(0u32, |n: &mut u32, _e: &TickEvt| *n += 1);
    let err = StdProjectionFactory::try_drain(&mut p, &[]).unwrap_err();
    assert_eq!(err, ProjectionError::EmptyStream);
}

#[test]
fn test_std_factory_std_factory_returns_copy_instance_edge() {
    let f = StdProjectionFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after move
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<StdProjectionFactory>());
}

#[test]
fn test_std_factory_try_drain_applies_all_events_happy() {
    let mut p = StdProjectionFactory::in_memory(0u32, |n: &mut u32, _e: &TickEvt| *n += 1);
    let count = StdProjectionFactory::try_drain(&mut p, &[TickEvt, TickEvt, TickEvt]).unwrap();
    assert_eq!(count, 3);
    assert_eq!(*p.read_model(), 3);
}

#[test]
fn test_std_factory_try_drain_single_event_returns_one_edge() {
    let mut p = StdProjectionFactory::in_memory(0u32, |n: &mut u32, _e: &TickEvt| *n += 1);
    let count = StdProjectionFactory::try_drain(&mut p, &[TickEvt]).unwrap();
    assert_eq!(count, 1);
}
