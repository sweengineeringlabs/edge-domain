//! Tests for `Page<T>` type

use edge_domain::Page;

#[test]
fn test_page_creation() {
    let page = Page::new(vec![1, 2, 3], 0, 10);
    assert_eq!(page.items().len(), 3);
    assert_eq!(page.offset(), 0);
    assert_eq!(page.limit(), 10);
}

#[test]
fn test_page_empty() {
    let page: Page<i32> = Page::new(vec![], 0, 10);
    assert!(page.items().is_empty());
}
