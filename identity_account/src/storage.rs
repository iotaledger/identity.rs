use std::time::{Duration, SystemTime};

mod cache;
mod compress;
mod file;
mod utils;

pub use cache::Cache;
pub use compress::HuffmanCodec;
pub use file::CacheFile;

/// Main value for the `Cache`.  Contains an expiration time and a boolean.
#[derive(Clone, Debug)]
pub(crate) struct Value<T> {
    pub val: T,
    expiration: Option<SystemTime>,
    file_backed: Option<bool>,
}

impl<T> Value<T> {
    pub fn new(val: T, duration: Option<Duration>, file_backed: Option<bool>) -> Self {
        Value {
            val,
            expiration: duration.map(|dur| SystemTime::now() + dur),
            file_backed,
        }
    }

    pub fn has_expired(&self, time_now: SystemTime) -> bool {
        self.expiration.map_or(false, |time| time_now >= time)
    }

    pub fn file_backed(&self) -> bool {
        self.file_backed.map_or(false, |t| t)
    }
}
