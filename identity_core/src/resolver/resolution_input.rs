use serde::{Deserialize, Serialize};

use crate::resolver::ResolutionVersion;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResolutionInput {
    /// Requests a specific type of DID document response.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#resolving-input-accept)
    pub accept: Option<String>,
    /// Requests a specific version of a DID document.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#resolving-input-version-id)
    #[serde(flatten)]
    pub version: Option<ResolutionVersion>,
    /// Return a cached DID document if one is found.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#resolving-input-no-cache)
    #[serde(rename = "no-cache")]
    pub no_cache: bool,
}

impl ResolutionInput {
    pub const fn new() -> Self {
        Self {
            accept: None,
            version: None,
            no_cache: false,
        }
    }
}
