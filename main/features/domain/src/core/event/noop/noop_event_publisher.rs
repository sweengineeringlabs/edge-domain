//! Noop event publisher — SEA structural counterpart.
//!
//! The real implementation lives in [`crate::core::event::noop_event_publisher`].

/// Marker type for the noop event publisher (rule 89 compliance).
///
/// Implementation is provided by [`crate::core::event::noop_event_publisher::NoopEventPublisher`].
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "SEA core/ structural anchor — constructed only in tests"
    )
)]
pub(crate) struct NoopEventPublisher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_event_publisher_noop_marker_is_constructible() {
        let _ = NoopEventPublisher;
    }
}
