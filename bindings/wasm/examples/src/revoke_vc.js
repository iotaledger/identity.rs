// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    Credential,
    CredentialValidationOptions,
    CredentialValidator,
    FailFast,
    Resolver,
    Timestamp
} from '@iota/identity-wasm';
import {createVC} from './create_vc';

/**
 This example shows how to revoke a verifiable credential.
 The Verifiable Credential is revoked by actually removing a verification method (public key) from the DID Document of the Issuer.
 As such, the Verifiable Credential can no longer be validated.
 This would invalidate every Verifiable Credential signed with the same public key, therefore the issuer would have to sign every VC with a different key.
 Have a look at the Merkle Key example on how to do that practically.

 Note that this example uses the "main" network, if you are writing code against the test network then most function
 calls will need to include information about the network, since this is not automatically inferred from the
 arguments in all cases currently.

 We recommend that you ALWAYS use a CLIENT_CONFIG parameter that you define when calling any functions that take a
 ClientConfig object. This will ensure that all the API calls use a consistent node and network.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function revokeVC(clientConfig) {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    // Creates new identities and a VC (see "create_vc" example)
    const {alice, issuer, credentialJSON} = await createVC(clientConfig);
    const signedVc = Credential.fromJSON(credentialJSON);

    // Remove the public key that signed the VC - effectively revoking the VC as it will no longer be able to verify
    issuer.doc.removeMethod(issuer.doc.id().toUrl().join("#newKey"));
    issuer.doc.setMetadataPreviousMessageId(issuer.updatedMessageId);
    issuer.doc.setMetadataUpdated(Timestamp.nowUTC());
    issuer.doc.signSelf(issuer.key, issuer.doc.defaultSigningMethod().id());
    // This is an integration chain update, so we publish the full document.
    const receipt = await client.publishDocument(issuer.doc);
    console.log(`published document`);
    // Log the resulting Identity update
    console.log(`Issuer Update Transaction: ${clientConfig.explorer.messageUrl(receipt.messageId())}`);
    console.log(`Explore the Issuer DID Document: ${clientConfig.explorer.resolverUrl(issuer.doc.id())}`);

    // Check the verifiable credential.
    const resolver = await Resolver
        .builder()
        .client(client)
        .build();
    let vc_revoked = false;
    try {
        // Resolve the issuer's updated DID Document to ensure the key was revoked successfully.
        const resolvedIssuerDoc = await resolver.resolveCredentialIssuer(signedVc);
        CredentialValidator.validate(
            signedVc,
            resolvedIssuerDoc,
            CredentialValidationOptions.default(),
            FailFast.FirstError
        );
    } catch (exception) {
        console.log(`${exception.message}`)
        vc_revoked = true;
    }
    if (!vc_revoked) throw new Error("VC not revoked");
    console.log(`Credential successfully revoked!`);

}

export {revokeVC};
