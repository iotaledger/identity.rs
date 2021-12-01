// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::crypto::signature::errors::MissingSignatureError;
use crate::crypto::signature::errors::ProofValueError;
use crate::crypto::signature::errors::SigningError;
use crate::crypto::SetSignature;
use crate::crypto::Signature;
use crate::crypto::SignatureValue;
use crate::crypto::TrySignature;

/// A common interface for digital signature creation.
pub trait Sign {
  /// The private key type of this signature implementation.
  type Private: ?Sized;

  /// The output type of this signature implementation.
  type Output;

  /// Signs the given `message` with `key` and returns a digital signature.
  fn sign(message: &[u8], key: &Self::Private) -> Result<Self::Output, SigningError>;
}

// =============================================================================
// =============================================================================

/// A common interface for digital signature verification
pub trait Verify {
  /// The public key type of this signature implementation.
  type Public: ?Sized;

  /// Error describing how `verify` may fail
  type Error: std::error::Error + TryInto<ProofValueError>;

  /// Verifies the authenticity of `data` and `signature` with `key`.
  fn verify(message: &[u8], signature: &[u8], key: &Self::Public) -> Result<(), Self::Error>;
}

// =============================================================================
// =============================================================================

/// A common interface for named signature suites.
pub trait Named {
  /// A unique identifier for the signatures created by this suite.
  const NAME: &'static str;
}

// =============================================================================
// =============================================================================

/// A common interface for digital signature creation.
pub trait Signer<Secret: ?Sized>: Named {
  /// Error describing how `sign` may fail.
  type SignError: std::error::Error;

  /// Error describing how `create_signature` may fail
  type SignatureCreationError: From<Self::SignError> + From<MissingSignatureError> + std::error::Error;

  /// Signs the given `data` and returns a digital signature.
  fn sign<T>(data: &T, secret: &Secret) -> Result<SignatureValue, Self::SignError>
  where
    T: Serialize;

  /// Creates and applies a [signature][`Signature`] to the given `data`.
  fn create_signature<T>(
    data: &mut T,
    method: impl Into<String>,
    secret: &Secret,
  ) -> Result<(), Self::SignatureCreationError>
  where
    T: Serialize + SetSignature,
  {
    data.set_signature(Signature::new(Self::NAME, method));

    let value: SignatureValue = Self::sign(&data, secret)?;
    let write: &mut Signature = data.try_signature_mut()?;

    write.set_value(value);

    Ok(())
  }
}

// =============================================================================
// =============================================================================

/// A common interface for digital signature verification
pub trait Verifier<Public: ?Sized>: Named {
  /// Error describing how `verify` can fail
  type AuthenticityError: std::error::Error + TryInto<ProofValueError>;

  /// Error describing how `verify_signature` can fail
  type SignatureVerificationError: std::error::Error
    + From<MissingSignatureError>
    + From<Self::AuthenticityError>
    + From<ProofValueError>;

  /// Verifies the authenticity of `data` and `signature`.
  fn verify<T>(data: &T, signature: &SignatureValue, public: &Public) -> Result<(), Self::AuthenticityError>
  where
    T: Serialize;

  /// Extracts and verifies a [signature][`Signature`] from the given `data`.
  fn verify_signature<T>(data: &T, public: &Public) -> Result<(), Self::SignatureVerificationError>
  where
    T: Serialize + TrySignature,
  {
    let signature: &Signature = data.try_signature()?;

    if signature.type_() != Self::NAME {
      return Err(ProofValueError("signature name").into());
    }

    signature.hide_value();

    Self::verify(&data, signature.value(), public)?;

    signature.show_value();

    Ok(())
  }
}
