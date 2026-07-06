use std::sync::Arc;

use crate::api::complete::traits::{Completer, ToolOps};

/// Reference [`ToolCallLoop`](crate::api::complete::traits::ToolCallLoop) implementation,
/// composing one [`Completer`] and one [`ToolOps`].
///
/// Construction and the trait impl live in `core::complete::bounded_tool_call_loop` — api/
/// is a declaration layer only.
pub struct BoundedToolCallLoop {
    pub(crate) completer: Arc<dyn Completer>,
    pub(crate) tool_ops: Arc<dyn ToolOps>,
}
