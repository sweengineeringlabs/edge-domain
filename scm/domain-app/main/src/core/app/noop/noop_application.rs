use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::Application;
use crate::api::NoopApplication;

impl Application for NoopApplication {
    fn run(&self) -> BoxFuture<'_, Result<(), AppError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn test_run_noop_happy() {
        let app = NoopApplication;
        assert_eq!(block_on(app.run()), Ok(()));
    }

    #[test]
    fn test_run_noop_twice_both_ok_error() {
        let app = NoopApplication;
        assert_eq!(block_on(app.run()), Ok(()));
        assert_eq!(block_on(app.run()), Ok(()));
    }

    #[test]
    fn test_name_default_is_application_edge() {
        assert_eq!(NoopApplication.name(), "application");
    }
}
