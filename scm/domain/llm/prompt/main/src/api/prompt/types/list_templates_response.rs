//! [`ListTemplatesResponse`] — response for [`TemplateProvider::list_templates`](crate::api::prompt::traits::TemplateProvider::list_templates).

use crate::api::prompt::types::PromptTemplate;

/// Every registered template.
#[derive(Debug, PartialEq)]
pub struct ListTemplatesResponse<'a> {
    /// All registered templates.
    pub templates: Vec<&'a PromptTemplate>,
}
