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

#[macro_export]
macro_rules! impl_builder_setter {
    ($fn:ident, $field:ident, Option<$ty:ty>) => {
        impl_builder_setter!(@impl $fn, $field, $ty, Option);
    };
    ($fn:ident, $field:ident, Vec<$ty:ty>) => {
        impl_builder_setter!(@impl $fn, $field, $ty, Vec);
    };
    ($fn:ident, $field:ident, $ty:ty) => {
        impl_builder_setter!(@impl $fn, $field, $ty, None);
    };
    (@impl $fn:ident, $field:ident, $inner:ty, $outer:ident) => {
        pub fn $fn(mut self, value: impl Into<$inner>) -> Self {
            impl_builder_setter!(@expr self, $field, value, $outer);
            self
        }
    };
    (@expr $self:ident, $field:ident, $value:expr, Option) => {
        $self.$field = Some($value.into());
    };
    (@expr $self:ident, $field:ident, $value:expr, Vec) => {
        $self.$field.push($value.into());
    };
    (@expr $self:ident, $field:ident, $value:expr, None) => {
        $self.$field = $value.into();
    };
}

#[macro_export]
macro_rules! impl_builder_try_setter {
    ($fn:ident, $field:ident, Option<$ty:ty>) => {
        impl_builder_try_setter!(@impl $fn, $field, $ty, Option);
    };

    ($fn:ident, $field:ident, Vec<$ty:ty>) => {
        impl_builder_try_setter!(@impl $fn, $field, $ty, Vec);
    };

    ($fn:ident, $field:ident, $ty:ty) => {
        impl_builder_try_setter!(@impl $fn, $field, $ty, None);
    };
    (@impl $fn:ident, $field:ident, $inner:ty, $outer:ident) => {
        pub fn $fn<T>(mut self, value: T) -> ::std::result::Result<Self, T::Error>
        where
            T: ::std::convert::TryInto<$inner>
        {
            value.try_into()
                .map(|value| {
                    impl_builder_try_setter!(@expr self, $field, value, $outer);
                    self
                })
                .map_err(Into::into)
        }
    };
    (@expr $self:ident, $field:ident, $value:expr, Option) => {
        $self.$field = Some($value);
    };
    (@expr $self:ident, $field:ident, $value:expr, Vec) => {
        $self.$field.push($value);
    };
    (@expr $self:ident, $field:ident, $value:expr, None) => {
        $self.$field = $value;
    };
}
