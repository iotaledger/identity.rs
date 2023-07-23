// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    KeyIdMemStore,
    Storage,
} from "@iota/identity-wasm/node";
import {
    AddressUnlockCondition,
    AliasAddress,
    AliasOutput,
    BasicOutput,
    Client,
    ExpirationUnlockCondition,
    FoundryOutput,
    ImmutableAliasAddressUnlockCondition,
    IRent,
    MnemonicSecretManager,
    Output,
    OutputResponse,
    OutputType,
    SecretManager,
    SimpleTokenScheme,
    UnlockConditionType,
    Utils,
} from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

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
        mnemonic: Utils.generateMnemonic(),
    };

    // Create a new DID for the authority. (see "0_create_did" example).
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDid(
        client,
        secretManager,
        storage,
    );
    let authorityDid = document.id();

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // We want to update the foundry counter of the authority's Alias Output, so we create an
    // updated version of the output. We pass in the previous document,
    // because we don't want to modify it in this update.
    var authorityDocument: IotaDocument = await didClient.resolveDid(authorityDid);
    var authorityAliasOutput: AliasOutput = await didClient.updateDidOutput(authorityDocument);

    // We will add one foundry to this Alias Output.
    authorityAliasOutput = await client.buildAliasOutput({
        ...authorityAliasOutput,
        foundryCounter: authorityAliasOutput.getFoundryCounter() + 1,
        aliasId: authorityAliasOutput.getAliasId(),
        unlockConditions: authorityAliasOutput.getUnlockConditions(),
    });

    // Create a token foundry that represents carbon credits.
    const tokenScheme: SimpleTokenScheme = new SimpleTokenScheme(
        BigInt(500_000),
        BigInt(0),
        BigInt(1_000_000),
    );

    // Create the identifier of the token, which is partially derived from the Alias Address.
    const tokenId: string = Utils.computeTokenId(authorityDid.toAliasId(), 1, tokenScheme.getType());

    // Create a token foundry that represents carbon credits.
    var carbonCreditsFoundry: FoundryOutput = await client.buildFoundryOutput({
        tokenScheme,
        serialNumber: 1,
        // Initially, all carbon credits are owned by the foundry.
        nativeTokens: [
            {
                id: tokenId,
                amount: BigInt(500_000),
            },
        ],
        // The authority is set as the immutable owner.
        unlockConditions: [
            new ImmutableAliasAddressUnlockCondition(
                new AliasAddress(authorityDid.toAliasId()),
            ),
        ],
    });

    // Set the appropriate storage deposit.
    carbonCreditsFoundry = await client.buildFoundryOutput({
        ...carbonCreditsFoundry,
        amount: Utils.computeStorageDeposit(carbonCreditsFoundry, rentStructure),
        tokenScheme,
        serialNumber: carbonCreditsFoundry.getSerialNumber(),
        unlockConditions: carbonCreditsFoundry.getUnlockConditions(),
    });

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
    const outputResponse: OutputResponse = await client.getOutput(outputId);
    const output: Output = outputResponse.output;

    if (output.getType() === OutputType.Foundry) {
        carbonCreditsFoundry = output as FoundryOutput;
    } else {
        throw new Error("expected foundry output");
    }

    // Get the Alias Id of the authority that issued the carbon credits foundry.
    // Non-null assertion is safe as each founry output needs to have an immutable alias unlock condition.
    const aliasUnlockCondition: ImmutableAliasAddressUnlockCondition = carbonCreditsFoundry.getUnlockConditions().find(
        unlockCondition => unlockCondition.getType() === UnlockConditionType.ImmutableAliasAddress,
    )! as ImmutableAliasAddressUnlockCondition;

    // We know the immutable alias unlock condition contains an alias address.
    const authorityAliasId: string = (aliasUnlockCondition.getAddress() as AliasAddress).getAliasId();

    // Reconstruct the DID of the authority.
    authorityDid = IotaDID.fromAliasId(authorityAliasId, networkName);

    // Resolve the authority's DID document.
    authorityDocument = await didClient.resolveDid(authorityDid);

    console.log("The authority's DID is:", JSON.stringify(authorityDocument, null, 2));

    // =========================================================
    // Transfer 1000 carbon credits to the address of a company.
    // =========================================================

    // Create a new address that represents the company.
    const companyAddressBech32: string = (await new SecretManager(secretManager).generateEd25519Addresses({
        accountIndex: 0,
        range: {
            start: 1,
            end: 2,
        },
    }))[0];
    const companyAddress = Utils.parseBech32Address(companyAddressBech32);

    // Create a timestamp 24 hours from now.
    const tomorrow: number = Math.floor(Date.now() / 1000) + (60 * 60 * 24);

    // Create a basic output containing our carbon credits that we'll send to the company's address.
    const basicOutput: BasicOutput = await client.buildBasicOutput({
        nativeTokens: [
            {
                amount: BigInt(1000),
                id: tokenId,
            },
        ],
        // Allow the company to claim the credits within 24 hours by using an expiration unlock condition.
        unlockConditions: [
            new AddressUnlockCondition(companyAddress),
            new ExpirationUnlockCondition(
                new AliasAddress(authorityAliasId),
                tomorrow,
            ),
        ],
    });

    // Reduce the carbon credits in the foundry by the amount that is sent to the company.
    carbonCreditsFoundry = await client.buildFoundryOutput({
        ...carbonCreditsFoundry,
        nativeTokens: [
            {
                amount: BigInt(499_000),
                id: tokenId,
            },
        ],
        tokenScheme,
        serialNumber: carbonCreditsFoundry.getSerialNumber(),
        unlockConditions: carbonCreditsFoundry.getUnlockConditions(),
    });

    // Publish the Basic Output and the updated foundry.
    const [blockId2, block2] = await client.buildAndPostBlock(secretManager, {
        outputs: [basicOutput, carbonCreditsFoundry],
    });
    await client.retryUntilIncluded(blockId2);

    console.log("Sent carbon credits to", companyAddressBech32);
}
