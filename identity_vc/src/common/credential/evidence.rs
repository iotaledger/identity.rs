use std::convert::TryFrom;

use crate::{
  common::{take_object_id, take_object_type, Object, OneOrMany},
  error::Error,
};

/// Information used to increase confidence in the claims of a `Credential`
///
/// Ref: https://www.w3.org/TR/vc-data-model/#evidence
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Evidence {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  #[serde(flatten)]
  pub properties: Object,
}

impl TryFrom<Object> for Evidence {
  type Error = Error;

  fn try_from(mut other: Object) -> Result<Self, Self::Error> {
    let mut this: Self = Default::default();

    this.id = take_object_id(&mut other);

    this.types = match take_object_type(&mut other) {
      Some(types) => types,
      None => return Err(Error::BadObjectConversion("Evidence")),
    };

    this.properties = other;

    Ok(this)
  }
}
