//! Step service facade — provides step execution interface.

pub use crate::api::Step;

/// Marker constant for step service identification.
pub const STEP_SVC: &str = "step";

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: STEP_SVC constant
    #[test]
    fn test_step_svc_constant() {
        assert_eq!(STEP_SVC, "step");
    }
}
