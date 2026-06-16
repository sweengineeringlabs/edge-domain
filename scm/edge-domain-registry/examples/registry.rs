//! Basic `Registry` usage example.

use std::sync::Arc;

use edge_domain_registry::{Registry, RegistryFactory, StdRegistryFactory};

fn main() {
    let registry = StdRegistryFactory::in_memory::<str>();
    registry.register("greeting", Arc::from("hello"));
    registry.register("farewell", Arc::from("goodbye"));

    if let Some(v) = registry.get("greeting") {
        println!("greeting = {}", &*v);
    }
    println!("ids = {:?}", registry.list_ids());

    // strict registration rejects a duplicate id
    match registry.try_register("greeting", Arc::from("hi")) {
        Ok(()) => println!("registered"),
        Err(e) => println!("rejected: {e}"),
    }
}
