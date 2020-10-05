use std::time::{Duration, SystemTime};

mod cache;
mod compress;
mod file;
mod utils;

pub use cache::Cache;
pub use compress::HuffmanCodec;
pub use file::CacheFile;

/// Main value for the `Cache`.  Contains an expiration time and a boolean.
#[derive(Clone)]
pub(crate) struct Value<T> {
    pub val: T,
    expiration: Option<SystemTime>,
    needs_cache: Option<bool>,
}

impl<T> Value<T> {
    pub fn new(val: T, duration: Option<Duration>, needs_cache: Option<bool>) -> Self {
        Value {
            val,
            expiration: duration.map(|dur| SystemTime::now() + dur),
            needs_cache,
        }
    }

    pub fn has_expired(&self, time_now: SystemTime) -> bool {
        self.expiration.map_or(false, |time| time_now >= time)
    }

    pub fn should_cache(&self) -> bool {
        self.needs_cache.map_or(false, |t| t)
    }
}
