use serde::{Deserialize, Serialize};

use crate::resolver::ResolutionInput;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DereferenceInput {
    /// Requests a specific type of DID response.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#dereferencing-input-accept)
    pub accept: Option<String>,
    /// Instruct the client to follow redirects.
    ///
    /// [More Info](https://w3c-ccg.github.io/did-resolution/#dereferencing-input-follow-redirect)
    #[serde(rename = "follow-redirect")]
    pub follow_redirect: bool,
    /// Additional options passed to the DID resolution process.
    #[serde(rename = "_")]
    pub resolution: ResolutionInput,
}

impl DereferenceInput {
    pub const fn new() -> Self {
        Self {
            accept: None,
            follow_redirect: false,
            resolution: ResolutionInput::new(),
        }
    }
}
