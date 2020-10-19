use identity_core::did::DID;
use identity_iota::did::{create_address, create_diff_address};
use iota::transaction::bundled::BundledTransactionField;
use iota_conversion::Trinary;

/// test iota address creation from a DID
#[test]
fn test_create_did_address() {
    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456".into()],
        ..Default::default()
    }
    .init()
    .unwrap();
    let address = create_address(&did).unwrap();
    assert_eq!(
        address.to_inner().as_i8_slice().trytes().unwrap(),
        "VAWAXAYAZA9B999999999999999999999999999999999999999999999999999999999999999999999"
    );
}

/// test iota diff address creation from a public key
#[test]
fn test_create_diff_address() {
    let ed25519_public_key = "183e7cfbc21f62bfcd9b06fbed6a64c2585e6fe31828b589e48ef27e1a2c919c".to_string();
    let bs58_auth_key = bs58::encode(ed25519_public_key).into_string();

    let address = create_diff_address(&bs58_auth_key.as_bytes()).unwrap();

    assert_eq!(
        address.to_inner().as_i8_slice().trytes().unwrap(),
        "XAUBGCHCPCGDTBADEDFDGCEDADUBBCQBHDDDFDTCKBTCZBGDADBDBBACFCADVBVCBDTCDDXBLDLDIDHCG"
    );
}
