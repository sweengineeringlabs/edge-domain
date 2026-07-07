/// Marker satisfying SEA module correspondence with `core/conversation/conversation_turn_step.rs`,
/// which bridges one conversation turn into `edge-pipeline`'s foreign `Step`
/// contract — a foreign trait has no home in this crate's own `api/`, hence this marker.
pub struct ConversationTurnStep;
