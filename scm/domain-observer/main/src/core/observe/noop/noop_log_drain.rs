use crate::api::LogDrain;
use crate::api::LogRecord;

pub(crate) struct NoopLogDrain;

impl NoopLogDrain {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LogDrain for NoopLogDrain {
    fn emit(&self, record: LogRecord) {
        let _ = record;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_noop_log_drain_happy() {
        let d = NoopLogDrain::new();
        assert_eq!(std::mem::size_of_val(&d), 0);
    }

    #[test]
    fn test_emit_info_record_discarded_error() {
        let d = NoopLogDrain::new();
        d.emit(LogRecord::new("INFO", "h", "msg"));
        assert_eq!(std::mem::size_of_val(&d), 0);
    }

    #[test]
    fn test_emit_multiple_records_no_accumulation_edge() {
        let d = NoopLogDrain::new();
        for i in 0..10 {
            d.emit(LogRecord::new("DEBUG", "h", &format!("msg {i}")));
        }
        assert_eq!(std::mem::size_of_val(&d), 0);
    }
}
