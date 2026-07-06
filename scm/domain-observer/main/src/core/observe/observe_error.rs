//! `Display`/`Error` impls for [`ObserveError`].

use crate::api::ObserveError;

impl std::fmt::Display for ObserveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendUnavailable(msg) => write!(f, "observe backend unavailable: {msg}"),
            Self::NotInitialised => write!(f, "observe backend not initialised"),
        }
    }
}

impl std::error::Error for ObserveError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_backend_unavailable_includes_message() {
        let err = ObserveError::BackendUnavailable("timeout".to_string());
        assert_eq!(err.to_string(), "observe backend unavailable: timeout");
    }

    #[test]
    fn test_display_not_initialised_message() {
        assert_eq!(
            ObserveError::NotInitialised.to_string(),
            "observe backend not initialised"
        );
    }
}
