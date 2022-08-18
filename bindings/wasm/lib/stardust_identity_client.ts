// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// @ts-ignore: path is set to match runtime transpiled js path when bundled.
import {IStardustIdentityClient, StardustDID, StardustDocument, StardustIdentityClientExt} from './identity_wasm';

// NOTE: this import path is replaced with `/web` in the `build/web.js` script.
import type {Client, INodeInfoWrapper, SecretManager} from '@cycraig/iota-client-wasm/node';
import {
    ADDRESS_UNLOCK_CONDITION_TYPE,
    AddressTypes,
    ALIAS_OUTPUT_TYPE,
    IAliasOutput,
    IBlock,
    IOutputResponse,
    IRent,
    ITransactionPayload,
    IUTXOInput,
    OutputTypes,
    TRANSACTION_PAYLOAD_TYPE,
    TransactionHelper
} from '@iota/iota.js';
import {Converter} from '@iota/util.js';
import {Blake2b} from '@iota/crypto.js';

/** Provides operations for IOTA UTXO DID Documents with Alias Outputs. */
export class StardustIdentityClient implements IStardustIdentityClient {
    client: Client;

    constructor(client: Client) {
        this.client = client;
    }

    async getNetworkHrp() {
        return await this.client.getBech32Hrp();
    }

    async getAliasOutput(aliasId: string) {
        // Lookup latest OutputId from the indexer plugin.
        const outputId = await this.client.aliasOutputId(aliasId);

        // Fetch AliasOutput.
        const outputResponse: IOutputResponse = await this.client.getOutput(outputId);
        const output = outputResponse.output;
        if (output.type != ALIAS_OUTPUT_TYPE) {
            throw new Error("AliasId '" + aliasId + "' returned incorrect output type '" + output.type + "'");
        }
        // Coerce to tuple instead of an array.
        const ret: [string, IAliasOutput] = [outputId, output];
        return ret;
    }

    async getRentStructure() {
        const info: INodeInfoWrapper = await this.client.getInfo();
        return info.nodeInfo.protocol.rentStructure;
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

    /** Publish the given `aliasOutput` with the provided ` `, and returns
     * the DID document extracted from the published block.
     *
     * Note that only the state controller of an Alias Output is allowed to update its state.
     * This will attempt to move tokens to or from the state controller address to match
     * the storage deposit amount specified on `aliasOutput`.
     *
     * This method modifies the on-ledger state.
     */
    async publishDidOutput(secretManager: SecretManager, aliasOutput: IAliasOutput): Promise<StardustDocument> {
        const networkHrp = await this.getNetworkHrp();

        // Publish block.
        const [blockId, block] = await this.client.buildAndPostBlock(secretManager, {
            outputs: [aliasOutput],
        });
        await this.client.retryUntilIncluded(blockId);

        // Extract document with computed AliasId.
        const documents = extractDocumentsFromBlock(networkHrp, block);
        if (documents.length < 1) {
            throw new Error("publishDidOutput: no DID document in transaction payload");
        }
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
    async deleteDidOutput(secretManager: SecretManager, address: AddressTypes, did: StardustDID) {
        const networkHrp = await this.getNetworkHrp();
        if (networkHrp !== did.networkStr()) {
            throw new Error("deleteDidOutput: DID network mismatch, client expected `" + networkHrp + "`, DID network is `" + did.networkStr() + "`");
        }

        const aliasId: string = did.tag();
        const [outputId, aliasOutput] = await this.getAliasOutput(aliasId);
        const aliasInput: IUTXOInput = TransactionHelper.inputFromOutputId(outputId);

        // Send funds to the address.
        const basicOutput = await this.client.buildBasicOutput({
            amount: aliasOutput.amount,
            nativeTokens: aliasOutput.nativeTokens,
            unlockConditions: [
                {
                    type: ADDRESS_UNLOCK_CONDITION_TYPE,
                    address: address
                }
            ],
        })

        // Publish block.
        const [blockId, _block] = await this.client.buildAndPostBlock(secretManager, {
            inputs: [aliasInput],
            outputs: [basicOutput]
        });
        await this.client.retryUntilIncluded(blockId);
    }
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
