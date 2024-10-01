// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use std::collections::HashSet;

use crate::sui::iota_sdk_adapter::TransactionBuilderAdapter;
use crate::iota_sdk_abstraction::{
    IdentityMoveCalls,
    ProgrammableTransactionBcs,
    types::{
        TypeTag,
        base_types::{SequenceNumber, ObjectID, ObjectRef, IotaAddress},
        transaction::Argument,
    }
};
use crate::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use crate::utils::MoveType;
use crate::Error;

pub struct IdentityMoveCallsTsSdk {}

impl IdentityMoveCalls for IdentityMoveCallsTsSdk {
    type Error = Error;
    type TxBuilder = TransactionBuilderAdapter;

    fn propose_config_change<I1, I2>(
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        expiration: Option<u64>,
        threshold: Option<u64>,
        controllers_to_add: I1,
        controllers_to_remove: HashSet<ObjectID>,
        controllers_to_update: I2,
        package: ObjectID,
    ) -> anyhow::Result<(Self::TxBuilder, Argument)>
        where
            I1: IntoIterator<Item = (IotaAddress, u64)>,
            I2: IntoIterator<Item = (ObjectID, u64)>,
    {
        unimplemented!();
    }

    fn execute_config_change(
        ptb: Option<Self::TxBuilder>,
        proposal_id_arg: Option<Argument>,
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> anyhow::Result<ProgrammableTransactionBcs> {
        unimplemented!();
    }

    fn new_identity(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
        unimplemented!();
    }

    fn new_with_controllers<C>(
        did_doc: &[u8],
        controllers: C,
        threshold: u64,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        unimplemented!();
    }

    fn propose_deactivation(
        identity: OwnedObjectRef,
        capability: ObjectRef,
        expiration: Option<u64>,
        package_id: ObjectID,
    ) -> Result<(Self::TxBuilder, Argument), anyhow::Error> {
        unimplemented!();
    }

    fn execute_deactivation(
        ptb: Option<Self::TxBuilder>,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, anyhow::Error> {
        unimplemented!();
    }

    fn approve_proposal<T: MoveType>(
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        unimplemented!();
    }

    fn propose_update(
        identity: OwnedObjectRef,
        capability: ObjectRef,
        did_doc: impl AsRef<[u8]>,
        expiration: Option<u64>,
        package_id: ObjectID,
    ) -> Result<(Self::TxBuilder, Argument), anyhow::Error> {
        unimplemented!();
    }

    fn execute_update(
        ptb: Option<Self::TxBuilder>,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, anyhow::Error> {
        unimplemented!();
    }
}
