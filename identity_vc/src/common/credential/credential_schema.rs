use identity_core::common::Object;
use std::convert::TryFrom;

use crate::{
    common::{try_take_object_id, try_take_object_type, URI},
    error::Error,
};

/// Information used to validate the structure of a `Credential`.
///
/// Ref: https://www.w3.org/TR/vc-data-model/#data-schemas
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CredentialSchema {
    pub id: URI,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for CredentialSchema {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        let mut this: Self = Default::default();

        this.id = try_take_object_id("CredentialSchema", &mut other)?.into();
        this.type_ = try_take_object_type("CredentialSchema", &mut other)?;
        this.properties = other;

        Ok(this)
    }
}
