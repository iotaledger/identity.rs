use crate::common::{Object, OneOrMany, Url};

/// Information used to refresh or assert the status of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#refreshing)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RefreshService {
    pub id: Url,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl RefreshService {
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
    use crate::{convert::FromJson as _, vc::RefreshService};

    const JSON: &str = include_str!("../../../tests/fixtures/vc/refresh-service-1.json");

    #[test]
    #[rustfmt::skip]
    fn test_from_json() {
        let service: RefreshService = RefreshService::from_json(JSON).unwrap();
        assert_eq!(service.id, "https://example.edu/refresh/3732");
        assert_eq!(service.types.as_slice(), ["ManualRefreshService2018"]);
    }
}
