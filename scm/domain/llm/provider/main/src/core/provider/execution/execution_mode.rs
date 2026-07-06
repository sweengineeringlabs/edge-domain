//! Behaviour for [`ExecutionMode`].

use crate::api::ExecutionMode;

impl ExecutionMode {
    /// Check if this mode supports streaming
    pub fn is_streaming(&self) -> bool {
        matches!(self, ExecutionMode::Streaming)
    }

    /// Check if this mode is asynchronous
    pub fn is_async(&self) -> bool {
        !self.is_long_running()
    }

    /// Check if this mode requires out-of-band polling or callbacks.
    fn is_long_running(&self) -> bool {
        matches!(self, ExecutionMode::LongRunning)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_streaming
    #[test]
    fn test_is_streaming_true_for_streaming_variant() {
        assert!(ExecutionMode::Streaming.is_streaming());
    }

    /// @covers: is_streaming
    #[test]
    fn test_is_streaming_false_for_async_variant() {
        assert!(!ExecutionMode::Async.is_streaming());
    }

    /// @covers: is_async
    #[test]
    fn test_is_async_false_for_long_running() {
        assert!(!ExecutionMode::LongRunning.is_async());
    }

    /// @covers: is_long_running
    #[test]
    fn test_is_long_running_true_for_long_running_variant() {
        assert!(ExecutionMode::LongRunning.is_long_running());
    }
}
