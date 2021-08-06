import * as identity from "../../web/identity_wasm.js";
import { createIdentity } from "./create_did.js";
import {
    getExplorerUrl,
    logExplorerUrlToScreen,
    logObjectToScreen,
    logToScreen,
} from "./utils.js";

/**
    This example shows how to sign/revoke verifiable credentials on scale.
    Instead of revoking the entire verification method, a single key can be revoked from a MerkleKeyCollection.
    This MerkleKeyCollection can be created as a collection of a power of 2 amount of keys.
    Every key should be used once by the issuer for signing a verifiable credential.
    When the verifiable credential must be revoked, the issuer revokes the index of the revoked key.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
    @param {boolean} log log the events to the output window
**/
export async function merkleKey(clientConfig, log = true) {
    // Create a default client configuration from the parent config network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    if (log) logToScreen("Creating identities..");

    //Creates new identities (See "create_did" example)
    const alice = await createIdentity(clientConfig, false);
    const issuer = await createIdentity(clientConfig, false);

    if (log) logObjectToScreen(alice.doc);
    if (log) logObjectToScreen(issuer.doc);

    //Add a Merkle Key Collection Verification Method with 8 keys (Must be a power of 2)
    const keys = new identity.KeyCollection(identity.KeyType.Ed25519, 8);
    const method = identity.VerificationMethod.createMerkleKey(
        identity.Digest.Sha256,
        issuer.doc.id,
        keys,
        "key-collection"
    );

    // Add to the DID Document as a general-purpose verification method
    issuer.doc.insertMethod(method, "VerificationMethod");
    issuer.doc.previousMessageId = issuer.receipt.messageId;
    issuer.doc.sign(issuer.key);

    //Publish the Identity to the IOTA Network and log the results, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(issuer.doc.toJSON());

    //Log the resulting Identity update
    const explorerUrl = getExplorerUrl(issuer.doc, receipt.messageId);
    if (log) logExplorerUrlToScreen(explorerUrl);

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

    // Sign the credential with Issuer's Merkle Key Collection method, with key index 0
    const signedVc = issuer.doc.signCredential(unsignedVc, {
        method: method.id.toString(),
        public: keys.public(0),
        secret: keys.secret(0),
        proof: keys.merkleProof(identity.Digest.Sha256, 0),
    });

    //Check the verifiable credential
    const result = await client.checkCredential(signedVc.toString());
    if (log) logToScreen(`VC verification result: ${result.verified}`);

    // The Issuer would like to revoke the credential (and therefore revokes key 0)
    issuer.doc.revokeMerkleKey(method.id.toString(), 0);
    issuer.doc.previousMessageId = receipt.messageId;
    const revokeReceipt = await client.publishDocument(issuer.doc.toJSON());

    //Log the resulting Identity update
    const revokeExplorerUrl = getExplorerUrl(issuer.doc, revokeReceipt.messageId);
    if (log) logExplorerUrlToScreen(revokeExplorerUrl);

    //Check the verifiable credential
    const newResult = await client.checkCredential(signedVc.toString());
    if (log) logToScreen(`VC verification result: ${newResult.verified}`);
}
