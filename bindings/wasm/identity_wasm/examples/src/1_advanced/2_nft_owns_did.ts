// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDocument, IotaIdentityClient } from "@iota/identity-wasm/node";
import {
    Address,
    AddressType,
    AddressUnlockCondition,
    AliasOutput,
    Client,
    IRent,
    MnemonicSecretManager,
    NftAddress,
    NftOutput,
    Output,
    OutputResponse,
    OutputType,
    Payload,
    PayloadType,
    RegularTransactionEssence,
    SecretManager,
    StateControllerAddressUnlockCondition,
    TransactionEssenceType,
    TransactionPayload,
    UnlockConditionType,
    Utils,
} from "@iota/sdk-wasm/node";
import { API_ENDPOINT, ensureAddressHasFunds } from "../util";

/** Demonstrates how an identity can be owned by NFTs,
and how observers can verify that relationship.

For this example, we consider the case where a car's NFT owns
the DID of the car, so that transferring the NFT also transfers DID ownership. */
export async function nftOwnsDid() {
    // =============================
    // Create the car's NFT and DID.
    // =============================

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

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // Create a new address that will own the NFT.
    const addressBech32 = (await new SecretManager(secretManager).generateEd25519Addresses({
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: networkName,
    }))[0];
    const address = Utils.parseBech32Address(addressBech32);

    // Get funds for testing from the faucet.
    await ensureAddressHasFunds(client, addressBech32);

    // Create the car NFT with an Ed25519 address as the unlock condition.
    let carNft: NftOutput = await client.buildNftOutput({
        nftId: "0x0000000000000000000000000000000000000000000000000000000000000000",
        unlockConditions: [
            // The NFT will initially be owned by the Ed25519 address.
            new AddressUnlockCondition(address),
        ],
    });

    // Set the appropriate storage deposit.
    carNft = await client.buildNftOutput({
        ...carNft,
        amount: Utils.computeStorageDeposit(carNft, rentStructure),
        nftId: carNft.getNftId(),
        unlockConditions: carNft.getUnlockConditions(),
    });

    // Publish the NFT.
    const [blockId, block] = await client.buildAndPostBlock(secretManager, { outputs: [carNft] });
    await client.retryUntilIncluded(blockId);

    // Extract the identifier of the NFT from the published block.
    // Non-null assertion is safe because we published a block with a payload.
    var carNftId: string = computeNftOutputId(block.payload!);

    // Create the address of the NFT.
    const nftAddress: Address = new NftAddress(carNftId);

    // Construct a DID document for the car.
    var carDocument: IotaDocument = new IotaDocument(networkName);

    // Create a new DID for the car that is owned by the car NFT.
    var carDidAliasOutput: AliasOutput = await didClient.newDidOutput(nftAddress, carDocument, rentStructure);

    // Publish the car DID.
    carDocument = await didClient.publishDidOutput(secretManager, carDidAliasOutput);

    // ============================================
    // Determine the car's NFT given the car's DID.
    // ============================================

    // Resolve the Alias Output of the DID.
    carDidAliasOutput = await didClient.resolveDidOutput(carDocument.id());

    // Extract the NFT Id from the state controller unlock condition.
    const stateControllerUnlockCondition: StateControllerAddressUnlockCondition = carDidAliasOutput
        .getUnlockConditions()
        .find(feature =>
            feature.getType() === UnlockConditionType.StateControllerAddress
        ) as StateControllerAddressUnlockCondition;
    if (stateControllerUnlockCondition.getAddress().getType() === AddressType.Nft) {
        carNftId = (stateControllerUnlockCondition.getAddress() as NftAddress).getNftId();
    } else {
        throw new Error("expected nft address unlock condition");
    }

    // Fetch the NFT Output of the car.
    const nftOutputId: string = await client.nftOutputId(carNftId);
    const outputResponse: OutputResponse = await client.getOutput(nftOutputId);
    const output: Output = outputResponse.output;

    if (output.getType() === OutputType.Nft) {
        carNft = output as NftOutput;
    } else {
        throw new Error("expected nft output type");
    }

    console.log("The car's DID is:", JSON.stringify(carDocument, null, 2));
    console.log("The car's NFT is:", JSON.stringify(carNft, null, 2));
}

function computeNftOutputId(payload: Payload): string {
    if (payload.getType() === PayloadType.Transaction) {
        const transactionPayload: TransactionPayload = payload as TransactionPayload;
        const transactionId = Utils.transactionId(transactionPayload);

        if (transactionPayload.essence.getType() === TransactionEssenceType.Regular) {
            const regularTxPayload = transactionPayload.essence as RegularTransactionEssence;
            const outputs = regularTxPayload.outputs;
            for (const index in outputs) {
                if (outputs[index].getType() === OutputType.Nft) {
                    const outputId: string = Utils.computeOutputId(transactionId, parseInt(index));
                    return Utils.computeNftId(outputId);
                }
            }
            throw new Error("no NFT output in transaction essence");
        } else {
            throw new Error("expected transaction essence");
        }
    } else {
        throw new Error("expected transaction payload");
    }
}
