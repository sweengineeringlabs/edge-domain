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
        Self { items, total, offset, limit }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: has_more
    #[test]
    fn test_page_has_more_returns_true_when_items_remain() {
        let page = Page::new(vec![1, 2, 3], 10, 0, 3);
        assert!(page.has_more());
    }

    /// @covers: has_more
    #[test]
    fn test_page_has_more_returns_false_on_last_page() {
        let page = Page::new(vec![9, 10], 10, 8, 3);
        assert!(!page.has_more());
    }

    /// @covers: next_offset
    #[test]
    fn test_next_offset_returns_some_when_more_items_remain() {
        let page = Page::new(vec![1, 2, 3], 10, 0, 3);
        assert_eq!(page.next_offset(), Some(3));
    }

    /// @covers: next_offset
    #[test]
    fn test_next_offset_returns_none_on_last_page() {
        let page = Page::new(vec![9, 10], 10, 8, 3);
        assert_eq!(page.next_offset(), None);
    }

    #[test]
    fn test_page_empty_items_has_no_more() {
        let page: Page<u32> = Page::new(vec![], 0, 0, 10);
        assert!(!page.has_more());
    }
}
