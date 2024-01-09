use std::str::FromStr;

use identity_iota::{credential::{status_list_2021::{StatusList2021, StatusList2021CredentialBuilder, StatusList2021Entry}, Issuer, Credential, JwtCredentialValidatorUtils, StatusCheck, JwtValidationError}, core::{Url, Context, Timestamp}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new status list to be stored off-chain, for the sake of this example
  // its going to stay in memory.
  let mut status_list_credential = StatusList2021CredentialBuilder::new(StatusList2021::default())
    .context(Context::Url(Url::from_str("https://www.w3.org/2018/credentials/v1")?))
    .issuer(Issuer::Url(Url::from_str("did:example:1234")?))
    .subject_id(Url::from_str("https://example.com/credentials/status")?)
    .build()?;

  // Let's revoke a credential using this status list.
  // First we create a credential.
  let mut credential = serde_json::from_value::<Credential>(serde_json::json!({
      "@context": "https://www.w3.org/2018/credentials/v1",
      "id": "https://example.com/credentials/12345678",
      "type": ["VerifiableCredential"],
      "issuer": "did:example:1234",
      "issuanceDate": format!("{}", Timestamp::now_utc()),
      "credentialSubject": {
          "id": "did:example:4321",
          "type": "UniversityDegree",
          "gpa": "4.0",
      }
  }))?;

  // We add to this credential a status which references the 420th entry
  // in the status list we previously created.
  let revocation_entry = serde_json::from_value::<StatusList2021Entry>(serde_json::json!({
    "id": "https://example.com/credentials/status#420",
    "type": "StatusList2021Entry",
    "statusPurpose": "revocation",
    "statusListIndex": "420",
    "statusListCredential": "https://example.com/credentials/status"
  }))?;
  credential.credential_status = Some(revocation_entry.into());

  // To revoke this credential we set the status of the 420th entry
  status_list_credential.update_status_list(|status_list| status_list.set(420, true))?;

  // The credential has now been revoked, verifying this credential will now fail
  let validation = JwtCredentialValidatorUtils::check_status_with_status_list_2021(
    &credential,
    &status_list_credential,
    StatusCheck::Strict,
  );
  assert!(validation.is_err_and(|e| matches!(e, JwtValidationError::Revoked)));

  Ok(())
}
