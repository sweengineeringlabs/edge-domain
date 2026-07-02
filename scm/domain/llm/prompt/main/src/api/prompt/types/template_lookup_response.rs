//! [`TemplateLookupResponse`] — response for [`TemplateProvider::get_template`](crate::api::prompt::traits::TemplateProvider::get_template).

use crate::api::prompt::types::PromptTemplate;

/// The template found for the requested id, if any.
#[derive(Debug, PartialEq)]
pub struct TemplateLookupResponse<'a> {
    /// The template registered under the requested id, if present.
    pub template: Option<&'a PromptTemplate>,
}
