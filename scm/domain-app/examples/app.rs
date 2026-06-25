//! Basic `Application` and `Bootstrap` usage example.

use edge_domain_app::{AppError, Application, Bootstrap};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct GreetApp;

impl Application for GreetApp {
    fn name(&self) -> &str { "greet" }
    fn run(&self) -> BoxFuture<'_, Result<(), AppError>> {
        Box::pin(async {
            println!("Hello from edge-domain-app!");
            Ok(())
        })
    }
}

struct GreetBootstrap;

impl Bootstrap for GreetBootstrap {
    fn build(&self) -> Result<Box<dyn Application>, AppError> {
        Ok(Box::new(GreetApp))
    }
}

fn main() {
    let bootstrap = GreetBootstrap;
    let app = bootstrap.build().expect("bootstrap failed");
    block_on(app.run()).expect("app run failed");
}
