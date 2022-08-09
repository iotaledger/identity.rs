// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {IStardustIdentityClient, StardustDID, StardustDocument, StardustIdentityClientExt} from '../../node';

import {
    ADDRESS_UNLOCK_CONDITION_TYPE,
    AddressTypes,
    ALIAS_ADDRESS_TYPE,
    ALIAS_OUTPUT_TYPE,
    ALIAS_UNLOCK_TYPE,
    BASIC_OUTPUT_TYPE,
    Bech32Helper,
    DEFAULT_PROTOCOL_VERSION,
    ED25519_ADDRESS_TYPE,
    ED25519_SIGNATURE_TYPE,
    Ed25519Address,
    FOUNDRY_OUTPUT_TYPE,
    GOVERNOR_ADDRESS_UNLOCK_CONDITION_TYPE,
    IAliasAddress,
    IAliasOutput,
    IAliasUnlock,
    IBasicOutput,
    IBlock,
    IClient,
    IKeyPair,
    IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE,
    IndexerPluginClient,
    INftAddress,
    INftUnlock,
    IOutputMetadataResponse,
    IOutputsResponse,
    IReferenceUnlock,
    IRent,
    ISignatureUnlock,
    ITransactionEssence,
    ITransactionPayload,
    IUTXOInput,
    NFT_ADDRESS_TYPE,
    NFT_OUTPUT_TYPE,
    NFT_UNLOCK_TYPE,
    OutputTypes,
    promote,
    reattach,
    REFERENCE_UNLOCK_TYPE,
    serializeAddress,
    serializeOutput,
    SIGNATURE_UNLOCK_TYPE,
    STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE,
    TRANSACTION_ESSENCE_TYPE,
    TRANSACTION_PAYLOAD_TYPE,
    TransactionHelper,
    TREASURY_OUTPUT_TYPE,
    UnlockTypes
} from '@iota/iota.js';
import {Converter, WriteStream} from "@iota/util.js";
import {Blake2b, Ed25519} from "@iota/crypto.js";

const PUBLISH_RETRY_INTERVAL_MS: number = 5000;
const PUBLISH_RETRY_MAX_ATTEMPTS: number = 20;

/** Provides operations for IOTA UTXO DID Documents with Alias Outputs. */
export class StardustIdentityClient implements IStardustIdentityClient {
    client: IClient;
    indexer: IndexerPluginClient;

    constructor(client: IClient) {
        this.client = client;
        this.indexer = new IndexerPluginClient(client);
    }

    async getNetworkHrp() {
        const nodeInfo = await this.client.protocolInfo();
        return nodeInfo.bech32Hrp;
    }

    async getAliasOutput(aliasId: string) {
        // Lookup latest OutputId from the indexer plugin.
        const aliasResponse = await this.indexer.alias(aliasId);
        if (aliasResponse.items.length == 0) {
            throw new Error("AliasId '" + aliasId + "' not found");
        }
        const outputId = aliasResponse.items[0];

        // Fetch AliasOutput.
        const outputResponse = await this.client.output(outputId);
        const output = outputResponse.output;
        if (output.type != ALIAS_OUTPUT_TYPE) {
            throw new Error("AliasId '" + aliasId + "' returned incorrect type '" + output.type + "'");
        }
        // Coerce to tuple instead of an array.
        const ret: [string, IAliasOutput] = [outputId, output];
        return ret;
    }

    async getRentStructure() {
        const nodeInfo = await this.client.info();
        return nodeInfo.protocol.rentStructure;
    }

