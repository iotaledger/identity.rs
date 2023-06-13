// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, MnemonicSecretManager } from "@iota/client-wasm/node";
import { Bip39 } from "@iota/crypto.js";
import { IotaDID, IotaDocument, IotaIdentityClient, Storage, JwkMemStore, KeyIdMemStore } from "@iota/identity-wasm/node";
import {
    ADDRESS_UNLOCK_CONDITION_TYPE,
    ALIAS_ADDRESS_TYPE,
    Bech32Helper,
    EXPIRATION_UNLOCK_CONDITION_TYPE,
    FOUNDRY_OUTPUT_TYPE,
    IAliasAddress,
    IAliasOutput,
    IBasicOutput,
    IImmutableAliasUnlockCondition,
    IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE,
    IOutputResponse,
    IRent,
    ISimpleTokenScheme,
    OutputTypes,
    SIMPLE_TOKEN_SCHEME_TYPE,
    TransactionHelper,
} from "@iota/iota.js";
import type { IFoundryOutput } from "@iota/types";
import { HexHelper } from "@iota/util.js";
import bigInt from "big-integer";
import { API_ENDPOINT, createDid, createDidStorage } from "../util";

/** Demonstrates how an identity can issue and control a Token Foundry and its tokens.

For this example, we consider the case where an authority issues carbon credits
that can be used to pay for carbon emissions or traded on a marketplace. */
export async function didIssuesTokens() {
    // ===========================================
    // Create the authority's DID and the foundry.
    // ===========================================

    // Create a new Client to interact with the IOTA ledger.
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    // Create a new DID for the authority. (see "0_create_did" example).
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDidStorage(
        client,
        secretManager,
        storage,
    )
    let authorityDid = document.id();

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // We want to update the foundry counter of the authority's Alias Output, so we create an
    // updated version of the output. We pass in the previous document,
    // because we don't want to modify it in this update.
    var authorityDocument: IotaDocument = await didClient.resolveDid(authorityDid);
    const authorityAliasOutput: IAliasOutput = await didClient.updateDidOutput(authorityDocument);

    // We will add one foundry to this Alias Output.
    authorityAliasOutput.foundryCounter += 1;

    // Create a token foundry that represents carbon credits.
    const tokenScheme: ISimpleTokenScheme = {
        type: SIMPLE_TOKEN_SCHEME_TYPE,
        mintedTokens: HexHelper.fromBigInt256(bigInt(500_000)),
        meltedTokens: HexHelper.fromBigInt256(bigInt(0)),
        maximumSupply: HexHelper.fromBigInt256(bigInt(1_000_000)),
    };

    // Create the identifier of the token, which is partially derived from the Alias Address.
    const tokenId: string = TransactionHelper.constructTokenId(authorityDid.toAliasId(), 1, tokenScheme.type);

    // Create a token foundry that represents carbon credits.
    var carbonCreditsFoundry: IFoundryOutput = await client.buildFoundryOutput({
        tokenScheme,
        serialNumber: 1,
        // Initially, all carbon credits are owned by the foundry.
        nativeTokens: [
            {
                id: tokenId,
                amount: HexHelper.fromBigInt256(bigInt(500_000)),
            },
        ],
        // The authority is set as the immutable owner.
        unlockConditions: [
            {
                type: IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE,
                address: {
                    type: ALIAS_ADDRESS_TYPE,
                    aliasId: authorityDid.toAliasId(),
                },
            },
        ],
    });

    // Set the appropriate storage deposit.
    carbonCreditsFoundry.amount = TransactionHelper.getStorageDeposit(carbonCreditsFoundry, rentStructure).toString();

    // Publish the foundry.
    const [blockId, block] = await client.buildAndPostBlock(secretManager, {
        outputs: [authorityAliasOutput, carbonCreditsFoundry],
    });
    await client.retryUntilIncluded(blockId);

    // ===================================
    // Resolve foundry and its issuer DID.
    // ===================================

    // Get the latest output that contains the foundry.
    const carbonCreditsFoundryId: string = tokenId;
    const outputId: string = await client.foundryOutputId(carbonCreditsFoundryId);
    const outputResponse: IOutputResponse = await client.getOutput(outputId);
    const output: OutputTypes = outputResponse.output;

    if (output.type === FOUNDRY_OUTPUT_TYPE) {
        carbonCreditsFoundry = output;
    } else {
        throw new Error("expected foundry output");
    }

    // Get the Alias Id of the authority that issued the carbon credits foundry.
    // Non-null assertion is safe as each founry output needs to have an immutable alias unlock condition.
    const aliasUnlockCondition: IImmutableAliasUnlockCondition = carbonCreditsFoundry.unlockConditions.find(
        unlockCondition => unlockCondition.type === IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE,
    )! as IImmutableAliasUnlockCondition;

    // We know the immutable alias unlock condition contains an alias address.
    const authorityAliasId: string = (aliasUnlockCondition.address as IAliasAddress).aliasId;

    // Reconstruct the DID of the authority.
    authorityDid = IotaDID.fromAliasId(authorityAliasId, networkName);

    // Resolve the authority's DID document.
    authorityDocument = await didClient.resolveDid(authorityDid);

    console.log("The authority's DID is:", JSON.stringify(authorityDocument, null, 2));

    // =========================================================
    // Transfer 1000 carbon credits to the address of a company.
    // =========================================================

    // Create a new address that represents the company.
    const companyAddressBech32: string = (await client.generateAddresses(secretManager, {
        accountIndex: 0,
        range: {
            start: 1,
            end: 2,
        },
    }))[0];
    const companyAddress = Bech32Helper.addressFromBech32(companyAddressBech32, networkName);

    // Create a timestamp 24 hours from now.
    const tomorrow: number = Math.floor(Date.now() / 1000) + (60 * 60 * 24);

    // Create a basic output containing our carbon credits that we'll send to the company's address.
    const basicOutput: IBasicOutput = await client.buildBasicOutput({
        nativeTokens: [
            {
                amount: HexHelper.fromBigInt256(bigInt(1000)),
                id: tokenId,
            },
        ],
        // Allow the company to claim the credits within 24 hours by using an expiration unlock condition.
        unlockConditions: [
            {
                type: ADDRESS_UNLOCK_CONDITION_TYPE,
                address: companyAddress,
            },
            {
                type: EXPIRATION_UNLOCK_CONDITION_TYPE,
                unixTime: tomorrow,
                returnAddress: {
                    type: ALIAS_ADDRESS_TYPE,
                    aliasId: authorityAliasId,
                },
            },
        ],
    });

    // Reduce the carbon credits in the foundry by the amount that is sent to the company.
    carbonCreditsFoundry.nativeTokens = [
        {
            amount: HexHelper.fromBigInt256(bigInt(499_000)),
            id: tokenId,
        },
    ];

    // Publish the Basic Output and the updated foundry.
    const [blockId2, block2] = await client.buildAndPostBlock(secretManager, {
        outputs: [basicOutput, carbonCreditsFoundry],
    });
    await client.retryUntilIncluded(blockId2);

    console.log("Sent carbon credits to", companyAddressBech32);
}
