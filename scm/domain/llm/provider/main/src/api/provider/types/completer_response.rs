use std::sync::Arc;

use edge_llm_complete::Completer;

/// Response for [`Provider::completer`](crate::api::provider::traits::Provider::completer).
#[derive(Clone)]
pub struct CompleterResponse {
    /// The HTTP-level completion boundary this provider delegates to.
    pub completer: Arc<dyn Completer>,
}
