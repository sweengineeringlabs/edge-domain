//! `AppServiceProvider` impl for [`DefaultAppServiceProvider`] — returns a `NoopAppBootstrap` as the default service graph.

use crate::api::AppServiceProvider;
use crate::api::Bootstrap;
use crate::api::NoopAppBootstrap;

pub(crate) struct DefaultAppServiceProvider;

impl AppServiceProvider for DefaultAppServiceProvider {
    fn build(&self) -> Box<dyn Bootstrap> {
        Box::new(NoopAppBootstrap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::AppServiceProvider;

    #[test]
    fn test_build_returns_bootstrap_happy() {
        let f = DefaultAppServiceProvider;
        let app = f.build().build().expect("DefaultAppServiceProvider must produce a buildable bootstrap");
        assert_eq!(app.name(), "application");
    }

    #[test]
    fn test_build_called_twice_both_succeed_error() {
        let f = DefaultAppServiceProvider;
        let app1 = f.build().build().expect("first build must succeed");
        let app2 = f.build().build().expect("second build must succeed");
        assert_eq!(app1.name(), app2.name());
    }

    #[test]
    fn test_name_returns_app_service_provider_edge() {
        assert_eq!(DefaultAppServiceProvider.name(), "app_service_provider");
    }
}
