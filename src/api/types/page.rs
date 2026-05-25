//! `Page<T>` — paginated result from a [`Repository`](super::repository::Repository) list query.

/// A single page of results from a paginated repository query.
///
/// `total` reflects the full entity count across all pages, not just this page.
/// Use `has_more` or `next_offset` to drive cursor advancement.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// Entities on this page.
    pub items: Vec<T>,
    /// Total entity count across all pages (not just this one).
    pub total: usize,
    /// Zero-based index of the first item on this page.
    pub offset: usize,
    /// Maximum number of items requested per page.
    pub limit: usize,
}

impl<T> Page<T> {
    /// Construct a page result.
    pub fn new(items: Vec<T>, total: usize, offset: usize, limit: usize) -> Self {
        Self {
            items,
            total,
            offset,
            limit,
        }
    }

    /// `true` when more items exist beyond this page.
    pub fn has_more(&self) -> bool {
        self.offset + self.items.len() < self.total
    }

    /// Offset of the next page's first item, or `None` if this is the last page.
    pub fn next_offset(&self) -> Option<usize> {
        self.has_more().then_some(self.offset + self.items.len())
    }
}


