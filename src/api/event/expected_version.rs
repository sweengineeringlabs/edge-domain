//! `ExpectedVersion` — optimistic concurrency control for event streams.

/// Controls optimistic concurrency when appending to an event stream.
///
/// Pass to [`EventStore::append`](crate::EventStore::append) to guard against
/// concurrent writes to the same aggregate stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedVersion {
    /// Do not check the current stream version — always append.
    Any,

    /// The stream must not exist yet (current version is 0).
    ///
    /// Use when creating a new aggregate to prevent accidental duplicate creation.
    NoStream,

    /// The stream must be at exactly this version.
    ///
    /// Use when updating an existing aggregate to detect concurrent modifications.
    Exact(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_version_equality() {
        assert_eq!(ExpectedVersion::Exact(3), ExpectedVersion::Exact(3));
        assert_ne!(ExpectedVersion::Exact(3), ExpectedVersion::Exact(4));
    }

    #[test]
    fn test_any_is_not_no_stream() {
        assert_ne!(ExpectedVersion::Any, ExpectedVersion::NoStream);
    }
}
