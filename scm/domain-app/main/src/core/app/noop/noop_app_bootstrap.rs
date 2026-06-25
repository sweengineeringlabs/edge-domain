use crate::api::AppError;
use crate::api::Application;
use crate::api::Bootstrap;
use crate::api::NoopAppBootstrap;
use crate::api::NoopApplication;

impl Bootstrap for NoopAppBootstrap {
    fn build(&self) -> Result<Box<dyn Application>, AppError> {
        Ok(Box::new(NoopApplication))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_returns_application_happy() {
        let result = NoopAppBootstrap.build();
        let app = result.expect("build should succeed");
        assert_eq!(app.name(), "application");
    }

    #[test]
    fn test_build_never_returns_err_error() {
        let result = NoopAppBootstrap.build();
        assert_eq!(result.is_ok(), true);
        let app = result.unwrap();
        assert_eq!(app.name(), "application");
    }

    #[test]
    fn test_build_multiple_times_each_ok_edge() {
        for _ in 0..3 {
            let app = NoopAppBootstrap.build().expect("build should succeed");
            assert_eq!(app.name(), "application");
        }
    }
}
