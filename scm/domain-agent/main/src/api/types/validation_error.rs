#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub reason: String,
}

impl ValidationError {
    pub fn new(field: String, reason: String) -> Self {
        Self { field, reason }
    }
}
