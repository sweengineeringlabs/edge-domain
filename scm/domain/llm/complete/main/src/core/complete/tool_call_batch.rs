//! `ToolResultBatch` impl for [`DefaultToolCallBatch`] — `Ctx` for the per-turn tool-call
//! `ParallelStep` fan-out.

use std::sync::{Arc, Mutex};

use crate::api::{CompleteError, Message, ToolRecordRequest, ToolResultBatch};

/// `Ctx` for the per-turn tool-call `ParallelStep` fan-out (see `ToolCallStep`).
///
/// `ParallelStep` clones `Ctx` per branch and never merges branches back into the
/// caller's context. Cloning shares the same underlying `results` storage (`Arc`
/// clone), so each branch writes into its own reserved slot and every write is
/// visible on the original context after the parallel step completes.
#[derive(Clone)]
pub(super) struct DefaultToolCallBatch {
    results: Arc<Mutex<Vec<Option<Message>>>>,
}

impl DefaultToolCallBatch {
    /// Pre-size storage for `len` pending tool-call results.
    pub(super) fn new(len: usize) -> Self {
        Self {
            results: Arc::new(Mutex::new(vec![None; len])),
        }
    }

    /// Drain all recorded messages in original branch order, dropping any slot a
    /// branch never reached.
    pub(super) fn into_messages(self) -> Vec<Message> {
        let results = match Arc::try_unwrap(self.results) {
            Ok(mutex) => mutex.into_inner().unwrap_or_else(|e| e.into_inner()),
            Err(shared) => shared
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .clone(),
        };
        results.into_iter().flatten().collect()
    }
}

impl ToolResultBatch for DefaultToolCallBatch {
    fn record(&self, req: ToolRecordRequest) -> Result<(), CompleteError> {
        let mut results = self
            .results
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(slot) = results.get_mut(req.index) {
            *slot = Some(*req.message);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::MessageContent;

    fn msg(content: &str) -> Box<Message> {
        Box::new(Message::assistant(content))
    }

    /// @covers: new
    #[test]
    fn test_new_creates_none_filled_slots_of_given_length_happy() {
        let batch = DefaultToolCallBatch::new(3);
        assert_eq!(batch.into_messages().len(), 0);
    }

    /// @covers: record
    #[test]
    fn test_record_stores_message_at_index_happy() {
        let batch = DefaultToolCallBatch::new(2);
        batch
            .record(ToolRecordRequest {
                index: 0,
                message: msg("a"),
            })
            .expect("ok");
        batch
            .record(ToolRecordRequest {
                index: 1,
                message: msg("b"),
            })
            .expect("ok");
        assert_eq!(batch.into_messages().len(), 2);
    }

    /// @covers: record
    #[test]
    fn test_record_overwrites_existing_slot_edge() {
        let batch = DefaultToolCallBatch::new(1);
        batch
            .record(ToolRecordRequest {
                index: 0,
                message: msg("first"),
            })
            .expect("ok");
        batch
            .record(ToolRecordRequest {
                index: 0,
                message: msg("second"),
            })
            .expect("ok");
        let messages = batch.into_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages[0].content,
            MessageContent::Text("second".to_string())
        );
    }

    /// @covers: record
    #[test]
    fn test_record_out_of_range_index_is_ignored_error() {
        let batch = DefaultToolCallBatch::new(1);
        batch
            .record(ToolRecordRequest {
                index: 5,
                message: msg("ignored"),
            })
            .expect("ok");
        assert_eq!(batch.into_messages().len(), 0);
    }

    /// @covers: into_messages
    #[test]
    fn test_into_messages_returns_in_index_order_happy() {
        let batch = DefaultToolCallBatch::new(3);
        batch
            .record(ToolRecordRequest {
                index: 2,
                message: msg("third"),
            })
            .expect("ok");
        batch
            .record(ToolRecordRequest {
                index: 0,
                message: msg("first"),
            })
            .expect("ok");
        batch
            .record(ToolRecordRequest {
                index: 1,
                message: msg("second"),
            })
            .expect("ok");
        let messages = batch.into_messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(
            messages[0].content,
            MessageContent::Text("first".to_string())
        );
        assert_eq!(
            messages[1].content,
            MessageContent::Text("second".to_string())
        );
        assert_eq!(
            messages[2].content,
            MessageContent::Text("third".to_string())
        );
    }

    /// @covers: into_messages
    #[test]
    fn test_into_messages_skips_unset_slots_edge() {
        let batch = DefaultToolCallBatch::new(3);
        batch
            .record(ToolRecordRequest {
                index: 1,
                message: msg("only one set"),
            })
            .expect("ok");
        assert_eq!(batch.into_messages().len(), 1);
    }
}
