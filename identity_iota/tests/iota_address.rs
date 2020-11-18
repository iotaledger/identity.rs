use identity_iota::did::{IotaDID, IotaDocument};
use iota::transaction::bundled::BundledTransactionField;
use iota_conversion::Trinary;

/// test iota address creation from a DID
#[test]
fn test_create_did_address() {
    let did = IotaDID::parse("did:iota:com:HbuRS48djS5PbLQciy6iE9BTdaDTBM3GxcbGdyuv3TWo").unwrap();
    let address = did.create_address().unwrap();
    assert_eq!(
        address.to_inner().as_i8_slice().trytes().unwrap(),
        "RBQCIDACBCYABBSCYCBCZAZBQCVB9CRCXCMD9BXCOBCBLBCCSCPCNBCCLBWBXAQBLDRCQCQBSCMDIDJDX"
    );
}

/// test iota diff address creation from a public key
#[test]
fn test_create_diff_address() {
    let ed25519_public_key = "183e7cfbc21f62bfcd9b06fbed6a64c2585e6fe31828b589e48ef27e1a2c919c".to_string();
    let bs58_auth_key = bs58::encode(ed25519_public_key).into_string();

    let address = IotaDocument::create_diff_address(&bs58_auth_key.as_bytes()).unwrap();

    assert_eq!(
        address.to_inner().as_i8_slice().trytes().unwrap(),
        "XAUBGCHCPCGDTBADEDFDGCEDADUBBCQBHDDDFDTCKBTCZBGDADBDBBACFCADVBVCBDTCDDXBLDLDIDHCG"
    );
}
