// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::str::FromStr;

use bcs;

use crate::iota_sdk_abstraction::{
    IdentityMoveCalls,
    ProgrammableTransactionBcs,
};

// ProgrammableTransactionBuilder can only be used cause this is a platform specific file
use crate::iota_sdk_abstraction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;

use crate::iota_sdk_abstraction::types::{
    TypeTag, IOTA_FRAMEWORK_PACKAGE_ID,
};
use crate::iota_sdk_abstraction::types::{
    object::Owner,
    base_types::{ObjectID, ObjectRef, IotaAddress},
    transaction::{Argument, Command, ObjectArg, ProgrammableMoveCall, ProgrammableTransaction},
};
use crate::iota_sdk_abstraction::rpc_types::OwnedObjectRef;

use crate::migration::OnChainIdentity;
use crate::utils::MoveType;
use crate::Error;
use crate::ident_str;

use super::super::TransactionBuilderAdapter;

pub struct IdentityMoveCallsRustSdk {}

impl IdentityMoveCalls for IdentityMoveCallsRustSdk {
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
        let mut ptb = ProgrammableTransactionBuilder::new();

        let controllers_to_add = {
            let (addresses, vps): (Vec<IotaAddress>, Vec<u64>) = controllers_to_add.into_iter().unzip();
            let addresses = ptb.pure(addresses)?;
            let vps = ptb.pure(vps)?;

            ptb.programmable_move_call(
                package,
                ident_str!("utils").into(),
                ident_str!("vec_map_from_keys_values").into(),
                vec![TypeTag::Address, TypeTag::U64],
                vec![addresses, vps],
            )
        };
        let controllers_to_update = {
            let (ids, vps): (Vec<ObjectID>, Vec<u64>) = controllers_to_update.into_iter().unzip();
            let ids = ptb.pure(ids)?;
            let vps = ptb.pure(vps)?;

            ptb.programmable_move_call(
                package,
                ident_str!("utils").into(),
                ident_str!("vec_map_from_keys_values").into(),
                vec![TypeTag::from_str("0x2::object::ID").expect("valid utf8"), TypeTag::U64],
                vec![ids, vps],
            )
        };
        let identity = super::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
        let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
        let expiration = super::utils::option_to_move(expiration, &mut ptb, package)?;
        let threshold = super::utils::option_to_move(threshold, &mut ptb, package)?;
        let controllers_to_remove = ptb.pure(controllers_to_remove)?;

        let proposal_id = ptb.programmable_move_call(
            package,
            ident_str!("identity").into(),
            ident_str!("propose_config_change").into(),
            vec![],
            vec![
                identity,
                controller_cap,
                expiration,
                threshold,
                controllers_to_add,
                controllers_to_remove,
                controllers_to_update,
            ],
        );

