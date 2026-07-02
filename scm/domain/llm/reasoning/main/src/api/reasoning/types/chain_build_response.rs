use crate::api::reasoning::types::ReasoningChain;

/// Response for [`Reasoning::build_chain`](crate::api::reasoning::traits::Reasoning::build_chain).
#[derive(Debug, Clone)]
pub struct ChainBuildResponse {
    /// Assembled reasoning chain.
    pub chain: Box<ReasoningChain>,
}
