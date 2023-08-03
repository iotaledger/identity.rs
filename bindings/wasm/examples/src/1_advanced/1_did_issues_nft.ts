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
    Address,
    AddressType,
    AddressUnlockCondition,
    AliasAddress,
    Client,
    FeatureType,
    IRent,
    IssuerFeature,
    MetadataFeature,
    MnemonicSecretManager,
    NftOutput,
    Output,
    OutputResponse,
    OutputType,
    Payload,
    PayloadType,
    RegularTransactionEssence,
    TransactionEssenceType,
    TransactionPayload,
    utf8ToHex,
    Utils,
} from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

/** Demonstrates how an identity can issue and own NFTs,
and how observers can verify the issuer of the NFT.

For this example, we consider the case where a manufacturer issues
a digital product passport (DPP) as an NFT. */
export async function didIssuesNft() {
    // ==============================================
    // Create the manufacturer's DID and the DPP NFT.
    // ==============================================

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

    // Create a new DID for the manufacturer. (see "0_create_did" example).
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDid(
        client,
        secretManager,
        storage,
    );
    let manufacturerDid = document.id();

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // Create the Alias Address of the manufacturer.
    const manufacturerAliasAddress: Address = new AliasAddress(
        manufacturerDid.toAliasId(),
    );

    // Create a Digital Product Passport NFT issued by the manufacturer.
    let productPassportNft: NftOutput = await client.buildNftOutput({
        nftId: "0x0000000000000000000000000000000000000000000000000000000000000000",
        immutableFeatures: [
            // Set the manufacturer as the immutable issuer.
            new IssuerFeature(manufacturerAliasAddress),
            // A proper DPP would hold its metadata here.
            new MetadataFeature(utf8ToHex("Digital Product Passport Metadata")),
        ],
        unlockConditions: [
            // The NFT will initially be owned by the manufacturer.
            new AddressUnlockCondition(manufacturerAliasAddress),
        ],
    });

    // Set the appropriate storage deposit.
    productPassportNft = await client.buildNftOutput({
        ...productPassportNft,
        amount: Utils.computeStorageDeposit(productPassportNft, rentStructure),
        nftId: productPassportNft.getNftId(),
        unlockConditions: productPassportNft.getUnlockConditions(),
    });

    // Publish the NFT.
    const [blockId, block] = await client.buildAndPostBlock(secretManager, { outputs: [productPassportNft] });
    await client.retryUntilIncluded(blockId);

    // ========================================================
    // Resolve the Digital Product Passport NFT and its issuer.
    // ========================================================

    // Extract the identifier of the NFT from the published block.
    // Non-null assertion is safe because we published a block with a payload.
    let nftId: string = computeNftOutputId(block.payload!);

    // Fetch the NFT Output.
    const nftOutputId: string = await client.nftOutputId(nftId);
    const outputResponse: OutputResponse = await client.getOutput(nftOutputId);
    const output: Output = outputResponse.output;

    // Extract the issuer of the NFT.
    let manufacturerAliasId: string;
    if (output.getType() === OutputType.Nft && (output as NftOutput).getImmutableFeatures()) {
        // Cast is fine as we checked the type.
        const nftOutput: NftOutput = output as NftOutput;
        // Non-null assertion is fine as we checked the immutable features are present.
        const issuerFeature: IssuerFeature = nftOutput.getImmutableFeatures()!.find(feature =>
            feature.getType() === FeatureType.Issuer
        ) as IssuerFeature;
        if (issuerFeature && issuerFeature.getIssuer().getType() === AddressType.Alias) {
            manufacturerAliasId = (issuerFeature.getIssuer() as AliasAddress).getAliasId();
        } else {
            throw new Error("expected to find issuer feature with an alias address");
        }
    } else {
        throw new Error("expected NFT output with immutable features");
    }

    // Reconstruct the manufacturer's DID from the Alias Id.
    manufacturerDid = IotaDID.fromAliasId(manufacturerAliasId, networkName);

    // Resolve the issuer of the NFT.
    const manufacturerDocument: IotaDocument = await didClient.resolveDid(manufacturerDid);

    console.log("The issuer of the Digital Product Passport NFT is:", JSON.stringify(manufacturerDocument, null, 2));
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
