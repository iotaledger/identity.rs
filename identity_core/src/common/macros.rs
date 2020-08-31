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
