//! `ClosedEventSource` — marker type for a never-emitting event source.

/// A closed event source that immediately signals unavailability on the first poll.
pub struct ClosedEventSource;