    /** Create a DID with a new Alias Output containing the given `document`.
     *
     * The `address` will be set as the state controller and governor unlock conditions.
     * The minimum required token deposit amount will be set according to the given
     * `rent_structure`, which will be fetched from the node if not provided.
     * The returned Alias Output can be further customised before publication, if desired.
     *
     * NOTE: this does *not* publish the Alias Output.
     */
    async newDidOutput(address: AddressTypes, document: StardustDocument, rentStructure?: IRent): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.newDidOutput(this, address, document, rentStructure);
    }

    /** Fetches the associated Alias Output and updates it with `document` in its state metadata.
     * The storage deposit on the output is left unchanged. If the size of the document increased,
     * the amount should be increased manually.
     *
     * NOTE: this does *not* publish the updated Alias Output.
     */
    async updateDidOutput(document: StardustDocument): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.updateDidOutput(this, document);
    }

    /** Removes the DID document from the state metadata of its Alias Output,
     * effectively deactivating it. The storage deposit on the output is left unchanged,
     * and should be reallocated manually.
     *
     * Deactivating does not destroy the output. Hence, it can be re-activated by publishing
     * an update containing a DID document.
     *
     * NOTE: this does *not* publish the updated Alias Output.
     */
    async deactivateDidOutput(did: StardustDID): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.deactivateDidOutput(this, did);
    }

    /** Resolve a {@link StardustDocument}. Returns an empty, deactivated document if the state
     * metadata of the Alias Output is empty.
     */
    async resolveDid(did: StardustDID): Promise<StardustDocument> {
        return await StardustIdentityClientExt.resolveDid(this, did);
    }

    /** Fetches the Alias Output associated with the given DID. */
    async resolveDidOutput(did: StardustDID): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.resolveDidOutput(this, did);
    }

    /** Publish the given `aliasOutput` with the provided `walletKeyPair`, and returns
     * the DID document extracted from the published block.
     *
     * Note that only the state controller of an Alias Output is allowed to update its state.
     * This will attempt to move tokens to or from the state controller address to match
     * the storage deposit amount specified on `aliasOutput`.
     *
     * This method modifies the on-ledger state.
     */
    async publishDidOutput(walletKeyPair: IKeyPair, aliasOutput: IAliasOutput): Promise<StardustDocument> {
        const networkHrp = await this.getNetworkHrp();

        const consumedOutputs: [string, OutputTypes][] = [];
        const outputs: OutputTypes[] = [aliasOutput];

        // Check if tokens need to be transferred to or from the output.
        let consumeAmount: bigint = BigInt(0);
        let remainderAmount: bigint = BigInt(0);
        if (aliasOutput.stateIndex === 0) {
            consumeAmount = BigInt(aliasOutput.amount);
        } else {
            const previousAlias: [string, IAliasOutput] = await this.getAliasOutput(aliasOutput.aliasId);
            const previousAmount = BigInt(previousAlias[1].amount);
            const nextAmount = BigInt(aliasOutput.amount)
            if (nextAmount > previousAmount) {
                consumeAmount = nextAmount - previousAmount;
            } else {
                remainderAmount = previousAmount - nextAmount;
            }

            // Consume previous Alias Output.
            consumedOutputs.push(previousAlias);
        }

        // Get the wallet address, which is the Blake2b-256 digest of the public key.
        const walletEd25519Address = new Ed25519Address(walletKeyPair.publicKey);
        const walletAddress = walletEd25519Address.toAddress();

        // Note: reclaiming funds from an Alias Output may cause a dust output with insufficient storage deposit,
        // so we consume a Basic Output with sufficient funds and merge it with the remainder tokens when
        // consumeAmount is zero and there is a remainder.
        if (consumeAmount > BigInt(0) || (consumeAmount === BigInt(0) && remainderAmount > BigInt(0))) {
            // Get tokens from wallet.
            const walletAddressBech32 = Bech32Helper.toBech32(ED25519_ADDRESS_TYPE, walletAddress, networkHrp);
            const walletOutput: [string, IBasicOutput] = await fetchBasicOutputWithAmount(walletAddressBech32, consumeAmount, this);
            // Mark any excess funds for return.
            // Special case: if consumeAmount is zero, this returns the full amount (to prevent a dust output).
            if (BigInt(walletOutput[1].amount) > consumeAmount) {
                remainderAmount = remainderAmount + BigInt(walletOutput[1].amount) - consumeAmount;
            }

            // Consume wallet output.
            consumedOutputs.push(walletOutput);
        }

        // Send remainder tokens to wallet.
        if (remainderAmount > BigInt(0)) {
            const walletAddressHex = Converter.bytesToHex(walletAddress, true);
            const walletOutput: IBasicOutput = {
                type: BASIC_OUTPUT_TYPE,
                amount: remainderAmount.toString(),
                nativeTokens: [],
                unlockConditions: [
                    {
                        type: ADDRESS_UNLOCK_CONDITION_TYPE,
                        address: {
                            type: ED25519_ADDRESS_TYPE,
                            pubKeyHash: walletAddressHex
                        }
                    }
                ],
                features: []
            };
            outputs.push(walletOutput);
        }

        // Check outputs have an amount sufficient to cover the storage deposit.
        const rentStructure: IRent = await this.getRentStructure();
        for (const output of outputs) {
            const minimumAmount: number = TransactionHelper.getStorageDeposit(output, rentStructure);
            if (BigInt(output.amount) < BigInt(minimumAmount)) {
                throw new Error("publishDidOutput: output amount " + output.amount + " insufficient to cover minimum storage deposit " + minimumAmount.toString());
            }
        }

        // Check input and output totals match.
        let totalConsumed: bigint = BigInt(0);
        let totalOutput: bigint = BigInt(0);
        for (const consumedOutput of consumedOutputs) {
            totalConsumed += BigInt(consumedOutput[1].amount);
        }
        for (const output of outputs) {
            totalOutput += BigInt(output.amount);
        }
        if (totalConsumed !== totalOutput) {
            throw new Error("publishDidOutput: total consumed amount " + totalConsumed + " != total output amount" + totalOutput);
        }

        // Construct block.
        const sortedConsumedOutputs = sortConsumedOutputs(consumedOutputs, outputs);
        const essence: ITransactionEssence = await prepareTransactionEssence(this.client, sortedConsumedOutputs, outputs);
        const payload: ITransactionPayload = await signTransactionPayload(this, walletKeyPair, sortedConsumedOutputs, outputs, essence);
        const block: IBlock = await prepareBlock(this.client, payload);

        // Extract document with computed AliasId.
        const documents = extractDocumentsFromBlock(networkHrp, block);
        if (documents.length < 1) {
            throw new Error("publishDidOutput: no DID document in transaction payload, aborting publishing");
        }

        // Publish block.
        const blockId = await this.client.blockSubmit(block);
        await retryUntilIncluded(this.client, blockId, PUBLISH_RETRY_INTERVAL_MS, PUBLISH_RETRY_MAX_ATTEMPTS);

        // Checked for non-zero length above.
        return documents[0];
    }

    /** Destroy the Alias Output containing the given `did`, sending its tokens to a new Basic Output
     * unlockable by the given address.
     *
     * Note that only the governor of an Alias Output is allowed to destroy it.
     *
     * ### WARNING
     *
     * This destroys the Alias Output and DID document, rendering them permanently unrecoverable.
     */
    async deleteDidOutput(address: AddressTypes, walletKeyPair: IKeyPair, did: StardustDID) {
        const networkHrp = await this.getNetworkHrp();
        if (networkHrp !== did.networkStr()) {
            throw new Error("deleteDidOutput: DID network mismatch, client expected `" + networkHrp + "`, DID network is `" + did.networkStr() + "`");
        }

        const aliasId: string = did.tag();
        const [outputId, aliasOutput] = await this.getAliasOutput(aliasId);

        // Send funds to the address.
        const fundsOutput: IBasicOutput = {
            type: BASIC_OUTPUT_TYPE,
            amount: aliasOutput.amount,
            nativeTokens: aliasOutput.nativeTokens,
            unlockConditions: [
                {
                    type: ADDRESS_UNLOCK_CONDITION_TYPE,
                    address: address
                }
            ],
            features: []
        };

        // Construct block.
        const consumedOutputs: [string, OutputTypes][] = [[outputId, aliasOutput]];
        const outputs: OutputTypes[] = [fundsOutput];
        const sortedConsumedOutputs = sortConsumedOutputs(consumedOutputs, outputs);
        const essence: ITransactionEssence = await prepareTransactionEssence(this.client, sortedConsumedOutputs, outputs);
        const payload: ITransactionPayload = await signTransactionPayload(this, walletKeyPair, sortedConsumedOutputs, outputs, essence);
        const block: IBlock = await prepareBlock(this.client, payload);

        // Publish block.
        const blockId = await this.client.blockSubmit(block);
        await retryUntilIncluded(this.client, blockId, PUBLISH_RETRY_INTERVAL_MS, PUBLISH_RETRY_MAX_ATTEMPTS);
    }
}

