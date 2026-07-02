use crate::api::types::InputOutputSchema;

/// Response for [`Skill::input_schema`](crate::api::traits::Skill::input_schema).
pub struct InputSchemaResponse {
    /// JSON-Schema describing this skill's input contract, if any.
    pub schema: Option<Box<InputOutputSchema>>,
}
