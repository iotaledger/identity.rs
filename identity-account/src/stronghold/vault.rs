// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::slip10::{Chain, ChainCode};
use engine::vault::RecordId;
use iota_stronghold::{Location, StrongholdResult};
use iota_stronghold::RecordHint;
use iota_stronghold::StrongholdFlags;
use iota_stronghold::VaultFlags;
use std::path::Path;
use iota_stronghold::procedures::{BIP39Generate, BIP39Recover, Ed25519Sign, GenerateSecret, MnemonicLanguage, Procedure, Slip10Generate, Slip10Derive, StrongholdProcedure};

use crate::error::Error;
use crate::error::PleaseDontMakeYourOwnResult;
use crate::error::Result;
use crate::stronghold::{Context, IotaStrongholdResult, StrongholdError};
use crate::stronghold::ProcedureResult;
use crate::stronghold::ProcedureResult::Ed25519PublicKey;

pub type Record = (RecordId, RecordHint);

#[derive(Debug)]
pub struct Vault<'snapshot> {
  path: &'snapshot Path,
  name: Vec<u8>,
  flags: Vec<StrongholdFlags>,
}

impl<'snapshot> Vault<'snapshot> {
  pub(crate) fn new<P, T>(path: &'snapshot P, name: &T, flags: &[StrongholdFlags]) -> Self
  where
    P: AsRef<Path> + ?Sized,
    T: AsRef<[u8]> + ?Sized,
  {
    Self {
      path: path.as_ref(),
      name: name.as_ref().to_vec(),
      flags: flags.to_vec(),
    }
  }
}

impl Vault<'_> {
  /// Returns the snapshot path of the vault.
  pub fn path(&self) -> &Path {
    self.path
  }

  /// Returns the name of the vault.
  pub fn name(&self) -> &[u8] {
    &self.name
  }

  /// Returns the vault policy options.
  pub fn flags(&self) -> &[StrongholdFlags] {
    &self.flags
  }

  /// Inserts a record.
  pub async fn insert<T>(&self, location: Location, payload: T, hint: RecordHint, flags: &[VaultFlags]) -> IotaStrongholdResult<()>
  where
    T: Into<Vec<u8>>,
  {
    Context::scope(self.path, &self.name, &self.flags)
      .await?
      .write_to_vault(location, payload.into(), hint, flags.to_vec())
      .await?;
    Ok(())
  }

  /// Deletes a record.
  pub async fn delete(&self, location: Location, gc: bool) -> IotaStrongholdResult<()> {
    Context::scope(self.path, &self.name, &self.flags)
      .await?
      .delete_data(location, gc)
      .await?;
    Ok(())
  }

  /// Returns true if the specified location exists.
  pub async fn exists(&self, location: Location) -> IotaStrongholdResult<bool> {
    let scope: _ = Context::scope(self.path, &self.name, &self.flags).await?;
    scope.vault_exists(location.vault_path()).await.map_err(StrongholdError::StrongholdActorError)
  }

  /// Runs the Stronghold garbage collector.
  pub async fn garbage_collect(&self, vault: &[u8]) -> IotaStrongholdResult<()> {
    Context::scope(self.path, &self.name, &self.flags)
      .await?
      .garbage_collect(vault.to_vec())
      .await?; //ToDo use return value
    Ok(())
  }
  // execute<Output>(&self, procedure: impl Procedure<Output = Output> -> Result<Output, Error>
  // execute<P: Procedure>(&self, P) -> Result<<P as Procedure>::Output, Error>

  /// Executes a runtime [`procedure`][`Procedure`].
  pub async fn execute<P>(&self, procedure: P) -> IotaStrongholdResult<P::Output> where
    P: Procedure + Into<StrongholdProcedure>,
  {
    let result = Context::scope(self.path, &self.name, &self.flags)
      .await?
      .runtime_exec(procedure)
      .await??;
    Ok(result)
  }

  /// Returns a list of available records and hints.
  pub async fn records<T>(&self, vault: &T) -> IotaStrongholdResult<Vec<(RecordId, RecordHint)>>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    let scope: _ = Context::scope(self.path, &self.name, &self.flags).await?;
    let a = scope.list_hints_and_ids(vault.as_ref()).await?;
    Ok(a)
    // let (data, status): (Vec<Record>, _) = scope.list_hints_and_ids(vault.as_ref()).await?;

    // status.to_result()?;

    // Ok(data)
  }

  // pub async fn slip10_generate(&self, output: Location, hint: RecordHint, bytes: Option<usize>) -> Result<()> {
  //   let procedure: Slip10Generate = Slip10Generate {
  //     output,
  //     hint,
  //     size_bytes: bytes,
  //   };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::SLIP10Generate => Ok(()),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }
  //
  // pub async fn slip10_derive(
  //   &self,
  //   chain: Chain,
  //   input: SLIP10DeriveInput,
  //   output: Location,
  //   hint: RecordHint,
  // ) -> Result<ChainCode> {
  //   let procedure = Slip10Derive {
  //     chain,
  //     input,
  //     output,
  //     hint,
  //   };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::SLIP10Derive(chaincode) => Ok(chaincode),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }

  // pub async fn bip39_recover<P>(
  //   &self,
  //   mnemonic: String,
  //   output: Location,
  //   passphrase: P,
  //   hint: RecordHint,
  // ) -> Result<()>
  // where
  //   P: Into<Option<String>>,
  // {
  //   let procedure: BIP39Recover = BIP39Recover {
  //     mnemonic,
  //     passphrase: passphrase.into(),
  //     output,
  //     hint,
  //   };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::BIP39Recover => Ok(()),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }
  //
  // pub async fn bip39_generate<P>(&self, output: Location, passphrase: P, hint: RecordHint) -> Result<()>
  // where
  //   P: Into<Option<String>>,
  // {
  //   let procedure: BIP39Generate = BIP39Generate {
  //     passphrase: passphrase.into(),
  //     language: MnemonicLanguage::English, // todo
  //     output,
  //     hint,
  //   };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::BIP39Generate => Ok(()),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }
  //
  // pub async fn bip39_mnemonic_sentence(&self, seed: Location) -> Result<String> {
  //   let procedure = BIP39MnemonicSentence { seed };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::BIP39MnemonicSentence(mnemonic) => Ok(mnemonic),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }
  //
  // pub async fn ed25519_public_key(&self, private_key: Location) -> Result<[u8; 32]> {
  //   let procedure = Ed25519PublicKey { private_key };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::Ed25519PublicKey(public_key) => Ok(public_key),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }
  //
  // pub async fn ed25519_sign(&self, msg: Vec<u8>, private_key: Location) -> Result<[u8; 64]> {
  //   let procedure: Ed25519Sign = Ed25519Sign { private_key, msg };
  //
  //   match self.execute(procedure).await? {
  //     ProcedureResult::Ed25519Sign(signature) => Ok(signature),
  //     _ => Err(Error::StrongholdProcedureFailure),
  //   }
  // }
}
