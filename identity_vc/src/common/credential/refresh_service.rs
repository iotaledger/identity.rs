use identity_core::common::Object;
use std::convert::TryFrom;

use crate::{
    common::{try_take_object_id, try_take_object_types, OneOrMany, URI},
    error::Error,
};

/// Information used to refresh or assert the status of a `Credential`.
///
/// Ref: https://www.w3.org/TR/vc-data-model/#refreshing
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RefreshService {
    pub id: URI,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for RefreshService {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        let mut this: Self = Default::default();

        this.id = try_take_object_id("RefreshService", &mut other)?.into();
        this.types = try_take_object_types("RefreshService", &mut other)?;
        this.properties = other;

        Ok(this)
    }
}
