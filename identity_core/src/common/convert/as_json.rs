use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

pub trait ToJson: Serialize {
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::EncodeJSON)
    }

    fn to_json_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).map_err(Error::EncodeJSON)
    }

    fn to_json_value(&self) -> Result<serde_json::Value> {
        serde_json::to_value(self).map_err(Error::EncodeJSON)
    }

    fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Error::EncodeJSON)
    }
}

impl<T> ToJson for T where T: Serialize {}

// =============================================================================
// =============================================================================

pub trait FromJson: for<'de> Deserialize<'de> + Sized {
    fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        serde_json::from_str(json.as_ref()).map_err(Error::DecodeJSON)
    }

    fn from_json_slice(json: &(impl AsRef<[u8]> + ?Sized)) -> Result<Self> {
        serde_json::from_slice(json.as_ref()).map_err(Error::DecodeJSON)
    }

    fn from_json_value(json: serde_json::Value) -> Result<Self> {
        serde_json::from_value(json).map_err(Error::DecodeJSON)
    }
}

impl<T> FromJson for T where T: for<'de> Deserialize<'de> + Sized {}

// =============================================================================
// =============================================================================

pub trait AsJson: FromJson + ToJson {
    fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        <Self as FromJson>::from_json(json)
    }

    fn from_json_slice(json: &(impl AsRef<[u8]> + ?Sized)) -> Result<Self> {
        <Self as FromJson>::from_json_slice(json)
    }

    fn to_json(&self) -> Result<String> {
        <Self as ToJson>::to_json(self)
    }

    fn to_json_vec(&self) -> Result<Vec<u8>> {
        <Self as ToJson>::to_json_vec(self)
    }

    fn to_json_value(&self) -> Result<serde_json::Value> {
        <Self as ToJson>::to_json_value(self)
    }

    fn to_json_pretty(&self) -> Result<String> {
        <Self as ToJson>::to_json_pretty(self)
    }
}

impl<T> AsJson for T where T: FromJson + ToJson {}
