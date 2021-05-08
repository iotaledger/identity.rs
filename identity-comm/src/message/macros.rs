// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

macro_rules! impl_message_accessor {
  ($name:expr, $field:ident => $ty:ty) => {
    paste::paste! {
      #[doc = "Sets the value of the `"]
      #[doc = $name]
      #[doc = "` field"]
      pub fn [<set_ $field>]<VALUE: Into<$ty>>(&mut self, value: VALUE) {
        self.$field = value.into();
      }

      #[doc = "Returns a reference to the `"]
      #[doc = $name]
      #[doc = "` field"]
      pub fn $field(&self) -> &$ty {
        &self.$field
      }

      #[doc = "Returns a mutable reference to the `"]
      #[doc = $name]
      #[doc = "` field"]
      pub fn [<$field _mut>](&mut self) -> &mut $ty {
        &mut self.$field
      }
    }
  };
  ($field:ident => $ty:ty) => {
    impl_message_accessor!(stringify!($field), $field => $ty);
  };
  ($($field:ident => $ty:ty)+) => {
    $(
      impl_message_accessor!($field, $ty);
    )+
  };
}
