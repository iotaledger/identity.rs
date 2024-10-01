// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::iter::IntoIterator;

use serde::Serialize;

use crate::utils::MoveType;
use crate::iota_sdk_abstraction::{
    ProgrammableTransactionBcs,
    types::TypeTag,
    types::base_types::{SequenceNumber, ObjectID, ObjectRef, IotaAddress},
    types::transaction::{Argument},
    rpc_types::OwnedObjectRef
};

pub trait AssetMoveCalls {
    type Error;

    fn new_asset<T: Serialize + MoveType>(
        inner: T,
        mutable: bool,
        transferable: bool,
        deletable: bool,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn delete<T: MoveType>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn transfer<T: MoveType>(
        asset: ObjectRef,
        recipient: IotaAddress,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn make_tx(
        proposal: (ObjectID, SequenceNumber),
        cap: ObjectRef,
        asset: ObjectRef,
        asset_type_param: TypeTag,
        package: ObjectID,
        function_name: &'static str,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn accept_proposal(
        proposal: (ObjectID, SequenceNumber),
        recipient_cap: ObjectRef,
        asset: ObjectRef,
        asset_type_param: TypeTag,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn conclude_or_cancel(
        proposal: (ObjectID, SequenceNumber),
        sender_cap: ObjectRef,
        asset: ObjectRef,
        asset_type_param: TypeTag,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn update<T: MoveType + Serialize>(asset: ObjectRef, new_content: T, package: ObjectID)
        -> Result<ProgrammableTransactionBcs, Self::Error>;
}

pub trait IdentityMoveCalls {
    type Error;
    type TxBuilder;

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
    ;

    fn execute_config_change(
        ptb: Option<Self::TxBuilder>,
        proposal_id_arg: Option<Argument>,
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> anyhow::Result<ProgrammableTransactionBcs>;

    fn new_identity(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn new_with_controllers<C: IntoIterator<Item = (IotaAddress, u64)>>(
        did_doc: &[u8],
        controllers: C,
        threshold: u64,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn propose_deactivation(
        identity: OwnedObjectRef,
        capability: ObjectRef,
        expiration: Option<u64>,
        package_id: ObjectID,
    ) -> Result<(Self::TxBuilder, Argument), anyhow::Error>;

    fn execute_deactivation(
        ptb: Option<Self::TxBuilder>,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, anyhow::Error>;

    fn approve_proposal<T: MoveType>(
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn propose_update(
        identity: OwnedObjectRef,
        capability: ObjectRef,
        did_doc: impl AsRef<[u8]>,
        expiration: Option<u64>,
        package_id: ObjectID,
    ) -> Result<(Self::TxBuilder, Argument), anyhow::Error>;

    fn execute_update(
        ptb: Option<Self::TxBuilder>,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, anyhow::Error>;
}