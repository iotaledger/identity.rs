use bs58::encode;
use identity_core::did::DID;
use iota_conversion::trytes_converter;
use multihash::Blake2b256;

/// Creates an 81 Trytes IOTA address from the DID
pub fn get_iota_address(did: &DID) -> crate::Result<String> {
    let iota_specific_idstring = did.id_segments.last().expect("Failed to get id_segment");
    let hash = Blake2b256::digest(iota_specific_idstring.as_bytes());
    let bs58key = encode(&hash.digest()).into_string();
    let trytes = match trytes_converter::to_trytes(&bs58key) {
        Ok(trytes) => trytes,
        _ => return Err(crate::Error::TryteConversionError),
    };
    Ok(trytes[0..81].into())
}
