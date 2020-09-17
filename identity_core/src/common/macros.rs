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
