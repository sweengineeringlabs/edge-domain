//! `ClosedEventSource` — marker type for a never-emitting event source.

/// A closed event source that immediately signals unavailability on the first poll.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedEventSource;
