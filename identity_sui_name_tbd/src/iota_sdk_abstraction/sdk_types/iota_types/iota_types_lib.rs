// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::super::move_core_types::{
    account_address::AccountAddress,
    language_storage::TypeTag,
};
use super::base_types::{ObjectID, SequenceNumber, IotaAddress};
use super::object::OBJECT_START_VERSION;

/// 0x1-- account address where Move stdlib modules are stored
/// Same as the ObjectID
pub const MOVE_STDLIB_ADDRESS: AccountAddress = AccountAddress::ONE;
pub const MOVE_STDLIB_PACKAGE_ID: ObjectID = ObjectID::from_address(MOVE_STDLIB_ADDRESS);

/// 0x2-- account address where iota framework modules are stored
/// Same as the ObjectID
pub const IOTA_FRAMEWORK_ADDRESS: AccountAddress = address_from_single_byte(2);
pub const IOTA_FRAMEWORK_PACKAGE_ID: ObjectID = ObjectID::from_address(IOTA_FRAMEWORK_ADDRESS);

/// 0x3-- account address where iota system modules are stored
/// Same as the ObjectID
pub const IOTA_SYSTEM_ADDRESS: AccountAddress = address_from_single_byte(3);
pub const IOTA_SYSTEM_PACKAGE_ID: ObjectID = ObjectID::from_address(IOTA_SYSTEM_ADDRESS);

/// 0xdee9-- account address where DeepBook modules are stored
/// Same as the ObjectID
pub const DEEPBOOK_ADDRESS: AccountAddress = deepbook_addr();
pub const DEEPBOOK_PACKAGE_ID: ObjectID = ObjectID::from_address(DEEPBOOK_ADDRESS);

/// 0x107a-- account address where Stardust modules are stored
/// Same as the ObjectID
pub const STARDUST_ADDRESS: AccountAddress = stardust_addr();
pub const STARDUST_PACKAGE_ID: ObjectID = ObjectID::from_address(STARDUST_ADDRESS);

/// 0x5: hardcoded object ID for the singleton iota system state object.
pub const IOTA_SYSTEM_STATE_ADDRESS: AccountAddress = address_from_single_byte(5);
pub const IOTA_SYSTEM_STATE_OBJECT_ID: ObjectID = ObjectID::from_address(IOTA_SYSTEM_STATE_ADDRESS);
pub const IOTA_SYSTEM_STATE_OBJECT_SHARED_VERSION: SequenceNumber = OBJECT_START_VERSION;

/// 0x6: hardcoded object ID for the singleton clock object.
pub const IOTA_CLOCK_ADDRESS: AccountAddress = address_from_single_byte(6);
pub const IOTA_CLOCK_OBJECT_ID: ObjectID = ObjectID::from_address(IOTA_CLOCK_ADDRESS);
pub const IOTA_CLOCK_OBJECT_SHARED_VERSION: SequenceNumber = OBJECT_START_VERSION;

/// 0x7: hardcode object ID for the singleton authenticator state object.
pub const IOTA_AUTHENTICATOR_STATE_ADDRESS: AccountAddress = address_from_single_byte(7);
pub const IOTA_AUTHENTICATOR_STATE_OBJECT_ID: ObjectID =
    ObjectID::from_address(IOTA_AUTHENTICATOR_STATE_ADDRESS);
pub const IOTA_AUTHENTICATOR_STATE_OBJECT_SHARED_VERSION: SequenceNumber = OBJECT_START_VERSION;

/// 0x8: hardcode object ID for the singleton randomness state object.
pub const IOTA_RANDOMNESS_STATE_ADDRESS: AccountAddress = address_from_single_byte(8);
pub const IOTA_RANDOMNESS_STATE_OBJECT_ID: ObjectID =
    ObjectID::from_address(IOTA_RANDOMNESS_STATE_ADDRESS);

/// 0x403: hardcode object ID for the singleton DenyList object.
pub const IOTA_DENY_LIST_ADDRESS: AccountAddress = deny_list_addr();
pub const IOTA_DENY_LIST_OBJECT_ID: ObjectID = ObjectID::from_address(IOTA_DENY_LIST_ADDRESS);

const fn address_from_single_byte(b: u8) -> AccountAddress {
    let mut addr = [0u8; AccountAddress::LENGTH];
    addr[AccountAddress::LENGTH - 1] = b;
    AccountAddress::new(addr)
}

/// return 0x0...dee9
pub(crate) const fn deepbook_addr() -> AccountAddress {
    let mut addr = [0u8; AccountAddress::LENGTH];
    addr[AccountAddress::LENGTH - 2] = 0xde;
    addr[AccountAddress::LENGTH - 1] = 0xe9;
    AccountAddress::new(addr)
}

/// return 0x0...107a
const fn stardust_addr() -> AccountAddress {
    let mut addr = [0u8; AccountAddress::LENGTH];
    addr[AccountAddress::LENGTH - 2] = 0x10;
    addr[AccountAddress::LENGTH - 1] = 0x7a;
    AccountAddress::new(addr)
}

/// return 0x0...403
const fn deny_list_addr() -> AccountAddress {
    let mut addr = [0u8; AccountAddress::LENGTH];
    addr[AccountAddress::LENGTH - 2] = 4;
    addr[AccountAddress::LENGTH - 1] = 3;
    AccountAddress::new(addr)
}

pub trait MoveTypeTagTrait {
    fn get_type_tag() -> TypeTag;
}

impl MoveTypeTagTrait for u8 {
    fn get_type_tag() -> TypeTag {
        TypeTag::U8
    }
}

impl MoveTypeTagTrait for u64 {
    fn get_type_tag() -> TypeTag {
        TypeTag::U64
    }
}

impl MoveTypeTagTrait for ObjectID {
    fn get_type_tag() -> TypeTag {
        TypeTag::Address
    }
}

impl MoveTypeTagTrait for IotaAddress {
    fn get_type_tag() -> TypeTag {
        TypeTag::Address
    }
}

impl<T: MoveTypeTagTrait> MoveTypeTagTrait for Vec<T> {
    fn get_type_tag() -> TypeTag {
        TypeTag::Vector(Box::new(T::get_type_tag()))
    }
}