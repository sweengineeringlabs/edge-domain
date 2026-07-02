//! [`ListByCategoryRequest`] — request for [`TemplateProvider::list_by_category`](crate::api::prompt::traits::TemplateProvider::list_by_category).

/// Request to list templates whose `category` matches.
#[derive(Debug, PartialEq)]
pub struct ListByCategoryRequest<'a> {
    /// The category to filter by.
    pub category: &'a str,
}
