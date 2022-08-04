// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountBuilder,
    ExplorerUrl,
    Storage,
    IStardustIdentityClient,
    DID,
    KeyLocation,
    KeyPair,
    KeyType, StardustDocument, StardustDID, StardustIdentityClientExt, IStardustIdentityClientExt
} from '../../node';
import {ALIAS_OUTPUT_TYPE} from "@iota/iota.js/src/models/outputs/IAliasOutput";
import {IAliasOutput, IRent, IndexerPluginClient} from '@iota/iota.js';

// ===========================================================================
// Implement IStardustIdentityClient for IClient.
// ===========================================================================
interface IClient {
    getNetworkHrp: Promise<string>;

}
IClient.prototype.getNetworkHrp = async function(): Promise<string> {
    const nodeInfo = await this.info();
    return nodeInfo.protocol.bech32Hrp;
}
IClient.prototype.getAliasOutput = async function(aliasId: string): Promise<[string, IAliasOutput]> {
    // Lookup latest OutputId from the indexer plugin.
    const indexer = new IndexerPluginClient(this);
    const response = await indexer.alias(aliasId);
    if(response.items.length == 0) {
        throw new Error("AliasId '" + aliasId + "' not found");
    }
    const outputId = response.items[0];

    // Fetch AliasOutput.
    const output = await this.output(outputId).output;
    if (output.type != ALIAS_OUTPUT_TYPE) {
        throw new Error("AliasId '" + aliasId + "' returned incorrect type '" + output.type + "'");
    }
    return [outputId, output];
}
IClient.prototype.getRentStructure = async function(): Promise<IRent> {
    const nodeInfo = await this.info();
    return nodeInfo.protocol.rentStructure;
}

// declare module '@iota/iota.js' {
//     export interface IClient extends IStardustIdentityClient {}
// }

// ===========================================================================
// Implement IStardustIdentityClientExt for IClient.
// ===========================================================================
IClient.prototype.newDidOutput = async function(addressKind: number, addressHex: string, document: StardustDocument, rentStructure?: IRent): Promise<IAliasOutput> {
    return await StardustIdentityClientExt.newDidOutput(this, addressKind, addressHex, document, rentStructure);
}
IClient.prototype.updateDidOutput = async function(document: StardustDocument): Promise<IAliasOutput> {
    return await StardustIdentityClientExt.updateDidOutput(this, document);
}
IClient.prototype.resolveDid = async function(did: StardustDID): Promise<StardustDocument> {
    return await StardustIdentityClientExt.resolveDid(this, did);
}
IClient.prototype.resolveDidOutput = async function(did: StardustDID): Promise<IAliasOutput> {
    return await StardustIdentityClientExt.resolveDidOutput(this, did);
}

