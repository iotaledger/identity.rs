use anyhow::Result;
use identity_account::identity_state::State;
use identity_core::crypto::KeyPair;
use identity_iota::did::IotaDocument;
use std::str::FromStr;

#[test]
fn fs() -> Result<()> {
    let filename = "test";
    let (document, keypair) = create()?;
    let state = State::new(keypair.clone(), document.clone())?;
    state.write_to_file(filename)?;
    let read_state = State::read_from_file(filename)?;
    assert_eq!(state, read_state);
    let state_string = read_state.to_string();
    let state_from_str = State::from_str(&state_string)?;
    assert_eq!(state, state_from_str);
    Ok(())
}

pub fn create() -> Result<(IotaDocument, KeyPair)> {
    // Create keypair/DID document
    let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", "main", None)?;

    // Sign the document with the authentication method secret
    document.sign(keypair.secret())?;

    // Ensure the document proof is valid
    assert!(document.verify().is_ok());
    Ok((document, keypair))
}
