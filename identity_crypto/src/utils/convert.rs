use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value};

use crate::error::{Error, Result};

pub fn convert<T>(this: &(impl Serialize + ?Sized)) -> Result<T>
where
  T: DeserializeOwned,
{
  to_value(this)
    .map_err(Error::EncodeJSON)
    .and_then(|value| from_value(value).map_err(Error::DecodeJSON))
}
