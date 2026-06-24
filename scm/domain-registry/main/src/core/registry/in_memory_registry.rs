//! `Registry` impl for `InMemoryRegistry`.

use std::sync::Arc;

use crate::api::RegistryError;
use crate::api::Registry;
use crate::api::InMemoryRegistry;

impl<V: ?Sized + Send + Sync> Registry for InMemoryRegistry<V> {
    type Value = V;

    fn register(&self, id: &str, entry: Arc<V>) {
        // A poisoned lock means a prior holder panicked; the map is still
        // structurally sound, so recover the guard rather than propagate.
        let mut map = self.entries.write().unwrap_or_else(|e| e.into_inner());
        map.insert(id.to_string(), entry);
    }

    fn try_register(&self, id: &str, entry: Arc<V>) -> Result<(), RegistryError> {
        let mut map = self.entries.write().unwrap_or_else(|e| e.into_inner());
        if map.contains_key(id) {
            return Err(RegistryError::DuplicateId(id.to_string()));
        }
        map.insert(id.to_string(), entry);
        Ok(())
    }

    fn deregister(&self, id: &str) -> bool {
        let mut map = self.entries.write().unwrap_or_else(|e| e.into_inner());
        map.remove(id).is_some()
    }

    fn get(&self, id: &str) -> Option<Arc<V>> {
        let map = self.entries.read().unwrap_or_else(|e| e.into_inner());
        map.get(id).map(Arc::clone)
    }

    fn list_ids(&self) -> Vec<String> {
        let map = self.entries.read().unwrap_or_else(|e| e.into_inner());
        map.keys().cloned().collect()
    }

    fn len(&self) -> usize {
        let map = self.entries.read().unwrap_or_else(|e| e.into_inner());
        map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_then_get_returns_entry() {
        let reg: InMemoryRegistry<str> = InMemoryRegistry::new();
        reg.register("a", Arc::from("alpha"));
        assert_eq!(reg.get("a").as_deref(), Some("alpha"));
    }

    #[test]
    fn test_register_replaces_existing_entry() {
        let reg: InMemoryRegistry<str> = InMemoryRegistry::new();
        reg.register("a", Arc::from("alpha"));
        reg.register("a", Arc::from("beta"));
        assert_eq!(reg.get("a").as_deref(), Some("beta"));
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_try_register_duplicate_returns_err() {
        let reg: InMemoryRegistry<str> = InMemoryRegistry::new();
        reg.register("a", Arc::from("alpha"));
        assert_eq!(
            reg.try_register("a", Arc::from("beta")),
            Err(RegistryError::DuplicateId("a".to_string()))
        );
        // original entry untouched
        assert_eq!(reg.get("a").as_deref(), Some("alpha"));
    }

    #[test]
    fn test_deregister_removes_entry_and_reports_presence() {
        let reg: InMemoryRegistry<str> = InMemoryRegistry::new();
        reg.register("a", Arc::from("alpha"));
        assert!(reg.deregister("a"));
        assert!(!reg.deregister("a"));
        assert!(reg.get("a").is_none());
    }

    #[test]
    fn test_list_ids_and_len_reflect_contents() {
        let reg: InMemoryRegistry<str> = InMemoryRegistry::new();
        assert!(reg.is_empty());
        reg.register("a", Arc::from("alpha"));
        reg.register("b", Arc::from("beta"));
        let mut ids = reg.list_ids();
        ids.sort();
        assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(reg.len(), 2);
    }
}
