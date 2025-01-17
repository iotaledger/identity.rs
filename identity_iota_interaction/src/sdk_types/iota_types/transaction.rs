// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::super::move_core_types::identifier::Identifier;
use super::super::move_core_types::language_storage::TypeTag;

use super::base_types::{ObjectID, ObjectRef, SequenceNumber};
use super::{
    IOTA_CLOCK_OBJECT_ID,
    IOTA_CLOCK_OBJECT_SHARED_VERSION,
    IOTA_AUTHENTICATOR_STATE_OBJECT_ID,
    IOTA_AUTHENTICATOR_STATE_OBJECT_SHARED_VERSION,
    IOTA_SYSTEM_STATE_OBJECT_ID,
    IOTA_SYSTEM_STATE_OBJECT_SHARED_VERSION,
};

pub const TEST_ONLY_GAS_UNIT_FOR_TRANSFER: u64 = 10_000;
pub const TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_PUBLISH: u64 = 70_000;
pub const TEST_ONLY_GAS_UNIT_FOR_STAKING: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_GENERIC: u64 = 50_000;
pub const TEST_ONLY_GAS_UNIT_FOR_SPLIT_COIN: u64 = 10_000;
// For some transactions we may either perform heavy operations or touch
// objects that are storage expensive. That may happen (and often is the case)
// because the object touched are set up in genesis and carry no storage cost
// (and thus rebate) on first usage.
pub const TEST_ONLY_GAS_UNIT_FOR_HEAVY_COMPUTATION_STORAGE: u64 = 5_000_000;

pub const GAS_PRICE_FOR_SYSTEM_TX: u64 = 1;

pub const DEFAULT_VALIDATOR_GAS_PRICE: u64 = 1000;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum CallArg {
    // contains no structs or objects
    Pure(Vec<u8>),
    // an object
    Object(ObjectArg),
}

impl CallArg {
    pub const IOTA_SYSTEM_MUT: Self = Self::Object(ObjectArg::IOTA_SYSTEM_MUT);
    pub const CLOCK_IMM: Self = Self::Object(ObjectArg::SharedObject {
        id: IOTA_CLOCK_OBJECT_ID,
        initial_shared_version: IOTA_CLOCK_OBJECT_SHARED_VERSION,
        mutable: false,
    });
    pub const CLOCK_MUT: Self = Self::Object(ObjectArg::SharedObject {
        id: IOTA_CLOCK_OBJECT_ID,
        initial_shared_version: IOTA_CLOCK_OBJECT_SHARED_VERSION,
        mutable: true,
    });
    pub const AUTHENTICATOR_MUT: Self = Self::Object(ObjectArg::SharedObject {
        id: IOTA_AUTHENTICATOR_STATE_OBJECT_ID,
        initial_shared_version: IOTA_AUTHENTICATOR_STATE_OBJECT_SHARED_VERSION,
        mutable: true,
    });
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ObjectArg {
    // A Move object, either immutable, or owned mutable.
    ImmOrOwnedObject(ObjectRef),
    // A Move object that's shared.
    // SharedObject::mutable controls whether caller asks for a mutable reference to shared
    // object.
    SharedObject {
        id: ObjectID,
        initial_shared_version: SequenceNumber,
        mutable: bool,
    },
    // A Move object that can be received in this transaction.
    Receiving(ObjectRef),
}

impl ObjectArg {
    pub const IOTA_SYSTEM_MUT: Self = Self::SharedObject {
        id: IOTA_SYSTEM_STATE_OBJECT_ID,
        initial_shared_version: IOTA_SYSTEM_STATE_OBJECT_SHARED_VERSION,
        mutable: true,
    };
}

/// The command for calling a Move function, either an entry function or a
/// public function (which cannot return references).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ProgrammableMoveCall {
    /// The package containing the module and function.
    pub package: ObjectID,
    /// The specific module in the package containing the function.
    pub module: Identifier,
    /// The function to be called.
    pub function: Identifier,
    /// The type arguments to the function.
    pub type_arguments: Vec<TypeTag>,
    /// The arguments to the function.
    pub arguments: Vec<Argument>,
}

/// A single command in a programmable transaction.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Command {
    /// A call to either an entry or a public Move function
    MoveCall(Box<ProgrammableMoveCall>),
    /// `(Vec<forall T:key+store. T>, address)`
    /// It sends n-objects to the specified address. These objects must have
    /// store (public transfer) and either the previous owner must be an
    /// address or the object must be newly created.
    TransferObjects(Vec<Argument>, Argument),
    /// `(&mut Coin<T>, Vec<u64>)` -> `Vec<Coin<T>>`
    /// It splits off some amounts into a new coins with those amounts
    SplitCoins(Argument, Vec<Argument>),
    /// `(&mut Coin<T>, Vec<Coin<T>>)`
    /// It merges n-coins into the first coin
    MergeCoins(Argument, Vec<Argument>),
    /// Publishes a Move package. It takes the package bytes and a list of the
    /// package's transitive dependencies to link against on-chain.
    Publish(Vec<Vec<u8>>, Vec<ObjectID>),
    /// `forall T: Vec<T> -> vector<T>`
    /// Given n-values of the same type, it constructs a vector. For non objects
    /// or an empty vector, the type tag must be specified.
    MakeMoveVec(Option<TypeTag>, Vec<Argument>),
    /// Upgrades a Move package
    /// Takes (in order):
    /// 1. A vector of serialized modules for the package.
    /// 2. A vector of object ids for the transitive dependencies of the new
    ///    package.
    /// 3. The object ID of the package being upgraded.
    /// 4. An argument holding the `UpgradeTicket` that must have been produced
    ///    from an earlier command in the same programmable transaction.
    Upgrade(Vec<Vec<u8>>, Vec<ObjectID>, ObjectID, Argument),
}

/// An argument to a programmable transaction command
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Argument {
    /// The gas coin. The gas coin can only be used by-ref, except for with
    /// `TransferObjects`, which can use it by-value.
    GasCoin,
    /// One of the input objects or primitive values (from
    /// `ProgrammableTransaction` inputs)
    Input(u16),
    /// The result of another command (from `ProgrammableTransaction` commands)
    Result(u16),
    /// Like a `Result` but it accesses a nested result. Currently, the only
    /// usage of this is to access a value from a Move call with multiple
    /// return values.
    NestedResult(u16, u16),
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::GasCoin => write!(f, "GasCoin"),
            Argument::Input(i) => write!(f, "Input({i})"),
            Argument::Result(i) => write!(f, "Result({i})"),
            Argument::NestedResult(i, j) => write!(f, "NestedResult({i},{j})"),
        }
    }
}