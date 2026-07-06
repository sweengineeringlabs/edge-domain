//! Scenario coverage for the `ToolResultBatch` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::cell::RefCell;

use edge_llm_complete::{CompleteError, Message, ToolRecordRequest, ToolResultBatch};

/// A minimal hand-written `ToolResultBatch` test double — records into a `Vec`,
/// rejecting out-of-range indices as an error (unlike the crate's own
/// `ToolCallBatch`, which silently ignores them) so both outcomes are exercised.
struct VecResultBatch {
    slots: RefCell<Vec<Option<Message>>>,
}

impl VecResultBatch {
    fn new(len: usize) -> Self {
        Self {
            slots: RefCell::new((0..len).map(|_| None).collect()),
        }
    }
}

impl ToolResultBatch for VecResultBatch {
    fn record(&self, req: ToolRecordRequest) -> Result<(), CompleteError> {
        let mut slots = self.slots.borrow_mut();
        match slots.get_mut(req.index) {
            Some(slot) => {
                *slot = Some(*req.message);
                Ok(())
            }
            None => Err(CompleteError::InvalidRequest(format!(
                "index {} out of range",
                req.index
            ))),
        }
    }
}

/// @covers: ToolResultBatch::record — happy path: records at a valid index
#[test]
fn test_record_valid_index_stores_message_happy() {
    let batch = VecResultBatch::new(2);
    batch
        .record(ToolRecordRequest {
            index: 0,
            message: Box::new(Message::tool("output", "call-1")),
        })
        .expect("record ok");
    assert_eq!(
        batch.slots.borrow()[0]
            .as_ref()
            .map(|m| m.tool_call_id.clone()),
        Some(Some("call-1".to_string()))
    );
}

/// @covers: ToolResultBatch::record — error: out-of-range index is rejected
#[test]
fn test_record_out_of_range_index_returns_error_error() {
    let batch = VecResultBatch::new(1);
    let result = batch.record(ToolRecordRequest {
        index: 9,
        message: Box::new(Message::tool("output", "call-1")),
    });
    assert!(matches!(result, Err(CompleteError::InvalidRequest(_))));
}

/// @covers: ToolResultBatch::record — edge: recording at index zero of a single-slot batch
#[test]
fn test_record_zero_index_single_slot_batch_edge() {
    let batch = VecResultBatch::new(1);
    batch
        .record(ToolRecordRequest {
            index: 0,
            message: Box::new(Message::tool("only", "call-0")),
        })
        .expect("record ok");
    assert_eq!(batch.slots.borrow().len(), 1);
    assert!(batch.slots.borrow()[0].is_some());
}
