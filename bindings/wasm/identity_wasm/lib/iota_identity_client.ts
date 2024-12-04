// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IIotaIdentityClient, IotaDID, IotaDocument, IotaIdentityClientExt } from "~identity_wasm";

import {
    Address,
    AddressUnlockCondition,
    AliasOutput,
    AliasOutputBuilderParams,
    Client,
    INodeInfoProtocol,
    INodeInfoWrapper,
    IRent,
    OutputResponse,
    OutputType,
    SecretManagerType,
    UTXOInput,
} from "~sdk-wasm";

/** Provides operations for IOTA DID Documents with Alias Outputs. */
export class IotaIdentityClient implements IIotaIdentityClient {
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
        const outputResponse: OutputResponse = await this.client.getOutput(outputId);
        const output = outputResponse.output;
        if (output.getType() != OutputType.Alias) {
            throw new Error("AliasId '" + aliasId + "' returned incorrect output type '" + output.getType() + "'");
        }
        // Coerce to tuple instead of an array.
        // Cast of output is fine as we checked the type earlier.
        const ret: [string, AliasOutput] = [outputId, output as AliasOutput];
        return ret;
    }

    async getRentStructure(): Promise<IRent> {
        const info: INodeInfoWrapper = await this.client.getInfo();
        return info.nodeInfo.protocol.rentStructure;
    }

    async getTokenSupply(): Promise<string> {
        return await this.client.getTokenSupply();
    }

    async getProtocolParameters(): Promise<INodeInfoProtocol> {
        const protocolParameters: INodeInfoProtocol = await this.client.getProtocolParameters();
        return protocolParameters;
    }

    /** Create a DID with a new Alias Output containing the given `document`.
     *
     * The `address` will be set as the state controller and governor unlock conditions.
     * The minimum required token deposit amount will be set according to the given
     * `rent_structure`, which will be fetched from the node if not provided.
     * The returned Alias Output can be further customized before publication, if desired.
     *
     * NOTE: this does *not* publish the Alias Output.
     */
    async newDidOutput(address: Address, document: IotaDocument, rentStructure?: IRent): Promise<AliasOutput> {
        const aliasOutputParams: AliasOutputBuilderParams = await IotaIdentityClientExt.newDidOutput(
            this,
            address,
            document,
            rentStructure,
        );
        return await this.client.buildAliasOutput(aliasOutputParams);
    }

    /** Fetches the associated Alias Output and updates it with `document` in its state metadata.
     * The storage deposit on the output is left unchanged. If the size of the document increased,
     * the amount should be increased manually.
     *
     * NOTE: this does *not* publish the updated Alias Output.
     */
    async updateDidOutput(document: IotaDocument): Promise<AliasOutput> {
        const aliasOutputParams: AliasOutputBuilderParams = await IotaIdentityClientExt.updateDidOutput(this, document);
        return await this.client.buildAliasOutput(aliasOutputParams);
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
    async deactivateDidOutput(did: IotaDID): Promise<AliasOutput> {
        const aliasOutputParams: AliasOutputBuilderParams = await IotaIdentityClientExt.deactivateDidOutput(this, did);
        return await this.client.buildAliasOutput(aliasOutputParams);
    }

    /** Resolve a {@link IotaDocument}. Returns an empty, deactivated document if the state
     * metadata of the Alias Output is empty.
     */
    async resolveDid(did: IotaDID): Promise<IotaDocument> {
        return await IotaIdentityClientExt.resolveDid(this, did);
    }

    /** Fetches the Alias Output associated with the given DID. */
    async resolveDidOutput(did: IotaDID): Promise<AliasOutput> {
        const aliasOutputParams: AliasOutputBuilderParams = await IotaIdentityClientExt.resolveDidOutput(this, did);
        return await this.client.buildAliasOutput(aliasOutputParams);
    }

    /** Publish the given `aliasOutput` with the provided `secretManager`, and returns
     * the DID document extracted from the published block.
     *
     * Note that only the state controller of an Alias Output is allowed to update its state.
     * This will attempt to move tokens to or from the state controller address to match
     * the storage deposit amount specified on `aliasOutput`.
     *
     * This method modifies the on-ledger state.
     */
    async publishDidOutput(secretManager: SecretManagerType, aliasOutput: AliasOutput): Promise<IotaDocument> {
        const networkHrp = await this.getNetworkHrp();
        // Publish block.
        const [blockId, block] = await this.client.buildAndPostBlock(secretManager, {
            outputs: [aliasOutput],
        });
        await this.client.retryUntilIncluded(blockId);

        // Extract document with computed AliasId.
        const documents = IotaDocument.unpackFromBlock(networkHrp, block);
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
    async deleteDidOutput(secretManager: SecretManagerType, address: Address, did: IotaDID) {
        const networkHrp = await this.getNetworkHrp();
        if (networkHrp !== did.network()) {
            throw new Error(
                "deleteDidOutput: DID network mismatch, client expected `" + networkHrp + "`, DID network is `"
                    + did.network() + "`",
            );
        }

        const aliasId: string = did.tag();
        const [outputId, aliasOutput] = await this.getAliasOutput(aliasId);
        const aliasInput: UTXOInput = UTXOInput.fromOutputId(outputId);

        // Send funds to the address.
        const basicOutput = await this.client.buildBasicOutput({
            amount: aliasOutput.getAmount(),
            nativeTokens: aliasOutput.getNativeTokens(),
            unlockConditions: [
                new AddressUnlockCondition(address),
            ],
        });

        // Publish block.
        const [blockId, _block] = await this.client.buildAndPostBlock(secretManager, {
            inputs: [aliasInput],
            outputs: [basicOutput],
            burn: {
                aliases: [aliasId],
            },
        });
        await this.client.retryUntilIncluded(blockId);
    }
}
