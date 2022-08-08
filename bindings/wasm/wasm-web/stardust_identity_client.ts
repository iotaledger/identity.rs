// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IStardustIdentityClient,
    IStardustIdentityClientExt,
    StardustDID,
    StardustDocument,
    StardustIdentityClientExt
} from './identity_wasm';
import {ALIAS_OUTPUT_TYPE, IAliasOutput, IClient, IndexerPluginClient, IRent} from '@iota/iota.js';

/** Provides operations for IOTA UTXO DID Documents with Alias Outputs. */
export class StardustIdentityClient implements IStardustIdentityClient, IStardustIdentityClientExt {
    client: IClient;
    indexer: IndexerPluginClient;

    constructor(client: IClient) {
        this.client = client;
        this.indexer = new IndexerPluginClient(client);
    }

    async getNetworkHrp() {
        const nodeInfo = await this.client.info();
        return nodeInfo.protocol.bech32Hrp;
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

    async newDidOutput(addressType: number, addressHex: string, document: StardustDocument, rentStructure?: IRent): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.newDidOutput(this, addressType, addressHex, document, rentStructure);
    }

    async updateDidOutput(document: StardustDocument): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.updateDidOutput(this, document);
    }

    async resolveDid(did: StardustDID): Promise<StardustDocument> {
        return await StardustIdentityClientExt.resolveDid(this, did);
    }

    async resolveDidOutput(did: StardustDID): Promise<IAliasOutput> {
        return await StardustIdentityClientExt.resolveDidOutput(this, did);
    }

    /// TODO: helper functions for publishing, deactivation, deletion.

}
