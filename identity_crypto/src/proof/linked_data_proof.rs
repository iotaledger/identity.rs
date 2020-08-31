use crate::{
  identity_core::{Object, Timestamp},
  proof::ProofOptions,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkedDataProof {
  #[serde(rename = "type")]
  pub type_: String,
  #[serde(rename = "verificationMethod")]
  pub verification_method: String,
  #[serde(rename = "proofPurpose")]
  pub proof_purpose: String,
  #[serde(rename = "proofValue")]
  pub proof_value: String,
  pub created: Timestamp,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
  #[serde(flatten)]
  pub properties: Object,
}

impl LinkedDataProof {
  pub fn to_options(&self) -> ProofOptions {
    ProofOptions {
      verification_method: self.verification_method.to_owned(),
      created: Some(self.created.to_owned()),
      proof_purpose: Some(self.proof_purpose.to_owned()),
      domain: self.domain.to_owned(),
      nonce: self.nonce.to_owned(),
    }
  }
}