/** Sort inputs so those with Ed25519 address unlock conditions are first.
 *  This is important and assumed to be the order when preparing the unlocks during signing.
 *
 *  Also appends the relevant unlock address to each input.
 */
function sortConsumedOutputs(consumedOutputs: [string, OutputTypes][], outputs: OutputTypes[]): [string, OutputTypes, AddressTypes][] {
    const toSort: [string, OutputTypes, AddressTypes, Uint8Array][] = [];
    for (const consumedOutput of consumedOutputs) {
        const unlockAddress: AddressTypes = extractUnlockConditionAddress(consumedOutput[0], consumedOutput[1], outputs);

        // Sort key is the packed bytes of the address, cannot use string comparison for this.
        const w = new WriteStream();
        serializeAddress(w, unlockAddress);
        const addressBytes: Uint8Array = w.finalBytes();

        toSort.push([consumedOutput[0], consumedOutput[1], unlockAddress, addressBytes]);
    }
    // Sort inputs so Ed25519 address unlocks are first.
    const sorted = toSort.sort((a, b) => {
        if (a[3] < b[3]) {
            return -1;
        } else if (a[3] > b[3]) {
            return 1;
        } else {
            return 0;
        }
    })
    return sorted.map((value, _index, _array) => {
        return [value[0], value[1], value[2]]
    });
}

