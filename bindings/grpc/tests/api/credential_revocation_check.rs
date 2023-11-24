use credentials::{credential_revocation_client::CredentialRevocationClient, RevocationStatus};
use identity_iota::{
  credential::{self, RevocationBitmap, RevocationBitmapStatus, StatusCheck},
  did::DID,
};

use crate::{
  credential_revocation_check::credentials::RevocationCheckRequest,
  helpers::{Entity, TestServer},
};

mod credentials {
  tonic::include_proto!("credentials");
}

#[tokio::test]
async fn checking_status_of_valid_credential_works() -> anyhow::Result<()> {
  let server = TestServer::new().await;
  let client = server.client();
  let mut issuer = Entity::new();
  issuer.create_did(client).await?;

  let mut subject = Entity::new();
  subject.create_did(client).await?;

  let service_id = issuer
    .document()
    .unwrap() // Safety: `create_did` didn't fail
    .id()
    .to_url()
    .join("#my-revocation-service")?;

  // Add a revocation service to the issuer's DID document
  issuer
    .update_document(client, |mut doc| {
      let service = RevocationBitmap::new().to_service(service_id.clone()).unwrap();

      doc.insert_service(service).ok().map(|_| doc)
    })
    .await?;

  let credential_status: credential::Status = RevocationBitmapStatus::new(service_id, 3).into();

  let mut grpc_client = CredentialRevocationClient::connect(server.endpoint()).await?;
  let req = RevocationCheckRequest {
    r#type: credential_status.type_,
    url: credential_status.id.into_string(),
    properties: credential_status
      .properties
      .into_iter()
      .map(|(k, v)| (k, v.to_string().trim_matches('"').to_owned()))
      .collect(),
  };
  dbg!(&req);
  let res = grpc_client.check(tonic::Request::new(req)).await?.into_inner();

  assert_eq!(res.status(), RevocationStatus::Valid);

  Ok(())
}
