use crate::{
  error::Result,
  identity_core::{Object, Timestamp},
  proof::ProofDocument,
  utils::convert,
};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofOptions {
  pub verification_method: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proof_purpose: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
}

impl ProofDocument for ProofOptions {
  fn to_object(&self) -> Result<Object> {
    convert(self)
  }
}
