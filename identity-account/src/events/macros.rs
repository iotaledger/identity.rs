// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

macro_rules! ensure {
  ($cond:expr, $error:expr $(,)?) => {
    if !$cond {
      return Err($crate::Error::CommandError($error));
    }
  };
}

macro_rules! impl_command_builder {
  (@finish $this:ident optional $field:ident $ty:ty) => {
    $this.$field
  };
  (@finish $this:ident default $field:ident $ty:ty) => {
    $this.$field.unwrap_or_default()
  };
  (@finish $this:ident defaulte $field:ident $ty:ty = $variant:ident) => {
    $this.$field.unwrap_or(<$ty>::$variant)
  };
  (@finish $this:ident defaultv $field:ident $ty:ty = $value:expr) => {
    $this.$field.unwrap_or_else(|| $value)
  };
  (@finish $this:ident required $field:ident $ty:ty) => {
    match $this.$field {
      Some(value) => value,
      None => return Err($crate::Error::CommandError(
        $crate::events::CommandError::MissingRequiredField(stringify!($field)),
      )),
    }
  };
  ($ident:ident { $(@ $requirement:ident $field:ident $ty:ty $(= $value:expr)?),* $(,)* }) => {
    paste::paste! {
      #[derive(Clone, Debug, PartialEq)]
      pub struct [<$ident Builder>] {
        $(
          $field: Option<$ty>,
        )*
      }

      impl [<$ident Builder>] {
        $(
          pub fn $field<VALUE: Into<$ty>>(mut self, value: VALUE) -> Self {
            self.$field = Some(value.into());
            self
          }
        )*

        pub fn new() -> [<$ident Builder>] {
          [<$ident Builder>] {
            $(
              $field: None,
            )*
          }
        }

        pub fn finish(self) -> $crate::Result<$crate::events::Command> {
          Ok($crate::events::Command::$ident {
            $(
              $field: impl_command_builder!(@finish self $requirement $field $ty $(= $value)?),
            )*
          })
        }
      }

      impl Default for [<$ident Builder>] {
        fn default() -> Self {
          Self::new()
        }
      }

      impl $crate::events::Command {
        pub fn [<$ident:snake>]() -> [<$ident Builder>] {
          [<$ident Builder>]::new()
        }
      }
    }
  };
}
