//! `Page` — paginated result wrapper for repository list operations.

/// A paginated slice of results from a repository list operation.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Total number of matching entities across all pages.
    pub total: usize,
    /// The zero-based offset this page starts at.
    pub offset: usize,
    /// The maximum number of items per page that was requested.
    pub limit: usize,
}

impl<T> Page<T> {
    /// Creates a new `Page` with the given items, total count, offset, and limit.
    pub fn new(items: Vec<T>, total: usize, offset: usize, limit: usize) -> Self {
        Self {
            items,
            total,
            offset,
            limit,
        }
    }

    /// Returns `true` if there are more items beyond this page.
    pub fn has_more(&self) -> bool {
        self.offset + self.items.len() < self.total
    }

    /// Returns the offset to use for the next page, or `None` if this is the last page.
    pub fn next_offset(&self) -> Option<usize> {
        self.has_more().then_some(self.offset + self.items.len())
    }
}
