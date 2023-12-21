use std::error::Error;
use std::str::FromStr;

use identity_core::common::Context;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_credential::credential::Credential;
use identity_credential::credential::Issuer;
use identity_credential::revocation::status_list_2021::StatusList2021;
use identity_credential::revocation::status_list_2021::StatusList2021CredentialBuilder;
use identity_credential::revocation::status_list_2021::StatusList2021Entry;
use identity_credential::validator::JwtCredentialValidatorUtils;
use identity_credential::validator::JwtValidationError;
use identity_credential::validator::StatusCheck;

#[test]
fn status_list_2021_workflow() -> Result<(), Box<dyn Error>> {
  // Create a new revocation list
  let mut status_list_credential = StatusList2021CredentialBuilder::new(StatusList2021::default())
    .context(Context::Url(Url::from_str("https://www.w3.org/2018/credentials/v1")?))
    .issuer(Issuer::Url(Url::from_str("did:example:1234")?))
    .subject_id(Url::from_str("https://example.com/credentials/status")?)
    .build()?;

  // Create a credential
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
  // use entry 420 to revoke the credential
  status_list_credential.update_status_list(|status_list| status_list.set(420, true))?;
  let revocation_entry = serde_json::from_value::<StatusList2021Entry>(serde_json::json!({
    "id": "https://example.com/credentials/status#420",
    "type": "StatusList2021Entry",
    "statusPurpose": "revocation",
    "statusListIndex": "420",
    "statusListCredential": "https://example.com/credentials/status"
  }))?;
  credential.credential_status = Some(revocation_entry.into());

  let validation = JwtCredentialValidatorUtils::check_status_with_status_list_2021(
    &credential,
    &status_list_credential,
    StatusCheck::Strict,
  );
  assert!(validation.is_err_and(|e| matches!(e, JwtValidationError::Revoked)));

  Ok(())
}
