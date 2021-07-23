// export FlatEnum derive
#[cfg(feature = "derive")]
pub use flat_enum_derive as derive;

/// Convenience trait to enable generics over the derived structs, such as with [IntoFlatEnum]
pub trait FlatEnum: serde::Serialize {}

pub trait IntoFlatEnum<T>
where
  T: FlatEnum,
{
  fn into_flat_enum(self) -> T;
}
