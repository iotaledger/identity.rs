use serde::{Deserialize, Serialize};
use serde_json::{from_slice, from_str, to_string, to_string_pretty, to_vec};

use crate::error::{Error, Result};

pub trait AsJson: for<'de> Deserialize<'de> + Serialize + Sized {
    fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        from_str(json.as_ref()).map_err(Error::DecodeJSON)
    }

    fn from_json_slice(json: &(impl AsRef<[u8]> + ?Sized)) -> Result<Self> {
        from_slice(json.as_ref()).map_err(Error::DecodeJSON)
    }

    fn to_json(&self) -> Result<String> {
        to_string(self).map_err(Error::EncodeJSON)
    }

    fn to_json_vec(&self) -> Result<Vec<u8>> {
        to_vec(self).map_err(Error::EncodeJSON)
    }

    fn to_json_pretty(&self) -> Result<String> {
        to_string_pretty(self).map_err(Error::EncodeJSON)
    }
}

impl<T> AsJson for T where T: for<'de> Deserialize<'de> + Serialize + Sized {}
