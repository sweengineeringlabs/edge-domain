//! `CacheableMessage` impl for `Message`.

use crate::api::{CacheControl, CacheableMessage, Message};

impl CacheableMessage for Message {
    fn with_cache_control(mut self, cache: CacheControl) -> Self {
        self.cache_control = Some(cache);
        self
    }
}
