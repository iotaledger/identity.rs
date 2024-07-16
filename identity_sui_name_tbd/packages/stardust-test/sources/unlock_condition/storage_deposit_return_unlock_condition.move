// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::storage_deposit_return_unlock_condition {

    use iota::balance::{Balance, split};
    use iota::coin::from_balance;
    use iota::iota::IOTA;
    use iota::transfer::public_transfer;

    /// The Stardust storage deposit return unlock condition.
    public struct StorageDepositReturnUnlockCondition has store {
        /// The address to which the consuming transaction should deposit the amount defined in Return Amount.
        return_address: address,
        /// The amount of IOTA coins the consuming transaction should deposit to the address defined in Return Address.
        return_amount: u64,
    }

    /// Check the unlock condition.
    public fun unlock(condition: StorageDepositReturnUnlockCondition, funding: &mut Balance<IOTA>, ctx: &mut TxContext) {
        // Aborts if `funding` is not enough.
        let return_balance = funding.split(condition.return_amount());

        // Recipient will need to transfer the coin to a normal ed25519 address instead of legacy.
        public_transfer(from_balance(return_balance, ctx), condition.return_address());

        let StorageDepositReturnUnlockCondition {
            return_address: _,
            return_amount: _,
        } = condition;
    }

    /// Get the unlock condition's `return_address`.
    public fun return_address(condition: &StorageDepositReturnUnlockCondition): address {
        condition.return_address
    }

    /// Get the unlock condition's `return_amount`.
    public fun return_amount(condition: &StorageDepositReturnUnlockCondition): u64 {
        condition.return_amount
    }

    #[test_only]
    public fun create_for_testing(return_address: address, return_amount: u64): StorageDepositReturnUnlockCondition {
        StorageDepositReturnUnlockCondition {
            return_address,
            return_amount,
        }
    }
}