        Ok((TransactionBuilderAdapter::new(ptb), proposal_id))
    }

    fn execute_config_change(
        ptb: Option<Self::TxBuilder>,
        proposal_id_arg: Option<Argument>,
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> anyhow::Result<ProgrammableTransactionBcs> {
        let TransactionBuilderAdapter{ builder } = ptb.unwrap_or_default();
        let programmable_tx = Self::execute_config_change_inner(
            builder,
            proposal_id_arg,
            identity,
            controller_cap,
            proposal_id,
            package
        )?;
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn new_identity(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let doc_arg = super::utils::ptb_pure(&mut ptb, "did_doc", did_doc)?;

        // Create a new identity, sending its capability to the tx's sender.
        let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: ident_str!("identity").into(),
            function: ident_str!("new").into(),
            type_arguments: vec![],
            arguments: vec![doc_arg],
        })));

        // Share the resulting identity.
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: IOTA_FRAMEWORK_PACKAGE_ID,
            module: ident_str!("transfer").into(),
            function: ident_str!("public_share_object").into(),
            type_arguments: vec![OnChainIdentity::move_type(package_id)],
            arguments: vec![identity_res],
        })));

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn new_with_controllers<C: IntoIterator<Item = (IotaAddress, u64)>>(
        did_doc: &[u8],
        controllers: C,
        threshold: u64,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let controllers = {
            let (ids, vps): (Vec<IotaAddress>, Vec<u64>) = controllers.into_iter().unzip();
            let ids = ptb.pure(ids).map_err(|e| Error::InvalidArgument(e.to_string()))?;
            let vps = ptb.pure(vps).map_err(|e| Error::InvalidArgument(e.to_string()))?;
            ptb.programmable_move_call(
                package_id,
                ident_str!("utils").into(),
                ident_str!("vec_map_from_keys_values").into(),
                vec![TypeTag::Address, TypeTag::U64],
                vec![ids, vps],
            )
        };
        let doc_arg = ptb.pure(did_doc).map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let threshold_arg = ptb.pure(threshold).map_err(|e| Error::InvalidArgument(e.to_string()))?;

        // Create a new identity, sending its capabilities to the specified controllers.
        let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: package_id,
            module: ident_str!("identity").into(),
            function: ident_str!("new_with_controllers").into(),
            type_arguments: vec![],
            arguments: vec![doc_arg, controllers, threshold_arg],
        })));

        // Share the resulting identity.
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: IOTA_FRAMEWORK_PACKAGE_ID,
            module: ident_str!("transfer").into(),
            function: ident_str!("public_share_object").into(),
            type_arguments: vec![OnChainIdentity::move_type(package_id)],
            arguments: vec![identity_res],
        })));

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn propose_deactivation(
        identity: OwnedObjectRef,
        capability: ObjectRef,
        expiration: Option<u64>,
        package_id: ObjectID,
    ) -> Result<(Self::TxBuilder, Argument), anyhow::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
        let identity_arg = super::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
        let exp_arg = super::utils::option_to_move(expiration, &mut ptb, package_id)?;

        let proposal_id = ptb.programmable_move_call(
            package_id,
            ident_str!("identity").into(),
            ident_str!("propose_deactivation").into(),
            vec![],
            vec![identity_arg, cap_arg, exp_arg],
        );

        Ok((TransactionBuilderAdapter::new(ptb), proposal_id))
    }

    fn execute_deactivation(
        ptb: Option<Self::TxBuilder>,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, anyhow::Error> {
        let TransactionBuilderAdapter{ builder } = ptb.unwrap_or_default();
        let programmable_tx = Self::execute_deactivation_inner(
            builder,
            proposal_arg,
            identity,
            capability,
            proposal_id,
            package_id
        )?;
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn approve_proposal<T: MoveType>(
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, Self::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let Owner::Shared { initial_shared_version } = identity.owner else {
            return Err(Error::TransactionBuildingFailed(format!(
                "Identity \"{}\" is not a shared object",
                identity.object_id()
            )));
        };
        let identity = ptb
            .obj(ObjectArg::SharedObject {
                id: identity.object_id(),
                initial_shared_version,
                mutable: true,
            })
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let controller_cap = ptb
            .obj(ObjectArg::ImmOrOwnedObject(controller_cap))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let proposal_id = ptb
            .pure(proposal_id)
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;

        ptb.programmable_move_call(
            package,
            ident_str!("identity").into(),
            ident_str!("approve_proposal").into(),
            vec![T::move_type(package)],
            vec![identity, controller_cap, proposal_id],
        );

        let programmable_tx = ptb.finish();
        Ok(bcs::to_bytes(&programmable_tx)?)
    }

    fn propose_update(
        identity: OwnedObjectRef,
        capability: ObjectRef,
        did_doc: impl AsRef<[u8]>,
        expiration: Option<u64>,
        package_id: ObjectID,
    ) -> Result<(Self::TxBuilder, Argument), anyhow::Error> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
        let identity_arg = super::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
        let exp_arg = super::utils::option_to_move(expiration, &mut ptb, package_id)?;
        let doc_arg = ptb.pure(did_doc.as_ref())?;

        let proposal_id = ptb.programmable_move_call(
            package_id,
            ident_str!("identity").into(),
            ident_str!("propose_update").into(),
            vec![],
            vec![identity_arg, cap_arg, doc_arg, exp_arg],
        );

        Ok((TransactionBuilderAdapter::new(ptb), proposal_id))
    }

    fn execute_update(
        ptb: Option<Self::TxBuilder>,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransactionBcs, anyhow::Error> {
        let TransactionBuilderAdapter{ builder } = ptb.unwrap_or_default();
        let programmable_tx = Self::execute_update_inner(
            builder,
            proposal_arg,
            identity,
            capability,
            proposal_id,
            package_id,
        )?;
        Ok(bcs::to_bytes(&programmable_tx)?)
    }
}

impl IdentityMoveCallsRustSdk {
    fn execute_config_change_inner(
        mut ptb: ProgrammableTransactionBuilder,
        proposal_id_arg: Option<Argument>,
        identity: OwnedObjectRef,
        controller_cap: ObjectRef,
        proposal_id: ObjectID,
        package: ObjectID,
    ) -> anyhow::Result<ProgrammableTransaction> {
        let Owner::Shared { initial_shared_version } = identity.owner else {
            anyhow::bail!("identity \"{}\" is a not shared object", identity.reference.object_id);
        };
        let identity = ptb.obj(ObjectArg::SharedObject {
            id: identity.reference.object_id,
            initial_shared_version,
            mutable: true,
        })?;
        let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
        let proposal_id = if let Some(proposal_id) = proposal_id_arg {
            proposal_id
        } else {
            ptb.pure(proposal_id)?
        };
        ptb.programmable_move_call(
            package,
            ident_str!("identity").into(),
            ident_str!("execute_config_change").into(),
            vec![],
            vec![identity, controller_cap, proposal_id],
        );

        Ok(ptb.finish())
    }

    pub fn execute_deactivation_inner(
        mut ptb: ProgrammableTransactionBuilder,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransaction, anyhow::Error> {
        let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
        let proposal_id = if let Some(proposal_id) = proposal_arg {
            proposal_id
        } else {
            ptb.pure(proposal_id)?
        };
        let identity_arg = super::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;

        let _ = ptb.programmable_move_call(
            package_id,
            ident_str!("identity").into(),
            ident_str!("execute_deactivation").into(),
            vec![],
            vec![identity_arg, cap_arg, proposal_id],
        );

        Ok(ptb.finish())
    }

    fn execute_update_inner(
        mut ptb: ProgrammableTransactionBuilder,
        proposal_arg: Option<Argument>,
        identity: OwnedObjectRef,
        capability: ObjectRef,
        proposal_id: ObjectID,
        package_id: ObjectID,
    ) -> Result<ProgrammableTransaction, anyhow::Error> {
        let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
        let proposal_id = if let Some(proposal_id) = proposal_arg {
            proposal_id
        } else {
            ptb.pure(proposal_id)?
        };
        let identity_arg = super::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;

        let _ = ptb.programmable_move_call(
            package_id,
            ident_str!("identity").into(),
            ident_str!("execute_update").into(),
            vec![],
            vec![identity_arg, cap_arg, proposal_id],
        );

        Ok(ptb.finish())
    }
}