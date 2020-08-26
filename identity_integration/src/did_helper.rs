use blake2::{Blake2s, Digest};
use iota::ternary::TryteBuf;
use std::convert::TryInto;

/// Returns an address from a did segment
pub fn did_iota_address(did: &str) -> String {
    let mut hasher = Blake2s::new();
    hasher.update(did.as_bytes());
    let hash = hasher.finalize();
    let duplicated = [hash, hash].concat();
    // for bee byte-tryte conversion
    // let i8slice: &[i8] = unsafe { std::slice::from_raw_parts(duplicated.as_ptr() as *const i8, duplicated.len()) };
    let trytes = bytes_to_trytes(&duplicated);
    trytes.to_string()[0..81].to_string()
}

/// Converts a sequence of bytes to trytes
pub fn bytes_to_trytes(input: &[u8]) -> TryteBuf {
    let mut trytes = TryteBuf::with_capacity(input.len() * 2);
    for byte in input {
        let first: i8 = match (byte % 27) as i8 {
            b @ 0..=13 => b,
            b @ 14..=26 => b - 27,
            _ => unreachable!(),
        };
        let second = match (byte / 27) as i8 {
            b @ 0..=13 => b,
            b @ 14..=26 => b - 27,
            _ => unreachable!(),
        };

        trytes.push(first.try_into().expect("Tryteconversion failed"));
        trytes.push(second.try_into().expect("Tryteconversion failed"));
    }
    trytes
}
