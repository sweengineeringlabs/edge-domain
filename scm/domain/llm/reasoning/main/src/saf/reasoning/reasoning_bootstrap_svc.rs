pub use crate::api::LinearReasoning;
pub use crate::api::PatternMetadataBuilder;
pub use crate::api::ReasoningChainBuilder;
pub use crate::api::ReasoningBootstrap;
pub use crate::api::ReasoningStepBuilder;
pub use crate::api::StdReasoningFactory;
pub use crate::api::StepResultBuilder;
pub use crate::api::ThinkingProcessBuilder;

use crate::api::{Reasoning, ReasoningPattern, ThinkingProcess};
use crate::spi::DefaultReasoningHandler;
use edge_domain_handler::Handler;
use std::sync::Arc;

/// SAF contract identifier for the reasoning-bootstrap service.
pub const REASONING_FACTORY_SVC: &str = "reasoning_factory";

impl StdReasoningFactory {
    /// Construct a dispatchable reasoning handler backed by the given reasoner.
    pub fn reasoning_handler(
        reasoner: Arc<dyn Reasoning>,
    ) -> impl Handler<Request = String, Response = ThinkingProcess> {
        DefaultReasoningHandler { reasoner }
    }

    /// Construct a dispatchable reasoning handler backed by the reference [`LinearReasoning`].
    pub fn default_reasoning_handler(
        pattern: ReasoningPattern,
    ) -> impl Handler<Request = String, Response = ThinkingProcess> {
        DefaultReasoningHandler::with_pattern(pattern)
    }
}
