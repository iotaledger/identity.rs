import * as identity from "../../web/identity_wasm.js";
import { manipulateIdentity } from "./mainpulate_did.js";
import { createIdentity } from "./create_did.js";
import { logObjectToScreen, logToScreen } from "./utils.js";

/**
    This example shows how to create a Verifiable Credential and validate it.
    In this example, alice takes the role of the subject, while we also have an issuer.
    The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
    This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with whoever they please.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
    @param {boolean} log log the events to the output window
**/
export async function createVC(clientConfig, log = true) {
    if (log) logToScreen("Verifiable Credential creation started...");

    // Create a default client configuration from mainNet.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // Creates new identities (See "create_did" and "manipulate_did" examples)
    const alice = await createIdentity(clientConfig, false);
    const issuer = await manipulateIdentity(clientConfig, false);

    if (log) logToScreen("Holder (Alice):");
    if (log) logObjectToScreen(alice);

    if (log) logToScreen("Issuer:");
    if (log) logObjectToScreen(issuer);

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: alice.doc.id.toString(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0",
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = identity.VerifiableCredential.extend({
        id: "http://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.doc.id.toString(),
        credentialSubject,
    });

    // Sign the credential with the Issuer's newKey
    const signedVc = issuer.doc.signCredential(unsignedVc, {
        method: issuer.doc.id.toString() + "#newKey",
        public: issuer.newKey.public,
        secret: issuer.newKey.secret,
    });

    if (log) logToScreen("VC:");
    if (log) logObjectToScreen(signedVc);

    // Check if the credential is verifiable.
    const checkResult = await client.checkCredential(signedVc.toString());

    if (log) logToScreen("Check credential result: " + checkResult.verified);

    return { alice, issuer, signedVc, checkResult };
}
