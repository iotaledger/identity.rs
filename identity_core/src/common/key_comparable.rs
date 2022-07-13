// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A trait for comparing types only by a certain key.
pub trait KeyComparable {
  /// Key type for comparisons.
  type Key: PartialEq + ?Sized;

  /// Returns a reference to the key.
  fn key(&self) -> &Self::Key;
}

/// Macro to implement the `KeyComparable` trait for primitive types.
///
/// This approach is used to avoid conflicts from a blanket implementation where the type is the
/// key itself.
macro_rules! impl_key_comparable {
    ($($t:ty)*) => ($(
        impl KeyComparable for $t {
            type Key = $t;
            #[inline]
            fn key(&self) -> &Self::Key { self }
        }
    )*)
}

impl_key_comparable! {
    str bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64
}

impl KeyComparable for &str {
  type Key = str;

  fn key(&self) -> &Self::Key {
    self
  }
}

impl KeyComparable for String {
  type Key = str;

  fn key(&self) -> &Self::Key {
    self
  }
}
