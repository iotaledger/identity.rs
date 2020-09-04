use identity_common::{Object, OneOrMany, Uri};
use std::convert::TryFrom;

use crate::{
    common::{try_take_object_id, try_take_object_types},
    error::Error,
};

/// Information used to determine the current status of a `Credential`.
///
/// Ref: https://www.w3.org/TR/vc-data-model/#status
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CredentialStatus {
    pub id: Uri,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for CredentialStatus {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        let mut this: Self = Default::default();

        this.id = try_take_object_id("CredentialStatus", &mut other)?.into();
        this.types = try_take_object_types("CredentialStatus", &mut other)?;
        this.properties = other;

        Ok(this)
    }
}
