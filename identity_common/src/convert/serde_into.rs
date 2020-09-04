use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value};

use crate::error::{Error, Result};

pub trait SerdeInto: Serialize + Sized {
    fn serde_into<T>(&self) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        to_value(self)
            .map_err(Error::EncodeJSON)
            .and_then(|value| from_value(value).map_err(Error::DecodeJSON))
    }
}

impl<T> SerdeInto for T where T: Serialize + Sized {}
