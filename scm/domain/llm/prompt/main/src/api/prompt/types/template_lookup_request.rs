//! [`TemplateLookupRequest`] — request for [`TemplateProvider::get_template`](crate::api::prompt::traits::TemplateProvider::get_template).

/// Request to fetch a template by `id`.
#[derive(Debug, PartialEq)]
pub struct TemplateLookupRequest<'a> {
    /// The template id to look up.
    pub id: &'a str,
}
