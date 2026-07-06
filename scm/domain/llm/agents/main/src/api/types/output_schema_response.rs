use crate::api::types::InputOutputSchema;

/// Response for [`Skill::output_schema`](crate::api::traits::Skill::output_schema).
pub struct OutputSchemaResponse {
    /// JSON-Schema describing this skill's output contract, if any.
    pub schema: Option<Box<InputOutputSchema>>,
}
