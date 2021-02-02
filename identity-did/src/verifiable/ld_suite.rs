// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use identity_core::crypto::SigSign;
use identity_core::crypto::Signature;
use identity_core::crypto::SignatureData;
use identity_core::crypto::SignatureOptions;
use identity_core::crypto::SigName;
use identity_core::crypto::SigVerify;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;
use crate::verifiable::ResolveMethod;
use crate::verifiable::SetSignature;
use crate::verifiable::TrySignature;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;
use crate::verification::MethodType;

pub trait LdSignature: SigName {
  const METHODS: &'static [MethodType];
}

#[derive(Clone, Copy, Debug)]
pub struct LdSuite<S> {
  suite: S,
}

impl<S> LdSuite<S> {
  pub fn new(suite: S) -> Self {
    Self { suite }
  }
}

impl<S> LdSuite<S>
where
  S: SigSign + LdSignature,
{
  pub fn sign<T, K>(&self, message: &mut T, options: SignatureOptions, secret: &K) -> Result<()>
  where
    T: Serialize + SetSignature,
    K: AsRef<[u8]> + ?Sized,
  {
    message.set_signature(Signature::new(self.suite.name(), options));

    let value: SignatureData = self.suite.sign(message, secret.as_ref())?;

    message.try_signature_mut()?.set_data(value);

    Ok(())
  }
}

impl<S> LdSuite<S>
where
  S: SigVerify + LdSignature,
{
  pub fn verify<T, M>(&self, message: &T) -> Result<()>
  where
    T: Serialize + TrySignature + ResolveMethod<M>,
    M: Serialize,
  {
    self.verify_data(message, message)
  }

  pub fn verify_data<T, R, M>(&self, message: &T, resolver: R) -> Result<()>
  where
    T: Serialize + TrySignature,
    R: ResolveMethod<M>,
    M: Serialize,
  {
    let signature: &Signature = message.try_signature()?;

    if signature.type_() != self.suite.name() {
      return Err(Error::UnknownSignatureType);
    }

    let query: MethodQuery<'_> = signature.try_into()?;
    let method: MethodWrap<'_, M> = resolver.try_resolve_method(query)?;

    if !S::METHODS.contains(&method.key_type()) {
      return Err(Error::UnknownMethodType);
    }

    let public: Vec<u8> = method.key_data().try_decode()?;

    signature.verify(&self.suite, message, &public)?;

    Ok(())
  }
}
