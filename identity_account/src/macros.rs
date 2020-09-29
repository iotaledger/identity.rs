/// A macro for defining functions whose return values will be cached in a `Cache`.
#[macro_export]
macro_rules! cache {
    (fn $name:ident ($($arg:ident: $arg_type:ty), *) -> $ret:ty = $body:expr) => {
        #[allow(unused_parens)]
        fn $name($(arg: $arg_type), *) -> $ret {
            static CACHE: Lazy<Mutex<Cache<($($arg_type),*), $ret>>> =
                Lazy::new(|| Mutex::new(Cache::new()));

            let key = ($($arg.clone()), *);

            let cache = CACHE.lock().unwrap();

            match cache.get(&key) {
                Some(val) => val.clone(),
                None => {
                    drop(cache);
                    let value = (||$body)();
                    let mut cache = CACHE.lock().unwrap();
                    cache.insert(key, value, None);
                    value.clone()
                }
            }
        }
    };
}
