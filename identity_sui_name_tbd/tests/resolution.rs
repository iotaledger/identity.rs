use common::get_client;
use identity_sui_name_tbd::migration;

mod common;

#[tokio::test]
async fn legacy_did_document_resolution_works() -> anyhow::Result<()> {
  let client = get_client().await?;
  let alias_id = client.create_legacy_did().await?;

  let resolved_alias = migration::get_alias(&client, alias_id.to_string().as_ref()).await?;
  assert!(resolved_alias.is_some());

  Ok(())
}

#[tokio::test]
async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
  let client = get_client().await?;
  let alias_id = client.create_legacy_did().await?;

  let (doc_id, _cap_id) = client.migrate_legacy_did(alias_id).await?;

  assert!(migration::get_alias(&client, alias_id.to_string().as_str())
    .await?
    .is_none());

  let resolved_id = migration::lookup(&client, alias_id)
    .await?
    .map(|doc| *doc.id.object_id())
    .unwrap();
  assert_eq!(resolved_id, doc_id);

  Ok(())
}
