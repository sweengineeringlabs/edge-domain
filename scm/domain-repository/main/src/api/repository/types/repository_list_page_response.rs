//! [`RepositoryListPageResponse`] — wrapper for a paginated slice of entities.

use crate::api::repository::types::Page;

/// Result of [`Repository::list_page`](crate::api::repository::traits::Repository::list_page).
#[derive(Debug, Clone)]
pub struct RepositoryListPageResponse<T> {
    /// The requested page.
    pub page: Page<T>,
}
