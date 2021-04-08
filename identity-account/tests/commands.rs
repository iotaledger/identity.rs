use identity_account::account::Account;
use identity_account::account::AutoSave;
use identity_account::chain::ChainKey;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::events::Snapshot;
use identity_account::storage::MemStore;
use identity_account::types::ChainId;
use identity_account::types::Index;
use identity_account::types::Timestamp;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;

async fn new_account() -> Result<Account<MemStore>> {
  Account::new(MemStore::new()).await
}

fn assert_chain(state: &Snapshot, chain: ChainId) {
  assert_eq!(state.chain(), chain);
}

fn assert_version(state: &Snapshot, version: Index) {
  assert_eq!(state.version(), version);
}

fn assert_document(state: &Snapshot, some: bool) {
  assert_eq!(state.state().document().is_some(), some);
}

fn assert_timestamps(state: &Snapshot, fresh: bool) {
  assert_eq!(state.state().created() == Timestamp::EPOCH, fresh);
  assert_eq!(state.state().updated() == Timestamp::EPOCH, fresh);
}

#[tokio::test]
async fn test_create_chain() -> Result<()> {
  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();
  let state: Snapshot = account.snapshot(chain).await?;

  assert_version(&state, Index::ZERO);
  assert_chain(&state, chain);
  assert_document(&state, false);
  assert_timestamps(&state, true);

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let state: Snapshot = account.snapshot(chain).await?;

  assert_version(&state, Index::from(2));
  assert_chain(&state, chain);
  assert_document(&state, true);
  assert_timestamps(&state, false);

  Ok(())
}

#[tokio::test]
async fn test_create_method() -> Result<()> {
  const FRAG: &str = "test-frag";

  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment(FRAG.to_string())
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let state: Snapshot = account.snapshot(chain).await?;

  assert_version(&state, Index::from(4));
  assert_chain(&state, chain);
  assert_document(&state, true);
  assert_timestamps(&state, false);

  let methods = state.state().methods();

  assert_eq!(methods.len(), 1);

  let data: &(MethodScope, ChainKey, _) = methods.iter().find(|(_, key, _)| key.fragment() == FRAG).unwrap();

  assert_eq!(data.0, MethodScope::VerificationMethod);
  assert_eq!(data.1.type_(), MethodType::Ed25519VerificationKey2018);
  assert_eq!(data.1.fragment(), FRAG);

  Ok(())
}
