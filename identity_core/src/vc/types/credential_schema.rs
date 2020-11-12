use core::convert::TryFrom;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{Object, Url},
    error::Error,
};

/// Information used to validate the structure of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#data-schemas)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Builder)]
pub struct CredentialSchema {
    #[builder(setter(into))]
    pub id: Url,
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub type_: String,
    #[serde(flatten)]
    #[builder(default, setter(into))]
    pub properties: Object,
}

impl TryFrom<Object> for CredentialSchema {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.try_take_object_id()?.into(),
            type_: other.try_take_object_type()?,
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
        CredentialSchemaBuilder::default().type_("my-type").build().unwrap();
    }

    #[test]
    #[should_panic = "`type_` must be initialized"]
    fn test_builder_missing_type() {
        CredentialSchemaBuilder::default().id("did:test").build().unwrap();
    }
}
