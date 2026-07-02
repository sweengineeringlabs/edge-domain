//! [`ListTemplatesRequest`] — request for [`TemplateProvider::list_templates`](crate::api::prompt::traits::TemplateProvider::list_templates).

/// Request to list every registered template. Carries no data.
#[derive(Debug, PartialEq)]
pub struct ListTemplatesRequest;
