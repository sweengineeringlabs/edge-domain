/// A validation failure describing which field failed and why.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The name of the field that failed validation.
    pub field: String,
    /// The reason the field failed validation.
    pub reason: String,
}
