use identity_account::account::Account;
use identity_account::chain::ChainKey;
use identity_account::chain::TinyMethod;
use identity_account::error::Error;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::events::CommandError;
use identity_account::events::Snapshot;
use identity_account::storage::MemStore;
use identity_account::types::ChainId;
use identity_account::types::Index;
use identity_account::types::Timestamp;
use identity_did::verification::MethodType;

async fn new_account() -> Result<Account<MemStore>> {
  Account::new(MemStore::new()).await
}

#[track_caller]
fn assert_chain(state: &Snapshot, chain: ChainId) {
  assert_eq!(state.chain(), chain);
}

#[track_caller]
fn assert_version(state: &Snapshot, version: Index) {
  assert_eq!(state.version(), version);
}

#[track_caller]
fn assert_document(state: &Snapshot, some: bool) {
  assert_eq!(state.state().document().is_some(), some);
}

#[track_caller]
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
async fn test_create_chain_invalid_method() -> Result<()> {
  const TYPES: &[MethodType] = &[MethodType::MerkleKeyCollection2021];

  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  // initial snapshot version = 0
  assert_version(&account.snapshot(chain).await?, Index::ZERO);

  for type_ in TYPES.iter().copied() {
    let command: Command = Command::create_chain().authentication(type_).finish().unwrap();
    let output: _ = account.process(chain, command, false).await;

    assert!(matches!(
      output.unwrap_err(),
      Error::CommandError(CommandError::InvalidMethodType(_))
    ));

    // version is still 0, no events have been committed
    assert_version(&account.snapshot(chain).await?, Index::ZERO);
  }

  Ok(())
}

#[tokio::test]
async fn test_create_chain_already_exists() -> Result<()> {
  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  // initial snapshot version = 0
  assert_version(&account.snapshot(chain).await?, Index::ZERO);

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command.clone(), false).await?;

  // version is now 2
  assert_version(&account.snapshot(chain).await?, Index::from(2));

  let output: _ = account.process(chain, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::CommandError(CommandError::DocumentAlreadyExists),
  ));

  // version is still 2, no new events have been committed
  assert_version(&account.snapshot(chain).await?, Index::from(2));

  Ok(())
}

#[tokio::test]
async fn test_create_method() -> Result<()> {
  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let state: Snapshot = account.snapshot(chain).await?;

  assert_version(&state, Index::from(4));
  assert_chain(&state, chain);
  assert_document(&state, true);
  assert_timestamps(&state, false);

  assert_eq!(state.state().methods().len(), 2);

  let data: &TinyMethod = state.state().methods().fetch("key-1")?;

  assert_eq!(data.location().type_(), MethodType::Ed25519VerificationKey2018);
  assert_eq!(data.location().fragment(), "key-1");

  Ok(())
}

#[tokio::test]
async fn test_create_method_reserved_fragment() -> Result<()> {
  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment(ChainKey::AUTH.to_string())
    .finish()
    .unwrap();

  // version is now 2
  assert_version(&account.snapshot(chain).await?, Index::from(2));

  let output: _ = account.process(chain, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::CommandError(CommandError::InvalidMethodFragment(_)),
  ));

  // version is still 2, no new events have been committed
  assert_version(&account.snapshot(chain).await?, Index::from(2));

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() -> Result<()> {
  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .finish()
    .unwrap();

  assert_version(&account.snapshot(chain).await?, Index::from(2));
  account.process(chain, command.clone(), false).await?;
  assert_version(&account.snapshot(chain).await?, Index::from(4));

  let output: _ = account.process(chain, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::CommandError(CommandError::DuplicateKeyFragment(_)),
  ));

  assert_version(&account.snapshot(chain).await?, Index::from(4));

  Ok(())
}

#[tokio::test]
async fn test_delete_method() -> Result<()> {
  let account: Account<MemStore> = new_account().await?;
  let chain: ChainId = account.index()?.next_id();

  let command: Command = Command::create_chain()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(chain, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .finish()
    .unwrap();

  assert_version(&account.snapshot(chain).await?, Index::from(2));

  account.process(chain, command, false).await?;

  let state: Snapshot = account.snapshot(chain).await?;

  assert_version(&state, Index::from(4));
  assert_eq!(state.state().methods().len(), 2);
  assert_eq!(state.state().methods().contains("key-1"), true);
  assert_eq!(state.state().methods().get("key-1").is_some(), true);

  let command: Command = Command::delete_method().fragment("key-1").finish().unwrap();

  account.process(chain, command, false).await?;

  let state: Snapshot = account.snapshot(chain).await?;

  assert_version(&state, Index::from(6));
  assert_eq!(state.state().methods().len(), 1);
  assert_eq!(state.state().methods().contains("key-1"), false);
  assert_eq!(state.state().methods().get("key-1").is_none(), true);

  Ok(())
}
