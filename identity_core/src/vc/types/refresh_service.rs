use core::convert::TryFrom;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{Object, OneOrMany, Url},
    error::Error,
};

/// Information used to refresh or assert the status of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#refreshing)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Builder)]
pub struct RefreshService {
    #[builder(setter(into))]
    pub id: Url,
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    #[builder(default, setter(into))]
    pub properties: Object,
}

impl TryFrom<Object> for RefreshService {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.try_take_object_id()?.into(),
            types: other.try_take_object_types()?,
            properties: other,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "`id` must be initialized"]
    fn test_builder_missing_id() {
        RefreshServiceBuilder::default()
            .types("my-type".to_string())
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic = "`types` must be initialized"]
    fn test_builder_missing_types() {
        RefreshServiceBuilder::default().id("did:test").build().unwrap();
    }
}
