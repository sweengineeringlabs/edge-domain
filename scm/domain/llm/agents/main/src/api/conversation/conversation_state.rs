/// Marker satisfying SEA module correspondence with `core/conversation/conversation_state.rs`,
/// which declares `ConversationState` — the `Pipeline` `Ctx` for the conversation loop,
/// pure internal plumbing with no public `api/` type of its own.
///
/// SEA `no_orphan_types` tradeoff (accepted, tracked in edge-domain#132): revisit
/// only if `edge-pipeline`'s `Ctx` contract evolves to not require this marker.
pub struct ConversationState;
