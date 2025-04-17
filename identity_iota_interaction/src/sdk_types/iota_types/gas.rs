// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use super::iota_serde::{BigInt, Readable};

/// Summary of the charges in a transaction.
/// Storage is charged independently of computation.
/// There are 3 parts to the storage charges:
/// `storage_cost`: it is the charge of storage at the time the transaction
/// is executed.                 The cost of storage is the number of
/// bytes of the objects being mutated                 multiplied by a
/// variable storage cost per byte `storage_rebate`: this is the amount
/// a user gets back when manipulating an object.                   The
/// `storage_rebate` is the `storage_cost` for an object minus fees.
/// `non_refundable_storage_fee`: not all the value of the object storage
/// cost is                               given back to user and there
/// is a small fraction that                               is kept by
/// the system. This value tracks that charge.
///
/// When looking at a gas cost summary the amount charged to the user is
/// `computation_cost + storage_cost - storage_rebate`
/// and that is the amount that is deducted from the gas coins.
/// `non_refundable_storage_fee` is collected from the objects being
/// mutated/deleted and it is tracked by the system in storage funds.
///
/// Objects deleted, including the older versions of objects mutated, have
/// the storage field on the objects added up to a pool of "potential
/// rebate". This rebate then is reduced by the "nonrefundable rate"
/// such that: `potential_rebate(storage cost of deleted/mutated
/// objects) = storage_rebate + non_refundable_storage_fee`

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GasCostSummary {
  /// Cost of computation/execution
  #[schemars(with = "BigInt<u64>")]
  #[serde_as(as = "Readable<BigInt<u64>, _>")]
  pub computation_cost: u64,
  /// The burned component of the computation/execution costs
  #[schemars(with = "BigInt<u64>")]
  #[serde_as(as = "Readable<BigInt<u64>, _>")]
  pub computation_cost_burned: u64,
  /// Storage cost, it's the sum of all storage cost for all objects
  /// created or mutated.
  #[schemars(with = "BigInt<u64>")]
  #[serde_as(as = "Readable<BigInt<u64>, _>")]
  pub storage_cost: u64,
  /// The amount of storage cost refunded to the user for all objects
  /// deleted or mutated in the transaction.
  #[schemars(with = "BigInt<u64>")]
  #[serde_as(as = "Readable<BigInt<u64>, _>")]
  pub storage_rebate: u64,
  /// The fee for the rebate. The portion of the storage rebate kept by
  /// the system.
  #[schemars(with = "BigInt<u64>")]
  #[serde_as(as = "Readable<BigInt<u64>, _>")]
  pub non_refundable_storage_fee: u64,
}
