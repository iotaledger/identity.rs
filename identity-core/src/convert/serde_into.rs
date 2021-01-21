use crate::{
    convert::{AsJson, ToJson},
    error::Result,
};

/// An escape-hatch for converting between types that represent the same JSON
/// structure.
pub trait SerdeInto: ToJson {
    /// Converts `self` to `T` by converting to/from JSON.
    fn serde_into<T>(&self) -> Result<T>
    where
        T: AsJson,
    {
        <Self as ToJson>::to_json_value(self).and_then(<T as AsJson>::from_json_value)
    }
}

impl<T> SerdeInto for T where T: ToJson {}
