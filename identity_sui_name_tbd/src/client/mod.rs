mod full_client;
mod read_only;

pub use full_client::*;
pub use read_only::*;

use secret_storage::SignatureScheme;

pub struct IotaKeySignature {
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl SignatureScheme for IotaKeySignature {
    type PublicKey = Vec<u8>;
    type Signature = Vec<u8>;
}