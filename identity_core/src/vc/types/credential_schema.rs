use crate::common::{Object, OneOrMany, Url};

/// Information used to validate the structure of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#data-schemas)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CredentialSchema {
    pub id: Url,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl CredentialSchema {
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
    use crate::{convert::FromJson as _, vc::CredentialSchema};

    const JSON1: &str = include_str!("../../../tests/fixtures/vc/credential-schema-1.json");
    const JSON2: &str = include_str!("../../../tests/fixtures/vc/credential-schema-2.json");
    const JSON3: &str = include_str!("../../../tests/fixtures/vc/credential-schema-3.json");

    #[test]
    #[rustfmt::skip]
    fn test_from_json() {
        let schema: CredentialSchema = CredentialSchema::from_json(JSON1).unwrap();
        assert_eq!(schema.id, "https://example.org/examples/degree.json");
        assert_eq!(schema.types.as_slice(), ["JsonSchemaValidator2018"]);

        let schema: CredentialSchema = CredentialSchema::from_json(JSON2).unwrap();
        assert_eq!(schema.id, "https://example.org/examples/degree.zkp");
        assert_eq!(schema.types.as_slice(), ["ZkpExampleSchema2018"]);

        let schema: CredentialSchema = CredentialSchema::from_json(JSON3).unwrap();
        assert_eq!(schema.id, "did:example:cdf:35LB7w9ueWbagPL94T9bMLtyXDj9pX5o");
        assert_eq!(schema.types.as_slice(), ["did:example:schema:22KpkXgecryx9k7N6XN1QoN3gXwBkSU8SfyyYQG"]);
    }
}
