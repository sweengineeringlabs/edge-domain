mod registry;

pub use registry::{
    InMemoryRegistry, Registry, RegistryError, RegistryBootstrap, StdRegistryFactory,
    REGISTRY_FACTORY_SVC, REGISTRY_SVC,
};