async function prepareTransactionEssence(client: IClient, consumedOutputs: [string, OutputTypes, AddressTypes][], outputs: OutputTypes[]): Promise<ITransactionEssence> {
    const inputs: IUTXOInput[] = [];
    for (const consumedOutput of consumedOutputs) {
        const input: IUTXOInput = TransactionHelper.inputFromOutputId(consumedOutput[0]);
        inputs.push(input);
    }

    // Compute inputs commitment digest.
    const inputsCommitmentHasher = new Blake2b(Blake2b.SIZE_256);
    // Hash list of input hashes (the actual output objects they reference).
    for (const consumedOutput of consumedOutputs) {
        const w = new WriteStream();
        const outputHasher = new Blake2b(Blake2b.SIZE_256);
        serializeOutput(w, consumedOutput[1]);
        const consumedOutputBytes = w.finalBytes();
        outputHasher.update(consumedOutputBytes);
        const outputHash = outputHasher.final();

        inputsCommitmentHasher.update(outputHash);
    }
    // Calculate sum from buffer.
    const inputsCommitment: string = Converter.bytesToHex(inputsCommitmentHasher.final(), true);

    // Creating Transaction Essence
    const protocolInfo = await client.protocolInfo();
    return {
        type: TRANSACTION_ESSENCE_TYPE,
        networkId: protocolInfo.networkId,
        inputs,
        outputs,
        inputsCommitment,
    }
}

/** Extract the address required to unlock this output when consuming it. */
function extractUnlockConditionAddress(outputId: string, consumedOutput: OutputTypes, outputs: OutputTypes[]): AddressTypes {
    let unlockAddress: AddressTypes | null = null;
    switch (consumedOutput.type) {
        case TREASURY_OUTPUT_TYPE: {
            throw new Error("extractUnlockConditionAddress: treasury output is not supported");
        }
        case BASIC_OUTPUT_TYPE: {
            // BasicOutput must have an AddressUnlockCondition.
            for (const condition of consumedOutput.unlockConditions) {
                if (condition.type == ADDRESS_UNLOCK_CONDITION_TYPE) {
                    unlockAddress = condition.address;
                }
            }
            break;
        }
        case ALIAS_OUTPUT_TYPE: {
            // Check if governance transition (state index not incremented).
            let aliasId: string;
            if (consumedOutput.stateIndex === 0) {
                aliasId = computeAliasIdFromOutputId(outputId);
            } else {
                aliasId = consumedOutput.aliasId;
            }
            const nextAliasOutput: OutputTypes | undefined = outputs.find((output => {
                if (output.type === ALIAS_OUTPUT_TYPE) {
                    return output.aliasId === aliasId;
                }
                return false;
            }));
            let governanceTransition = false;
            if (nextAliasOutput === undefined || (nextAliasOutput.type === ALIAS_OUTPUT_TYPE && nextAliasOutput.stateIndex == consumedOutput.stateIndex)) {
                governanceTransition = true;
            }
            if (governanceTransition) {
                // Governor transition, AliasOutput must have a GovernorAddressUnlockCondition.
                for (const condition of consumedOutput.unlockConditions) {
                    if (condition.type == GOVERNOR_ADDRESS_UNLOCK_CONDITION_TYPE) {
                        unlockAddress = condition.address;
                    }
                }
            } else {
                // State controller transition, AliasOutput must have a StateControllerAddressUnlockCondition.
                for (const condition of consumedOutput.unlockConditions) {
                    if (condition.type == STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE) {
                        unlockAddress = condition.address;
                    }
                }
            }
            break;
        }
        case FOUNDRY_OUTPUT_TYPE: {
            // FoundryOutput must have an ImmutableAliasAddressUnlockCondition
            for (const condition of consumedOutput.unlockConditions) {
                if (condition.type == IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE) {
                    unlockAddress = condition.address;
                }
            }
            break;
        }
        case NFT_OUTPUT_TYPE: {
            // NftOutput must have an AddressUnlockCondition.
            for (const condition of consumedOutput.unlockConditions) {
                if (condition.type == ADDRESS_UNLOCK_CONDITION_TYPE) {
                    unlockAddress = condition.address;
                }
            }
            break;
        }
        default: {
            throw new Error("extractUnlockConditionAddress: unknown output type");
        }
    }
    if (unlockAddress === null) {
        throw new Error("extractUnlockConditionAddress: cannot extract unlock address for consumed output type " + consumedOutput.type);
    }
    return unlockAddress;
}

