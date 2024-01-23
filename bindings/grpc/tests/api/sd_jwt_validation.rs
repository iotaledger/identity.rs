use _sd_jwt::verification_client::VerificationClient;
use _sd_jwt::KeyBindingOptions;
use _sd_jwt::VerificationRequest;
use identity_iota::core::FromJson;
use identity_iota::core::Timestamp;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::Jws;
use identity_iota::credential::Subject;
use identity_iota::did::DID;
use identity_iota::sd_jwt_payload::KeyBindingJwtClaims;
use identity_iota::sd_jwt_payload::SdJwt;
use identity_iota::sd_jwt_payload::SdObjectEncoder;
use identity_iota::sd_jwt_payload::Sha256Hasher;
use identity_storage::JwkDocumentExt;
use identity_storage::JwsSignatureOptions;

use crate::helpers::Entity;
use crate::helpers::TestServer;

mod _sd_jwt {
  tonic::include_proto!("sd_jwt");
}

#[tokio::test]
async fn sd_jwt_validation_works() -> anyhow::Result<()> {
  let server = TestServer::new().await;
  let client = server.client();
  let mut issuer = Entity::new();
  issuer.create_did(client).await?;

  let mut holder = Entity::new();
  holder.create_did(client).await?;

  // Create an address credential subject.
  let subject = Subject::from_json_value(serde_json::json!({
    "id": holder.document().unwrap().id().as_str(),
    "name": "Alice",
    "address": {
      "locality": "Maxstadt",
      "postal_code": "12344",
      "country": "DE",
      "street_address": "Weidenstraße 22"
    }
  }))?;
  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.com/credentials/3732")?)
    .issuer(Url::parse(issuer.document().unwrap().id().as_str())?)
    .type_("AddressCredential")
    .subject(subject)
    .build()?;

  // In Order to create an selective disclosure JWT, the plain text JWT
  // claims set must be created first.
  let payload = credential.serialize_jwt(None)?;

  // Using the crate `sd-jwt` properties of the claims can be made selectively disclosable.
  // The default sha-256 hasher will be used to create the digests.
  // Read more in https://github.com/iotaledger/sd-jwt-payload .
  let mut encoder = SdObjectEncoder::new(&payload)?;

  // Make "locality", "postal_code" and "street_address" selectively disclosable while keeping
  // other properties in plain text.
  let disclosures = vec![
    encoder.conceal(&["vc", "credentialSubject", "address", "locality"], None)?,
    encoder.conceal(&["vc", "credentialSubject", "address", "postal_code"], None)?,
    encoder.conceal(&["vc", "credentialSubject", "address", "street_address"], None)?,
  ];

  // Add the `_sd_alg` property.
  encoder.add_sd_alg_property();
  let encoded_payload = encoder.try_to_string()?;

  // Create the signed JWT.
  let jwt: Jws = issuer
    .document()
    .unwrap()
    .create_jws(
      issuer.storage(),
      issuer.fragment().unwrap(),
      encoded_payload.as_bytes(),
      &JwsSignatureOptions::default(),
    )
    .await?;

  // One way to send the JWT and the disclosures, is by creating an SD-JWT with all the
  // disclosures.
  let disclosures: Vec<String> = disclosures
    .into_iter()
    .map(|disclosure| disclosure.to_string())
    .collect();
  let sd_jwt_str = SdJwt::new(jwt.into(), disclosures, None).presentation();

  const VERIFIER_DID: &str = "did:example:verifier";
  // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
  let nonce: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";

  // ===========================================================================
  // Step 5: Holder creates an SD-JWT to be presented to a verifier.
  // ===========================================================================

  let sd_jwt = SdJwt::parse(&sd_jwt_str)?;

  // The holder only wants to present "locality" and "postal_code" but not "street_address".
  let disclosures = vec![
    sd_jwt.disclosures.first().unwrap().clone(),
    sd_jwt.disclosures.get(1).unwrap().clone(),
  ];

  // Optionally, the holder can add a Key Binding JWT (KB-JWT). This is dependent on the verifier's policy.
  // Issuing the KB-JWT is done by creating the claims set and setting the header `typ` value
  // with the help of `KeyBindingJwtClaims`.
  let binding_claims = KeyBindingJwtClaims::new(
    &Sha256Hasher::new(),
    sd_jwt.jwt.as_str().to_string(),
    disclosures.clone(),
    nonce.to_string(),
    VERIFIER_DID.to_string(),
    Timestamp::now_utc().to_unix(),
  )
  .to_json()?;

  // Setting the `typ` in the header is required.
  let options = JwsSignatureOptions::new().typ(KeyBindingJwtClaims::KB_JWT_HEADER_TYP);

  // Create the KB-JWT.
  let kb_jwt: Jws = holder
    .document()
    .unwrap()
    .create_jws(
      holder.storage(),
      holder.fragment().unwrap(),
      binding_claims.as_bytes(),
      &options,
    )
    .await?;

  // Create the final SD-JWT.
  let sd_jwt_obj = SdJwt::new(sd_jwt.jwt, disclosures, Some(kb_jwt.into()));

  // Holder presents the SD-JWT to the verifier.
  let sd_jwt_presentation: String = sd_jwt_obj.presentation();

  // Verify the JWT.
  let mut sd_jwt_verification_client = VerificationClient::connect(server.endpoint()).await?;
  let _ = sd_jwt_verification_client
    .verify(VerificationRequest {
      jwt: sd_jwt_presentation,
      kb_options: Some(KeyBindingOptions {
        nonce: Some(nonce.to_owned()),
        aud: Some(VERIFIER_DID.to_owned()),
        holder_did: holder.document().unwrap().id().to_string(),
        ..Default::default()
      }),
    })
    .await?;

  Ok(())
}
