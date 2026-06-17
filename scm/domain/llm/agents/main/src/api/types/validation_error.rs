/// A validation failure describing which field failed and why.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The name of the field that failed validation.
    pub field: String,
    /// The reason the field failed validation.
    pub reason: String,
}

impl ValidationError {
    /// Creates a new validation error for the given field and reason.
    pub fn new(field: String, reason: String) -> Self {
        Self { field, reason }
    }
}
