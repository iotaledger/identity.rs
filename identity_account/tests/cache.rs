use identity_account::storage::Cache;
use std::time::Duration;

#[test]
#[should_panic]
fn test_expired_key() {
    let mut cache = Cache::new();
    let key: &'static str = "key";

    cache.insert(key, 1, Some(Duration::default()), None);

    assert!(cache.contains_key(&key));
}

#[test]
fn test_get_expired_value() {
    let mut cache = Cache::new();
    let key: &'static str = "key";

    cache.insert(key, 1, Some(Duration::default()), None);

    assert_eq!(cache.get(&key), None);
}

#[test]
fn test_insert_return_old() {
    let mut cache = Cache::new();
    let key: &'static str = "key";

    let res_a = cache.insert(key, 1, Some(Duration::default()), None);
    let res_b = cache.insert(key, 2, None, None);
    let res_c = cache.insert(key, 3, None, Some(true));

    assert_eq!(res_a, None);
    assert_eq!(res_b, None);
    assert_eq!(res_c, Some(2));
}

#[test]
fn test_get_or_insert_expired() {
    let mut cache = Cache::new();
    let key: &'static str = "key";

    cache.get_or_insert(key, || 1, Some(Duration::default()), None);
    let value = cache.get_or_insert(key, || 2, None, None);

    assert_eq!(value, &2);
}

#[test]
fn test_remove_expired() {
    let mut cache = Cache::new();
    let key: &'static str = "key";

    cache.insert(key, 1, Some(Duration::default()), None);
    let res = cache.remove(&key);

    assert_eq!(res, None);
}

#[test]
fn test_scanner() {
    let scanner = Duration::default();
    let mut cache = Cache::create_with_scanner(scanner);
    let key: &'static str = "key";

    cache.insert(key, 1, None, None);

    let scanner = cache.get_scan_frequency();

    assert!(scanner.is_some())
}
