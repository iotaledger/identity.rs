// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::{Object, OneOrMany, Url};

/// Information used to determine the current status of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialStatus {
    /// A Url identifying the credential status.
    pub id: Url,
    /// The type(s) of the credential status.
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    /// Additional properties of the credential status.
    #[serde(flatten)]
    pub properties: Object,
}

impl CredentialStatus {
    /// Creates a new [`CredentialStatus`].
    pub fn new<T>(id: Url, types: T) -> Self
    where
        T: Into<OneOrMany<String>>,
    {
        Self::with_properties(id, types, Object::new())
    }

    /// Creates a new [`CredentialStatus`] with the given `properties`.
    pub fn with_properties<T>(id: Url, types: T, properties: Object) -> Self
    where
        T: Into<OneOrMany<String>>,
    {
        Self {
            id,
            types: types.into(),
            properties,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert::FromJson as _, credential::CredentialStatus};

    const JSON: &str = include_str!("../../../tests/fixtures/vc/credential-status-1.json");

    #[test]
    #[rustfmt::skip]
    fn test_from_json() {
        let status: CredentialStatus = CredentialStatus::from_json(JSON).unwrap();
        assert_eq!(status.id, "https://example.edu/status/24");
        assert_eq!(status.types.as_slice(), ["CredentialStatusList2017"]);
    }
}
