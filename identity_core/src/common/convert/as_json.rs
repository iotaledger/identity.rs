use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

use crate::error::{Error, Result};

pub trait AsJson: for<'de> Deserialize<'de> + Serialize + Sized {
    fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        from_str(json.as_ref()).map_err(Error::DecodeJSON)
    }

    fn to_json(&self) -> Result<String> {
        to_string(self).map_err(Error::EncodeJSON)
    }

    fn to_json_pretty(&self) -> Result<String> {
        to_string_pretty(self).map_err(Error::EncodeJSON)
    }
}

impl<T> AsJson for T where T: for<'de> Deserialize<'de> + Serialize + Sized {}
