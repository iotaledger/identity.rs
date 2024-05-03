// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::nft_output {

    use sui::bag::Bag;
    use sui::balance::Balance;
    use sui::dynamic_object_field;
    use sui::sui::SUI;
    use sui::transfer::Receiving;

    use stardust::nft::Nft;

    use stardust::expiration_unlock_condition::ExpirationUnlockCondition;
    use stardust::storage_deposit_return_unlock_condition::StorageDepositReturnUnlockCondition;
    use stardust::timelock_unlock_condition::TimelockUnlockCondition;

    /// The NFT dynamic field name.
    const NFT_NAME: vector<u8> = b"nft";

    /// The Stardust NFT output representation.
    public struct NftOutput has key {
        /// This is a "random" UID, not the NFTID from Stardust.
        id: UID,

        /// The amount of IOTA tokens held by the output.
        iota: Balance<SUI>,

        /// The `Bag` holds native tokens, key-ed by the stringified type of the asset.
        /// Example: key: "0xabcded::soon::SOON", value: Balance<0xabcded::soon::SOON>.
        native_tokens: Bag,

        /// The storage deposit return unlock condition.
        storage_deposit_return: Option<StorageDepositReturnUnlockCondition>,
        /// The timelock unlock condition.
        timelock: Option<TimelockUnlockCondition>,
        /// The expiration unlock condition.
        expiration: Option<ExpirationUnlockCondition>,
    }

    /// The function extracts assets from a legacy NFT output.
    public fun extract_assets(mut output: NftOutput, ctx: &mut TxContext): (Balance<SUI>, Bag, Nft) {
        // Load the related Nft object.
        let nft = load_nft(&mut output);

        // Unpuck the output.
        let NftOutput {
            id,
            iota: mut iota,
            native_tokens,
            storage_deposit_return: mut storage_deposit_return,
            timelock: mut timelock,
            expiration: mut expiration
        } = output;

        // If the output has a timelock, then we need to check if the timelock has expired.
        if (timelock.is_some()) {
            timelock.extract().unlock(ctx);
        };

        // If the output has an expiration, then we need to check who can unlock the output.
        if (expiration.is_some()) {
            expiration.extract().unlock(ctx);
        };

        // If the output has an SDRUC, then we need to return the deposit.
        if (storage_deposit_return.is_some()) {
            storage_deposit_return.extract().unlock(&mut iota, ctx);
        };

        // Destroy the output.
        option::destroy_none(timelock);
        option::destroy_none(expiration);
        option::destroy_none(storage_deposit_return);

        object::delete(id);

        return (iota, native_tokens, nft)
    }

    /// Loads the related `Nft` object.
    fun load_nft(output: &mut NftOutput): Nft {
        dynamic_object_field::remove(&mut output.id, NFT_NAME)
    }

    // === Public-Package Functions ===

    /// Utility function to receive an `NftOutput` in other Stardust modules.
    /// Other modules in the stardust package can call this function to receive an `NftOutput` (alias).
    public(package) fun receive(parent: &mut UID, nft: Receiving<NftOutput>) : NftOutput {
        transfer::receive(parent, nft)
    }

    // === Test Functions ===

    #[test_only]
    public fun attach_nft(output: &mut NftOutput, nft: Nft) {
        dynamic_object_field::add(&mut output.id, NFT_NAME, nft)
    }

    #[test_only]
    public fun create_for_testing(
        iota: Balance<SUI>,
        native_tokens: Bag,
        storage_deposit_return: Option<StorageDepositReturnUnlockCondition>,
        timelock: Option<TimelockUnlockCondition>,
        expiration: Option<ExpirationUnlockCondition>,
        ctx: &mut TxContext,
    ): NftOutput {
        NftOutput {
            id: object::new(ctx),
            iota,
            native_tokens,
            storage_deposit_return,
            timelock,
            expiration,
        }
    }
}
