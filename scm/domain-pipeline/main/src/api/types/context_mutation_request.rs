//! [`ContextMutationRequest`] — shared request wrapping a mutable context reference.

/// Wraps a mutable reference to the shared execution context.
///
/// Shared by [`Pipeline::run`](crate::Pipeline::run) and [`Step::execute`](crate::Step::execute) —
/// both operate by mutating `Ctx` in place.
pub struct ContextMutationRequest<'a, Ctx> {
    /// Mutable reference to the context being threaded through execution.
    pub ctx: &'a mut Ctx,
}
