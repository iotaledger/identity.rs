#[macro_export]
macro_rules! object {
    () => {
      $crate::common::Object::default()
    };
    ($($key:ident : $value:expr),* $(,)*) => {
      {
        let mut object = ::std::collections::HashMap::new();

        $(
          object.insert(
            stringify!($key).to_string(),
            $crate::common::Value::from($value),
          );
        )*

        $crate::common::Object::from(object)
      }
    };
}

// create a line error with the file and the line number.  Good for debugging.

#[macro_export]
macro_rules! line_error {
    () => {
        concat!("Error at ", file!(), ":", line!())
    };
    ($string:expr) => {
        concat!($string, " @", file!(), ":", line!())
    };
}

/// Creates a simple map using `map! { "key" => "value"}
#[allow(unused_macros)]
#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $val); )*
            map
    }}
}

/// Creates a simple HashSet using set! {"val_1", "val_2", ...};
#[allow(unused_macros)]
#[macro_export]
macro_rules! set {
    ($($val:expr),* $(,)?) => {{ #[allow(redundant_semicolons)] {
        let mut set = HashSet::new();
        $( set.insert($val); )* ;
        set
    }}}
}
