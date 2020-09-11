use bs58::encode;
use iota_conversion::trytes_converter;
use multihash::Blake2b256;

/// Returns an address from a did segment
pub fn did_iota_address(did: &str) -> crate::Result<String> {
    let hash = Blake2b256::digest(did.as_bytes());
    let bs58key = encode(&hash.digest()).into_string();
    let trytes = match trytes_converter::to_trytes(&bs58key) {
        Ok(trytes) => trytes,
        _ => return Err(crate::Error::TryteConversionError),
    };
    Ok(trytes[0..81].into())
}
