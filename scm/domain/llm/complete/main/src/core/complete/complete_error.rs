//! Behaviour for [`CompleteError`].

use std::time::Duration;

use crate::api::CompleteError;

impl CompleteError {
    /// Returns `true` if the operation is safe to retry.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CompleteError::NetworkError(_)
                | CompleteError::RateLimited { .. }
                | CompleteError::Timeout(_)
        )
    }

    /// Returns the suggested retry delay, if the error carries one.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            CompleteError::RateLimited { retry_after_ms } => {
                retry_after_ms.map(Self::duration_from_millis)
            }
            _ => None,
        }
    }

    /// Convert a millisecond count into a [`Duration`].
    fn duration_from_millis(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_retryable
    #[test]
    fn test_is_retryable_true_for_network_error() {
        let err = CompleteError::NetworkError("down".to_string());
        assert!(err.is_retryable());
    }

    /// @covers: is_retryable
    #[test]
    fn test_is_retryable_false_for_invalid_request() {
        let err = CompleteError::InvalidRequest("bad".to_string());
        assert!(!err.is_retryable());
    }

    /// @covers: retry_after
    #[test]
    fn test_retry_after_returns_duration_for_rate_limited() {
        let err = CompleteError::RateLimited {
            retry_after_ms: Some(500),
        };
        assert_eq!(err.retry_after(), Some(Duration::from_millis(500)));
    }

    /// @covers: duration_from_millis
    #[test]
    fn test_duration_from_millis_converts_correctly() {
        assert_eq!(
            CompleteError::duration_from_millis(1_000),
            Duration::from_secs(1)
        );
    }
}
