// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

macro_rules! ensure {
  ($cond:expr, $error:expr $(,)?) => {
    if !$cond {
      return Err($crate::Error::UpdateError($error));
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
      None => return Err($crate::Error::UpdateError(
        $crate::events::UpdateError::MissingRequiredField(stringify!($field)),
      )),
    }
  };
  ($(#[$doc:meta])* $ident:ident { $(@ $requirement:ident $field:ident $ty:ty $(= $value:expr)?),* $(,)* }) => {
    paste::paste! {
      $(#[$doc])*
      #[derive(Clone, Debug)]
      pub struct [<$ident Builder>]<'account> {
        account: &'account Account,
        $(
          $field: Option<$ty>,
        )*
      }

      impl<'account> [<$ident Builder>]<'account> {
        $(
          pub fn $field<VALUE: Into<$ty>>(mut self, value: VALUE) -> Self {
            self.$field = Some(value.into());
            self
          }
        )*

        pub fn new(account: &'account Account) -> [<$ident Builder>]<'account> {
          [<$ident Builder>] {
            account,
            $(
              $field: None,
            )*
          }
        }

        pub async fn apply(self) -> $crate::Result<()> {
          let update = $crate::events::Update::$ident {
            $(
              $field: impl_command_builder!(@finish self $requirement $field $ty $(= $value)?),
            )*
          };

          self.account.process_update(update, false).await
        }
      }

      impl<'account> $crate::identity::IdentityUpdater<'account> {
        /// Creates a new builder to modify the identity. See the documentation of the return type for details.
        pub fn [<$ident:snake>](&self) -> [<$ident Builder>]<'account> {
          [<$ident Builder>]::new(self.account)
        }
      }
    }
  };
}
