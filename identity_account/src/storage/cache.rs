use crate::storage::Value;
use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    time::{Duration, SystemTime},
};

/// Local memory cache for unencrypted items.
pub struct Cache<K, V> {
    table: HashMap<K, Value<V>>,
    scan_frequency: Option<Duration>,
    created_at: SystemTime,
    last_scan_at: Option<SystemTime>,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    /// creates a new empty `Cache`
    /// # Example
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let mut cache = Cache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// cache.insert(key, value, None, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value))
    /// ```
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            scan_frequency: None,
            created_at: SystemTime::now(),
            last_scan_at: None,
        }
    }

    /// creates an empty `Cache` with a periodic scanner which identifies expired entries.
    ///
    /// # Example
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = Cache::create_with_scanner(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// cache.insert(key, value, None, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// ```
    pub fn create_with_scanner(scan_frequency: Duration) -> Self {
        Self {
            table: HashMap::new(),
            scan_frequency: Some(scan_frequency),
            created_at: SystemTime::now(),
            last_scan_at: None,
        }
    }

    /// Gets the value associated with the specified key.
    ///
    /// # Example
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let mut cache = Cache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// cache.insert(key, value, None, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value))
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let now = SystemTime::now();

        self.table
            .get(&key)
            .filter(|value| !value.has_expired(now))
            .map(|value| &value.val)
    }

    /// Gets the value associated with the specified key.  If the key could not be found in the `Cache`, creates and
    /// inserts the value using a specified `func` function. # Example
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let mut cache = Cache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// assert_eq!(cache.get_or_insert(key, move || value, None, None), &value);
    /// assert!(cache.contains_key(&key));
    /// ```
    pub fn get_or_insert<F>(&mut self, key: K, func: F, lifetime: Option<Duration>, needs_cache: Option<bool>) -> &V
    where
        F: Fn() -> V,
    {
        let now = SystemTime::now();

        self.try_remove_expired_items(now);

        match self.table.entry(key) {
            Entry::Occupied(mut occ) => {
                if occ.get().has_expired(now) {
                    occ.insert(Value::new(func(), lifetime, needs_cache));
                }

                &occ.into_mut().val
            }
            Entry::Vacant(vac) => &vac.insert(Value::new(func(), lifetime, needs_cache)).val,
        }
    }

    /// Insert a key-value pair into the cache.
    /// If key was not present, a `None` is returned, else the value is updated and the old value is returned.  
    ///
    /// # Example
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let mut cache = Cache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// let insert = cache.insert(key, value, None, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// assert!(insert.is_none());
    /// ```
    pub fn insert(&mut self, key: K, value: V, lifetime: Option<Duration>, needs_cache: Option<bool>) -> Option<V> {
        let now = SystemTime::now();

        self.try_remove_expired_items(now);

        self.table
            .insert(key, Value::new(value, lifetime, needs_cache))
            .filter(|value| !value.has_expired(now))
            .map(|value| value.val)
    }

    /// Removes a key from the cache.  Returns the value from the key if the key existed in the cache.
    ///
    /// # Example
    ///
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let mut cache = Cache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// let insert = cache.insert(key, value, None, None);
    /// assert_eq!(cache.remove(&key), Some(value));
    /// assert!(!cache.contains_key(&key));
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let now = SystemTime::now();

        self.try_remove_expired_items(now);

        self.table
            .remove(key)
            .filter(|value| !value.has_expired(now))
            .map(|value| value.val)
    }

    // Check if the `Cache<K, V>` contains a specific key.
    pub fn contains_key(&self, key: &K) -> bool {
        let now = SystemTime::now();

        self.table.get(key).filter(|value| !value.has_expired(now)).is_some()
    }

    // Get the last scanned at time.
    pub fn get_last_scanned_at(&self) -> Option<SystemTime> {
        self.last_scan_at
    }

    /// Get the cache's scan frequency.
    ///
    /// # Example
    /// ```
    /// use identity_account::storage::Cache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = Cache::create_with_scanner(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "value";
    ///
    /// cache.insert(key, value, None, None);
    ///
    /// assert_eq!(cache.get_scan_frequency(), Some(scan_frequency));
    /// ```
    pub fn get_scan_frequency(&self) -> Option<Duration> {
        self.scan_frequency
    }

    /// attempts to remove expired items based on the current system time provided.
    fn try_remove_expired_items(&mut self, now: SystemTime) {
        if let Some(frequency) = self.scan_frequency {
            let since = now
                .duration_since(self.last_scan_at.unwrap_or(self.created_at))
                .expect("System time is before the scanned time");

            if since >= frequency {
                self.table.retain(|_, value| !value.has_expired(now));

                self.last_scan_at = Some(now)
            }
        }
    }
}

/// Default implementation for `Cache<K, V>`
impl<K: Hash + Eq, V> Default for Cache<K, V> {
    fn default() -> Self {
        Cache::new()
    }
}
