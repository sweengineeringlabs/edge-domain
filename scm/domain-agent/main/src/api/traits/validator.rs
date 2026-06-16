//! Validator trait — validates agent and skill configurations.

/// A validator checks agent and skill configurations for correctness and compliance.
///
/// Implementations verify that configurations meet requirements before agents are deployed.
pub trait Validator: Send + Sync {
    /// Validate an agent identifier.
    ///
    /// # Arguments
    /// * `agent_id` - The agent identifier to validate (e.g., "chief_engineer")
    ///
    /// # Returns
    /// Ok(()) if valid, Err with reason if invalid
    fn validate_agent_id(&self, agent_id: &str) -> Result<(), String>;

    /// Validate a skill name.
    ///
    /// # Arguments
    /// * `skill_name` - The skill name to validate (e.g., "code_review")
    ///
    /// # Returns
    /// Ok(()) if valid, Err with reason if invalid
    fn validate_skill_name(&self, skill_name: &str) -> Result<(), String>;

    /// Validate skill input payload.
    ///
    /// # Arguments
    /// * `input` - The input payload (typically JSON string) to validate
    ///
    /// # Returns
    /// Ok(()) if valid, Err with reason if invalid
    fn validate_skill_input(&self, input: &str) -> Result<(), String>;
}
