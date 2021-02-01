// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::error::Error;
use crate::error::Result;
use crate::signature::Sign;
use crate::signature::Signature;
use crate::signature::SignatureData;
use crate::signature::SignatureOptions;
use crate::signature::SuiteName;
use crate::signature::Verify;
use crate::verifiable::ResolveMethod;
use crate::verifiable::SetSignature;
use crate::verifiable::TrySignature;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;

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
  S: Sign + SuiteName,
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
  S: Verify + SuiteName,
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

    let query: MethodQuery<'_> = signature.to_query()?;
    let method: MethodWrap<'_, M> = resolver.try_resolve_method(query)?;

    if !S::METHODS.contains(&method.key_type()) {
      return Err(Error::UnknownMethodType);
    }

    signature.verify(&self.suite, message, &method.key_data().try_decode()?)?;

    Ok(())
  }
}
