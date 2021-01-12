use identity_core::{crypto::KeyPair, utils::encode_b58};
use libjose::{
    jose::JoseTokenType,
    jws::{Encoder, JwsAlgorithm, JwsFormat, JwsHeader},
};
use serde::Serialize;

use crate::{
    envelope::{EnvelopeExt, Plaintext},
    error::Result,
    message::Message,
};

/// Supported digital signature algorithms
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#algorithms)
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    EdDSA, // crv=Ed25519
    ES256,
    ES256K,
}

impl From<Algorithm> for JwsAlgorithm {
    fn from(other: Algorithm) -> Self {
        match other {
            Algorithm::EdDSA => Self::EdDSA,
            Algorithm::ES256 => Self::ES256,
            Algorithm::ES256K => Self::ES256K,
        }
    }
}

/// A DIDComm Signed Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message)
///
/// # Layout
///
///   `JWS(Plaintext)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope(String);

impl Envelope {
    pub fn from_message<T>(message: &Message<T>, algorithm: Algorithm, keypair: &KeyPair) -> Result<Self>
    where
        T: Serialize,
    {
        Plaintext::from_message(message).and_then(|plaintext| Self::from_plaintext(&plaintext, algorithm, keypair))
    }

    pub fn from_plaintext(envelope: &Plaintext, algorithm: Algorithm, keypair: &KeyPair) -> Result<Self> {
        let header: JwsHeader = {
            let mut header: JwsHeader = JwsHeader::new(algorithm.into());
            header.set_kid(encode_b58(keypair.public()));
            header.set_typ(JoseTokenType::JWM.name());
            header
        };

        Encoder::new()
            .format(JwsFormat::Compact)
            .recipient((keypair.secret().as_ref(), &header))
            .encode(envelope.as_bytes())
            .map_err(Into::into)
            .map(Self)
    }
}

impl EnvelopeExt for Envelope {
    const FEXT: &'static str = "dcsm";
    const MIME: &'static str = "application/didcomm-signed+json";

    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
