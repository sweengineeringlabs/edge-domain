//! [`ListByCategoryResponse`] — response for [`TemplateProvider::list_by_category`](crate::api::prompt::traits::TemplateProvider::list_by_category).

use crate::api::prompt::types::PromptTemplate;

/// The templates matching the requested category.
#[derive(Debug, PartialEq)]
pub struct ListByCategoryResponse<'a> {
    /// Templates whose category matched.
    pub templates: Vec<&'a PromptTemplate>,
}
