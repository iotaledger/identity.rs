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
    /// cache.insert(key, value, None);
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
    /// cache.insert(key, value, None);
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
    /// cache.insert(key, value, None);
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
    /// assert_eq!(cache.get_or_insert(key, move || value, None), &value);
    /// assert!(cache.contains_key(&key));
    /// ```
    pub fn get_or_insert<F>(&mut self, key: K, func: F, lifetime: Option<Duration>) -> &V
    where
        F: Fn() -> V,
    {
        let now = SystemTime::now();

        self.try_remove_expired_items(now);

        match self.table.entry(key) {
            Entry::Occupied(mut occ) => {
                if occ.get().has_expired(now) {
                    occ.insert(Value::new(func(), lifetime));
                }

                &occ.into_mut().val
            }
            Entry::Vacant(vac) => &vac.insert(Value::new(func(), lifetime)).val,
        }
    }

    pub fn insert(&mut self, key: K, value: V, lifetime: Option<Duration>) -> Option<V> {
        let now = SystemTime::now();

        self.try_remove_expired_items(now);

        self.table
            .insert(key, Value::new(value, lifetime))
            .filter(|value| !value.has_expired(now))
            .map(|value| value.val)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let now = SystemTime::now();

        self.try_remove_expired_items(now);

        self.table
            .remove(key)
            .filter(|value| !value.has_expired(now))
            .map(|value| value.val)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let now = SystemTime::now();

        self.table.get(key).filter(|value| !value.has_expired(now)).is_some()
    }

    pub fn get_last_scanned_at(&self) -> Option<SystemTime> {
        self.last_scan_at
    }

    pub fn get_scan_frequency(&self) -> Option<Duration> {
        self.scan_frequency
    }

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
