// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::timelock_unlock_condition {

    /// The timelock is not expired error.
    const ETimelockNotExpired: u64 = 0;

    /// The Stardust timelock unlock condition.
    public struct TimelockUnlockCondition has store {
        /// The unix time (seconds since Unix epoch) starting from which the output can be consumed.
        unix_time: u32
    }

    /// Check the unlock condition.
    public fun unlock(condition: TimelockUnlockCondition, ctx: &TxContext) {
        assert!(!is_timelocked(&condition, ctx), ETimelockNotExpired);

        let TimelockUnlockCondition {
            unix_time: _,
        } = condition;
    }

    /// Check if the output is locked by the `Timelock` condition.
    public fun is_timelocked(condition: &TimelockUnlockCondition, ctx: &TxContext): bool {
        condition.unix_time() > ((tx_context::epoch_timestamp_ms(ctx) / 1000) as u32)
    }

    /// Get the unlock condition's `unix_time`.
    public fun unix_time(condition: &TimelockUnlockCondition): u32 {
        condition.unix_time
    }

    #[test_only]
    public fun create_for_testing(unix_time: u32): TimelockUnlockCondition {
        TimelockUnlockCondition { unix_time }
    }
}
