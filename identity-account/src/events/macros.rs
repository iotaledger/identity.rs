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
      #[derive(Clone, Debug)]
      pub struct [<$ident Builder>]<'account, K: IdentityKey> {
        account: &'account Account,
        key: K,
        $(
          $field: Option<$ty>,
        )*
      }

      impl<'account, K: IdentityKey> [<$ident Builder>]<'account, K> {
        $(
          pub fn $field<VALUE: Into<$ty>>(mut self, value: VALUE) -> Self {
            self.$field = Some(value.into());
            self
          }
        )*

        pub fn new(account: &'account Account, key: K) -> [<$ident Builder>]<'account, K> {
          [<$ident Builder>] {
            account,
            key,
            $(
              $field: None,
            )*
          }
        }

        pub async fn apply(self) -> $crate::Result<()> {
          let account = self.account;
          let update = $crate::events::Command::$ident {
            $(
              $field: impl_command_builder!(@finish self $requirement $field $ty $(= $value)?),
            )*
          };
          account.apply_command(self.key, update).await?;
          Ok(())
        }
      }

      // impl Default for [<$ident Builder>] {
      //   fn default() -> Self {
      //     Self::new()
      //   }
      // }

      impl<'account, K: IdentityKey + Clone> $crate::identity::IdentityUpdater<'account, K> {
        pub fn [<$ident:snake>](&self) -> [<$ident Builder>]<'account, K> {
          [<$ident Builder>]::new(self.account(), self.key().clone())
        }
      }
    }
  };
}