/** Creates and signs unlocks then constructs a transaction payload from them to be included in a block.
 *
 *  NOTE: consumedOutput MUST be sorted by
 *
 *  This is derived from the `iota.rs` implementation.
 *  https://github.com/iotaledger/iota.rs/blob/8e8715cc4fd863d6c68ddaea046891b4641654c6/src/secret/mod.rs#L288
 */
async function signTransactionPayload(didClient: StardustIdentityClient, walletKeyPair: IKeyPair, consumedOutputs: [string, OutputTypes, AddressTypes][], outputs: OutputTypes[], essence: ITransactionEssence): Promise<ITransactionPayload> {
    const unlocks: UnlockTypes[] = [];
    const unlockAddressesMap = new Map<string, number>();

    /** Computes a string for the given address usable for map key equality. */
    function addressMapKey(address: AddressTypes): string {
        switch (address.type) {
            case ED25519_ADDRESS_TYPE: {
                return "ed25519:" + address.pubKeyHash;
            }
            case ALIAS_ADDRESS_TYPE: {
                return "alias:" + address.aliasId;
            }
            case NFT_ADDRESS_TYPE: {
                return "nft:" + address.nftId;
            }
            default: {
                throw new Error("signTransactionPayload: unknown address type");
            }
        }
    }

    // Compute Transaction Essence Hash (to be signed in signature unlocks).
    const essenceHash = TransactionHelper.getTransactionEssenceHash(essence);

    for (let index = 0; index < consumedOutputs.length; index += 1) {
        const [outputId, consumedOutput, inputAddress] = consumedOutputs[index];
        // Don't need to unlock spent outputs?
        const metadataResponse: IOutputMetadataResponse = await didClient.client.outputMetadata(outputId);
        if (metadataResponse.isSpent) {
            continue;
        }

        // Check if we already added an unlock for this address.
        const unlockIndex: number | undefined = unlockAddressesMap.get(addressMapKey(inputAddress));
        if (unlockIndex !== undefined) {
            const type = inputAddress.type;
            switch (type) {
                case ED25519_ADDRESS_TYPE: {
                    let unlock: IReferenceUnlock = {
                        type: REFERENCE_UNLOCK_TYPE,
                        reference: unlockIndex
                    }
                    unlocks.push(unlock);
                    break;
                }
                case ALIAS_ADDRESS_TYPE: {
                    let unlock: IAliasUnlock = {
                        type: ALIAS_UNLOCK_TYPE,
                        reference: unlockIndex
                    }
                    unlocks.push(unlock);
                    break;
                }
                case NFT_ADDRESS_TYPE: {
                    let unlock: INftUnlock = {
                        type: NFT_UNLOCK_TYPE,
                        reference: unlockIndex
                    }
                    unlocks.push(unlock);
                    break;
                }
                default: {
                    throw new Error("signTransactionPayload: unknown address type " + type);
                }
            }
        } else {
            // We can only sign Ed25519 addresses and the map needs to contain the Alias or NFT
            // address already at this point, because the reference index needs to be lower
            // than the current block index
            // NOTE: this ordering is ensured by `sortConsumedOutputs`.
            if (inputAddress.type !== ED25519_ADDRESS_TYPE) {
                throw new Error("signTransactionPayload: missing input with Ed25519 unlock condition");
            }
            let unlock: ISignatureUnlock = {
                type: SIGNATURE_UNLOCK_TYPE,
                signature: {
                    type: ED25519_SIGNATURE_TYPE,
                    publicKey: Converter.bytesToHex(walletKeyPair.publicKey, true),
                    signature: Converter.bytesToHex(Ed25519.sign(walletKeyPair.privateKey, essenceHash), true)
                }
            };
            unlocks.push(unlock);

            // Add the Ed25519 address to the address map, so it gets referenced if further inputs have
            // the same address in their unlock condition.
            unlockAddressesMap.set(addressMapKey(inputAddress), index);
        }

        // When we have an Alias or NFT output, we will add their address to the map,
        // because they can be used to unlock outputs that have the corresponding
        // Alias or NFT address in their unlock condition.
        switch (consumedOutput.type) {
            case ALIAS_OUTPUT_TYPE: {
                let aliasId: string;
                if (consumedOutput.stateIndex === 0) {
                    aliasId = computeAliasIdFromOutputId(outputId);
                } else {
                    aliasId = consumedOutput.aliasId;
                }
                const outputAddress: IAliasAddress = {
                    type: ALIAS_ADDRESS_TYPE,
                    aliasId: consumedOutput.aliasId
                };
                unlockAddressesMap.set(addressMapKey(outputAddress), index);
                break;
            }
            case NFT_OUTPUT_TYPE: {
                let nftId: string;
                if (isHexNull(consumedOutput.nftId)) {
                    // NftId derivation is the same as AliasId.
                    nftId = computeAliasIdFromOutputId(outputId);
                } else {
                    nftId = consumedOutput.nftId;
                }
                const outputAddress: INftAddress = {
                    type: NFT_ADDRESS_TYPE,
                    nftId: nftId
                }
                unlockAddressesMap.set(addressMapKey(outputAddress), index);
                break;
            }
            default: {
                // Do nothing.
            }
        }
    }

    // Construct Transaction Payload.
    return {
        type: TRANSACTION_PAYLOAD_TYPE,
        essence: essence,
        unlocks
    };
}

