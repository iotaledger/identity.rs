use serde::{Deserialize, Serialize};

use crate::common::{Timestamp, Value};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResolutionVersion {
    /// Requests a specific version of a DID document via timestamp.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#resolving-input-version-time)
    #[serde(rename = "version-time")]
    Time(Timestamp),
    /// Requests a specific version of a DID document via method identifier.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#resolving-input-version-id)
    #[serde(rename = "version-id")]
    Id(Value),
}

impl From<Timestamp> for ResolutionVersion {
    fn from(other: Timestamp) -> Self {
        Self::Time(other)
    }
}

impl From<Value> for ResolutionVersion {
    fn from(other: Value) -> Self {
        Self::Id(other)
    }
}
