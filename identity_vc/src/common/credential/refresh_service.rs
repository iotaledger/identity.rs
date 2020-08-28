use std::convert::TryFrom;

use crate::{
  common::{take_object_id, take_object_type, Object, OneOrMany, URI},
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

    this.id = match take_object_id(&mut other) {
      Some(id) => id.into(),
      None => return Err(Error::BadObjectConversion("RefreshService")),
    };

    this.types = match take_object_type(&mut other) {
      Some(types) => types,
      None => return Err(Error::BadObjectConversion("RefreshService")),
    };

    this.properties = other;

    Ok(this)
  }
}
