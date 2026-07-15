//! [`ListNamesResponse`] — wrapper for service name listing.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNamesResponse {
    pub names: Vec<String>,
}
