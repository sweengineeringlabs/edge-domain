//! API-layer type for the direct (in-process) query bus.

/// Marker type describing a `QueryBus` that dispatches queries inline,
/// calling `query.execute()` directly in the same task with no queuing.
///
/// The concrete implementation lives in `core::query::direct_query_bus`.
#[allow(dead_code)]
pub struct DirectQueryBus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_query_bus_is_constructible() {
        let _: DirectQueryBus = DirectQueryBus;
    }
}
