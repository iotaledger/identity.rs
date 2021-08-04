import * as identity from "../../web/identity_wasm.js";
import { createVC } from "./create_vc.js";
import {
    getExplorerUrl,
    logExplorerUrlToScreen,
    logToScreen,
} from "./utils.js";

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

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
    @param {boolean} log log the events to the output window
**/
export async function revoke(clientConfig, log = true) {
    // Create a default client configuration from the parent config network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    //Creates new identities (See "create_did" and "manipulate_did" examples)
    const { alice, issuer, signedVc } = await createVC(clientConfig, log);

    if (log) logToScreen("Revoking VC...");

    //Remove the public key that signed the VC - effectively revoking the VC as it will no longer be able to verify
    issuer.doc.removeMethod(
        identity.DID.parse(issuer.doc.id.toString() + "#newKey")
    );
    issuer.doc.previousMessageId = issuer.receipt.messageId;
    issuer.doc.sign(issuer.key);
    const {messageId} = await client.publishDocument(issuer.doc.toJSON());

    //Log the resulting Identity update
    const explorerUrl = getExplorerUrl(issuer.doc, messageId);
    if (log) logExplorerUrlToScreen(explorerUrl);

    //Check the verifiable credential
    const result = await client.checkCredential(signedVc.toString());

    if (log) logToScreen(`VC verification result: ${result.verified}`);
}
