// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Config, Timestamp} from '@iota/identity-wasm';
import {createVC} from './create_vc';
import {logExplorerUrl, logResolverUrl} from './utils';

/**
 This example shows how to revoke a verifiable credential.
 The Verifiable Credential is revoked by actually removing a verification method (public key) from the DID Document of the Issuer.
 As such, the Verifiable Credential can no longer be validated.
 This would invalidate every Verifiable Credential signed with the same public key, therefore the issuer would have to sign every VC with a different key.
 Have a look at the Merkle Key example on how to do that practically.

 Note that this example uses the "main" network, if you are writing code against the test network then most function
 calls will need to include information about the network, since this is not automatically inferred from the
 arguments in all cases currently.

 We recommend that you ALWAYS using a CLIENT_CONFIG parameter that you define when calling any functions that take a
 ClientConfig object. This will ensure that all the API calls use a consistent node and network.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function revokeVC(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates new identities and a VC (see "create_vc" example)
    const {alice, issuer, signedVc} = await createVC(clientConfig);

    // Remove the public key that signed the VC - effectively revoking the VC as it will no longer be able to verify
    issuer.doc.removeMethod(issuer.doc.id.toUrl().join("#newKey"));
    issuer.doc.previousMessageId = issuer.updatedMessageId;
    issuer.doc.updated = Timestamp.nowUTC();
    issuer.doc.signSelf(issuer.key, issuer.doc.defaultSigningMethod().id.toString());
    // This is an integration chain update, so we publish the full document.
    const {messageId} = await client.publishDocument(issuer.doc);

    // Log the resulting Identity update
    logExplorerUrl("Issuer Update Transaction:", clientConfig.explorer, messageId);
    logResolverUrl("Explore the Issuer DID Document:", clientConfig.explorer, issuer.doc.id.toString());

    // Check the verifiable credential
    const result = await client.checkCredential(signedVc.toString());
    console.log(`VC verification result (false = revoked): ${result.verified}`);
}

export {revokeVC};