/** Constructs a block to publish the given payload. */
async function prepareBlock(client: IClient, payload: ITransactionPayload): Promise<IBlock> {
    // Select parents for the block.
    let parentsResponse = await client.tips();
    let parents = parentsResponse.tips;

    return {
        protocolVersion: DEFAULT_PROTOCOL_VERSION,
        parents: parents,
        payload,
        nonce: "0" // set by proof-of-work provider.
    };
}

/** Promotes or re-attaches the given block id until it's included (referenced by a milestone).
 *
 *  This is derived from the `iota.rs` implementation:
 *  https://github.com/iotaledger/iota.rs/blob/128283b14e6476b2fc497d2e4fd27028277a3a59/src/client.rs#L529
 */
async function retryUntilIncluded(client: IClient, blockId: string, intervalMs: number, maxAttempts: number) {
    // Track blocks, since re-attaching a block might produce more than one.
    const blockIds: string[] = [blockId];

    for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
        // Sleep.
        await new Promise(f => setTimeout(f, intervalMs));

        const length = blockIds.length;
        for (let index = 0; index < length; index += 1) {
            const currentBlockId = blockIds[index];

            // Check if block is already included successfully.
            const metadata = await client.blockMetadata(currentBlockId);
            if (metadata.ledgerInclusionState === "included" || metadata.ledgerInclusionState === "noTransaction") {
                return;
            }

            // Only promote or re-attach the latest attachment of the block.
            if (index == blockIds.length - 1) {
                if (metadata.shouldPromote) {
                    await promote(client, currentBlockId);
                } else if (metadata.shouldReattach) {
                    const reattached = await reattach(client, currentBlockId);
                    // Only track new reattached blocks; promoted blocks are empty and just attempt to confirm the
                    // original block.
                    blockIds.push(reattached.blockId);
                }
            }
        }
    }
}

/** Check whether the given hex-encoded string (with a prefix or not) is all zeroes. */
function isHexNull(hex: string): boolean {
    const bytes: Uint8Array = Converter.hexToBytes(hex);
    return bytes.every(element => element == 0);
}

/** Compute the AliasId as a prefix-hex encoded string. */
function computeAliasId(transactionIdHex: string, outputIndex: number): string {
    const outputIdHex: string = TransactionHelper.outputIdFromTransactionData(transactionIdHex, outputIndex);
    return computeAliasIdFromOutputId(outputIdHex);
}

