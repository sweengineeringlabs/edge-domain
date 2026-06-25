use crate::api::AppServiceProvider;
use crate::api::Bootstrap;
use crate::api::NoopAppBootstrap;
use crate::api::NoopAppSvcFactory;

impl AppServiceProvider for NoopAppSvcFactory {
    fn build(&self) -> Box<dyn Bootstrap> {
        Box::new(NoopAppBootstrap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::AppServiceProvider;

    #[test]
    fn test_build_returns_noop_bootstrap_happy() {
        let factory = NoopAppSvcFactory;
        let app = factory.build().build().expect("NoopAppSvcFactory must produce a buildable bootstrap");
        assert_eq!(app.name(), "application");
    }

    #[test]
    fn test_build_noop_bootstrap_application_name_is_default_error() {
        let factory = NoopAppSvcFactory;
        let bootstrap = factory.build();
        let app = bootstrap.build().expect("build succeeds");
        assert_eq!(app.name(), "application");
    }

    #[test]
    fn test_name_returns_app_service_provider_edge() {
        assert_eq!(NoopAppSvcFactory.name(), "app_service_provider");
    }
}
