// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::option::Option;
use std::result::Result;
use std::boxed::Box;
use std::marker::Send;

use secret_storage::Signer;

use identity_iota::iota::iota_sdk_abstraction::error::IotaRpcResult;
use identity_iota::iota::iota_sdk_abstraction::Error;
use identity_iota::iota::iota_sdk_abstraction::shared_crypto::intent::{Intent, IntentMessage};
use identity_iota::iota::iota_sdk_abstraction::{
    ProgrammableTransactionBcs,
    IotaClientTrait,
    IotaKeySignature,
    QuorumDriverTrait,
    ReadTrait,
    CoinReadTrait,
    EventTrait,
    IotaTransactionBlockResponseT
};
use identity_iota::iota::iota_sdk_abstraction::types::{
    base_types::{SequenceNumber, ObjectID, IotaAddress},
};
use identity_iota::iota::iota_sdk_abstraction::rpc_types::{
    IotaTransactionBlockResponseOptions,
    IotaObjectResponse,
    IotaPastObjectResponse,
    ObjectsPage,
    IotaObjectResponseQuery,
    IotaObjectDataOptions,
    IotaExecutionStatus,
    CoinPage,
    EventFilter,
    EventPage,
    IotaObjectData,
    OwnedObjectRef,
};

use super::wasm_iota_client::{WasmIotaClient, ManagedWasmIotaClient};

pub struct IotaTransactionBlockResponseProvider {}

impl IotaTransactionBlockResponseT for IotaTransactionBlockResponseProvider {
    type Error = Error;

    fn effects_is_none(&self) -> bool {
        unimplemented!();
    }

    fn effects_is_some(&self) -> bool {
        unimplemented!();
    }

    fn to_string(&self) -> String {
        unimplemented!();
    }

    fn effects_execution_status(&self) -> Option<IotaExecutionStatus> {
        unimplemented!();
    }

    fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct IotaClientTsSdk {
    iota_client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl IotaClientTrait for IotaClientTsSdk {
    type Error = Error;

    fn quorum_driver_api(&self) -> Box<dyn QuorumDriverTrait<Error = Self::Error> + Send + '_> {
        unimplemented!();
    }

    fn read_api(&self) -> Box<dyn ReadTrait<Error = Self::Error> + Send + '_> {
        unimplemented!();
    }

    fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + Send + '_> {
        unimplemented!();
    }

    fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + Send + '_> {
        unimplemented!();
    }

    async fn execute_transaction<S: Signer<IotaKeySignature> + Sync>(
        &self,
        sender_address: IotaAddress,
        sender_public_key: &[u8],
        tx_bcs: ProgrammableTransactionBcs,
        gas_budget: Option<u64>,
        signer: &S
    ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error=Self::Error>>, Self::Error> {
        unimplemented!();
    }

    async fn default_gas_budget(&self, sender_address: IotaAddress, tx_bcs: &ProgrammableTransactionBcs) -> Result<u64, Error> {
        unimplemented!();
    }

    async fn get_previous_version(&self, iod: IotaObjectData) -> Result<Option<IotaObjectData>, Self::Error> {
        unimplemented!();
    }

    async fn get_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
    ) -> Result<IotaPastObjectResponse, Self::Error> {
        unimplemented!();
    }
}

impl IotaClientTsSdk {
    pub fn new(iota_client: WasmIotaClient) -> Result<Self, Error> {
        Ok(
            Self {iota_client: ManagedWasmIotaClient::new(iota_client)}
        )
    }
}