/** Compute the AliasId as a prefix-hex encoded string. */
function computeAliasIdFromOutputId(outputIdHex: string): string {
    // Blake2b-256 digest of output id.
    const outputIdBytes: Uint8Array = Converter.hexToBytes(outputIdHex);
    const digest: Uint8Array = Blake2b.sum256(outputIdBytes);
    return Converter.bytesToHex(digest, true);
}

/** Extract all DID documents of the Alias Outputs contained in a transaction payload, if any. */
function extractDocumentsFromBlock(networkHrp: string, block: IBlock): StardustDocument[] {
    const documents: StardustDocument[] = [];

    if (block.payload === undefined || block.payload?.type !== TRANSACTION_PAYLOAD_TYPE) {
        throw new Error("failed to extract documents from block, transaction payload missing or wrong type");
    }
    const payload: ITransactionPayload = block.payload;

    // Compute TransactionId.
    const transactionPayloadHash: Uint8Array = TransactionHelper.getTransactionPayloadHash(payload);
    const transactionId: string = Converter.bytesToHex(transactionPayloadHash, true);

    // Loop over Alias Outputs.
    const outputs: OutputTypes[] = payload.essence.outputs;
    for (let index = 0; index < outputs.length; index += 1) {
        const output = outputs[index];
        if (output.type !== ALIAS_OUTPUT_TYPE) {
            continue;
        }

        // Compute Alias Id.
        let aliasIdHex: string;
        if (output.stateIndex === 0) {
            aliasIdHex = computeAliasId(transactionId, index);
        } else {
            aliasIdHex = output.aliasId;
        }
        const aliasId: Uint8Array = Converter.hexToBytes(aliasIdHex);

        // Unpack document.
        const did: StardustDID = new StardustDID(aliasId, networkHrp);
        let stateMetadata: Uint8Array;
        if (output.stateMetadata === undefined) {
            stateMetadata = new Uint8Array(0);
        } else {
            stateMetadata = Converter.hexToBytes(output.stateMetadata);
        }
        documents.push(StardustDocument.unpack(did, stateMetadata, true));
    }
    return documents;
}

/** Attempt to fetch a Basic Output with at least the minimum specified token amount.
 *
 *  If multiple blocks satisfy the minimum, returns the block that matches exactly, or else the one with the largest
 *  amount (to try to avoid creating an output below the dust threshold).
 */
async function fetchBasicOutputWithAmount(addressBech32: string, minAmount: bigint, didClient: StardustIdentityClient): Promise<[string, IBasicOutput]> {
    // Fetch Basic Output ids on this address from indexer plugin.
    let outputsResponse: IOutputsResponse = {ledgerIndex: 0, cursor: "", pageSize: "", items: []};
    let maxTries = 5;
    let tries = 0;
    while (outputsResponse.items.length == 0 && tries < maxTries) {
        tries++;

        // Basic Outputs with no special conditions.
        outputsResponse = await didClient.indexer.outputs({
            addressBech32: addressBech32,
            hasStorageReturnCondition: false,
            hasExpirationCondition: false,
            hasTimelockCondition: false,
            hasNativeTokens: false,
        });
        if (outputsResponse.items.length == 0) {
            await new Promise(f => setTimeout(f, 1000));
        }
    }
    if (tries > maxTries) {
        throw new Error("failed to find any Basic Outputs for address " + addressBech32);
    }

    // Fetch all Basic Outputs from client.
    const basicOutputs: [string, IBasicOutput][] = [];
    for (const outputId of outputsResponse.items) {
        const basicOutput: OutputTypes = (await didClient.client.output(outputId)).output;
        if (basicOutput.type !== BASIC_OUTPUT_TYPE) {
            continue;
        }
        basicOutputs.push([outputId, basicOutput]);
    }

    // Select Basic Output matching amount exactly, otherwise one with the largest amount.
    let matchOutput: [string, IBasicOutput] | null = null;
    for (const [outputId, output] of basicOutputs) {
        const outputAmount = BigInt(output.amount);
        if (outputAmount === minAmount) {
            // Exact match.
            matchOutput = [outputId, output];
            break;
        } else if (outputAmount > minAmount && (matchOutput == null || BigInt(matchOutput[1].amount) < outputAmount)) {
            // Largest amount.
            matchOutput = [outputId, output];
        }
    }
    if (matchOutput === null) {
        throw new Error("failed to find a Basic Output with at least " + minAmount + " tokens, consolidate or deposit more tokens for address " + addressBech32);
    }
    return matchOutput;
}
