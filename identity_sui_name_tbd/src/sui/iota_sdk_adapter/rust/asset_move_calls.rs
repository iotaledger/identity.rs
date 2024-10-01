// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use bcs;

use iota_sdk::types::transaction::ProgrammableMoveCall;
use crate::iota_sdk_abstraction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;

use crate::iota_sdk_abstraction::{
    AssetMoveCalls,
    ProgrammableTransactionBcs,
    types::{
        TypeTag,
        base_types::{SequenceNumber, ObjectID, ObjectRef, IotaAddress},
        transaction::{Argument, Command, ObjectArg},
    },
    rpc_types::OwnedObjectRef
};
use crate::utils::MoveType;
use crate::Error;
use crate::ident_str;

pub struct AssetMoveCallsRustSdk {}

impl AssetMoveCalls for AssetMoveCallsRustSdk {
    type Error = Error;

    fn new_asset<T: Serialize + MoveType>(
        inner: T,
        mutable: bool,
        transferable: bool,
        deletable: bool,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let inner = ptb.pure(inner).map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let mutable = ptb.pure(mutable).map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let transferable = ptb
            .pure(transferable)
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let deletable = ptb.pure(deletable).map_err(|e| Error::InvalidArgument(e.to_string()))?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package,
            module: ident_str!("asset").into(),
            function: ident_str!("new_with_config").into(),
            type_arguments: vec![T::move_type(package)],
            arguments: vec![inner, mutable, transferable, deletable],
        })));
        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn delete<T: MoveType>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let asset = ptb
            .obj(ObjectArg::ImmOrOwnedObject(asset))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;

        ptb.command(Command::move_call(
            package,
            ident_str!("asset").into(),
            ident_str!("delete").into(),
            vec![T::move_type(package)],
            vec![asset],
        ));

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn transfer<T: MoveType>(
        asset: ObjectRef,
        recipient: IotaAddress,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let asset = ptb
            .obj(ObjectArg::ImmOrOwnedObject(asset))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let recipient = ptb.pure(recipient).map_err(|e| Error::InvalidArgument(e.to_string()))?;

        ptb.command(Command::move_call(
            package,
            ident_str!("asset").into(),
            ident_str!("transfer").into(),
            vec![T::move_type(package)],
            vec![asset, recipient],
        ));

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn make_tx(
        proposal: (ObjectID, SequenceNumber),
        cap: ObjectRef,
        asset: ObjectRef,
        asset_type_param: TypeTag,
        package: ObjectID,
        function_name: &'static str,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let proposal = ptb
            .obj(ObjectArg::SharedObject {
                id: proposal.0,
                initial_shared_version: proposal.1,
                mutable: true,
            })
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let cap = ptb
            .obj(ObjectArg::ImmOrOwnedObject(cap))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let asset = ptb
            .obj(ObjectArg::Receiving(asset))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;

        ptb.command(Command::move_call(
            package,
            ident_str!("asset").into(),
            ident_str!(function_name).into(),
            vec![asset_type_param],
            vec![proposal, cap, asset],
        ));

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn accept_proposal(
        proposal: (ObjectID, SequenceNumber),
        recipient_cap: ObjectRef,
        asset: ObjectRef,
        asset_type_param: TypeTag,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        Self::make_tx(proposal, recipient_cap, asset, asset_type_param, package, "accept")
    }

    fn conclude_or_cancel(
        proposal: (ObjectID, SequenceNumber),
        sender_cap: ObjectRef,
        asset: ObjectRef,
        asset_type_param: TypeTag,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error>{
        Self::make_tx(
            proposal,
            sender_cap,
            asset,
            asset_type_param,
            package,
            "conclude_or_cancel",
        )
    }

    fn update<T: MoveType + Serialize>(asset: ObjectRef, new_content: T, package: ObjectID)
                                       -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let asset = ptb
            .obj(ObjectArg::ImmOrOwnedObject(asset))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let new_content = ptb
            .pure(new_content)
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;

        ptb.command(Command::move_call(
            package,
            ident_str!("asset").into(),
            ident_str!("set_content").into(),
            vec![T::move_type(package)],
            vec![asset, new_content],
        ));

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }
}