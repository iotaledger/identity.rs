use bzip2::Compression;
use bzip2::read::{BzDecoder, BzEncoder};

use std::io::Read;
use crate::Error;
use crate::Result;


// String, &str, Vec<u8>, &[u8]
pub fn compress_bzip2<T: AsRef<[u8]>>(input: &T) -> Result<Vec<u8>> {
    let mut compressor = BzEncoder::new(input.as_ref(), Compression::best());
    let mut bytes: Vec<u8> = Vec::new();
    let res = compressor.read_to_end(&mut bytes).map_err(|e| {
        return Error::CompressionError;
    });
    return Ok(bytes);
}

pub fn decompress_bzip2<T: AsRef<[u8]>>(input: &T) -> String {
    let mut decompressor = BzDecoder::new(input.as_ref());
    let mut s = String::new();
    decompressor.read_to_string(&mut s).unwrap();
    return s;
}

#[cfg(test)]
mod test {
    use identity_core::convert::ToJson;
    use identity_core::crypto::KeyPair;
    use crate::did::IotaDocument;
    use super::*;


    #[test]
    fn test_compress() {
        let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
        let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
        document.sign(&keypair.private()).unwrap();
        let data = document.to_json().unwrap();
        let compressed = compress_bzip2(&data).unwrap();
        let size_before = data.as_str().as_bytes().len();
        let size_after = compressed.len();
        let ratio: f64 = size_after as f64 / size_before as f64;
        let compressed_ratio: f64 = 1.0 - ratio;
        println!("Before: {}\nAfter: {}\nRatio: {}\nCompressed Ratio: {}", size_before, size_after, ratio, compressed_ratio);

        let sourceData = decompress_bzip2(&compressed);
        println!("original message: {}", sourceData);
    }
}

