/// A no-op [`Saga`](crate::api::saga::traits::Saga) that never handles events.
///
/// Useful as a placeholder or test double where a concrete `Saga` type is
/// required but no actual processing should occur.
pub struct NoopSaga {
    pub(crate) complete: bool,
}

impl Default for NoopSaga {
    fn default() -> Self {
        Self { complete: false }
    }
}
