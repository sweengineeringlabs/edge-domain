use crate::api::reasoning::types::ThinkingProcess;

/// Request for [`Reasoning::build_chain`](crate::api::reasoning::traits::Reasoning::build_chain).
#[derive(Debug, Clone)]
pub struct ChainBuildRequest<'a> {
    /// Identifier for the assembled chain.
    pub chain_id: &'a str,
    /// Ordered reasoning processes to assemble.
    pub processes: Vec<ThinkingProcess>,
}
