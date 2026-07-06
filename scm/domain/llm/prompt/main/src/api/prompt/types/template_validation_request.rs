//! [`TemplateValidationRequest`] — request for [`Prompt::validate`](crate::api::prompt::traits::Prompt::validate).

/// Request to validate template syntax. Carries no data.
#[derive(Debug, PartialEq)]
pub struct TemplateValidationRequest;
