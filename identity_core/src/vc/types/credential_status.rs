use crate::common::{Object, OneOrMany, Url};

/// Information used to determine the current status of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialStatus {
    pub id: Url,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl CredentialStatus {
    pub fn new<T>(id: Url, types: T) -> Self
    where
        T: Into<OneOrMany<String>>,
    {
        Self::with_properties(id, types, Object::new())
    }

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
    use crate::{convert::FromJson as _, vc::CredentialStatus};

    const JSON: &str = include_str!("../../../tests/fixtures/vc/credential-status-1.json");

    #[test]
    #[rustfmt::skip]
    fn test_from_json() {
        let status: CredentialStatus = CredentialStatus::from_json(JSON).unwrap();
        assert_eq!(status.id, "https://example.edu/status/24");
        assert_eq!(status.types.as_slice(), ["CredentialStatusList2017"]);
    }
}
