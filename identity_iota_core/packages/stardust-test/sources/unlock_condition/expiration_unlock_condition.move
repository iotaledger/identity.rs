// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::expiration_unlock_condition {

    /// The output can not be unlocked by the sender error.
    const EWrongSender: u64 = 0;

    /// The Stardust expiration unlock condition.
    public struct ExpirationUnlockCondition has store {
        /// The address who owns the output before the timestamp has passed.
        owner: address,
        /// The address that is allowed to spend the locked funds after the timestamp has passed.
        return_address: address,
        /// Before this unix time, Address Unlock Condition is allowed to unlock the output, after that only the address defined in Return Address.
        unix_time: u32,
    }

    /// Check the unlock condition.
    public fun unlock(condition: ExpirationUnlockCondition, ctx: &mut TxContext) {
        let unlock_address = condition.can_be_unlocked_by(ctx);
        
        assert!(unlock_address == ctx.sender(), EWrongSender);

        let ExpirationUnlockCondition {
            owner: _,
            return_address: _,
            unix_time: _,
        } = condition;
    }

    /// Return the address that can unlock the related output.
    public fun can_be_unlocked_by(condition: &ExpirationUnlockCondition, ctx: &TxContext): address {
        // Unix time in seconds.
        let current_time = ((tx_context::epoch_timestamp_ms(ctx) / 1000) as u32);

        if (condition.unix_time() < current_time) {
            condition.return_address()
        } else {
            condition.owner()
        }
    }

    /// Get the unlock condition's `owner`.
    public fun owner(condition: &ExpirationUnlockCondition): address {
        condition.owner
    }

    /// Get the unlock condition's `return_address`.
    public fun return_address(condition: &ExpirationUnlockCondition): address {
        condition.return_address
    }

    /// Get the unlock condition's `unix_time`.
    public fun unix_time(condition: &ExpirationUnlockCondition): u32 {
        condition.unix_time
    }

    #[test_only]
    public fun create_for_testing(owner: address, return_address: address, unix_time: u32): ExpirationUnlockCondition {
        ExpirationUnlockCondition {
            owner,
            return_address,
            unix_time,
        }
    }
}
