import * as identity from "../../web/identity_wasm.js";
import { createVC } from "./create_vc.js";
import { logObjectToScreen, logToScreen } from "./utils.js";

/**
    This example shows how to create a Verifiable Presentation and validate it.
    A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
    It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
    @param {boolean} log log the events to the output window
**/
export async function createVP(clientConfig, log = true) {
    if (log) logToScreen("creating Verifiable Presentation...");

    // Create a default client configuration from mainNet.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // Creates new identities (See "createVC" example)
    const { alice, issuer, signedVc } = await createVC(clientConfig, false);

    // Create a Verifiable Presentation from the Credential - signed by Alice's key
    // TODO: Sign with a challenge
    const unsignedVp = new identity.VerifiablePresentation(
        alice.doc,
        signedVc.toJSON()
    );

    const signedVp = alice.doc.signPresentation(unsignedVp, {
        method: "#key",
        secret: alice.key.secret,
    });

    if (log) logToScreen("signed VP:");
    if (log) logObjectToScreen(signedVp);

    // Check the validation status of the Verifiable Presentation
    const checkResult = await client.checkPresentation(signedVp.toString());

    if (log) logToScreen(`VP verification result: ${checkResult.verified}`);

    return { alice, issuer, signedVc, signedVp, checkResult };
}
