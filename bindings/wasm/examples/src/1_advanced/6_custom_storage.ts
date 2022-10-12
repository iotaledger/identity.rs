import { Base58 } from "@iota/util.js";
import {
    CoreDocument,
    CoreDocumentRc,
    CoreVerificationMethod,
    Credential,
    KeyAlias,
    KeyStorage,
    KeyType,
    MethodScope,
    MethodType1,
    ProofOptions,
    ProofValue,
    Signable,
    SignatureHandler,
    SignatureSuite,
} from "../../../node";

import { MemStore } from "./memStore";

/** Demonstrates how to ...
 */
export async function customStorage() {
    const memStore = new MemStore();

    const fragment = "#key-2";
    var document = new CoreDocument({ id: "did:iota:0x0002" });

    // const documentRc = new CoreDocumentRc(document);
    // await documentRc.createMethod({
    //     fragment,
    //     content: MethodContent.Generate(),
    //     type: MethodType1.ed25519VerificationKey2018(),
    // });
    // document = documentRc.intoDocument();

    let keyAlias = await memStore.generate("Ed25519");

    let keyPublic = await memStore.public(keyAlias);
    let method = new CoreVerificationMethod(document.id(), KeyType.Ed25519, keyPublic, fragment);
    document.insertMethod(method, MethodScope.VerificationMethod());

    let handlerMap: Map<string, SignatureHandler> = new Map();
    handlerMap.set(MethodType1.ed25519VerificationKey2018().toString(), new JcsEd25519Signature());

    const credential = testCredential();
    const signable = Signable.Credential(credential);
    const signatureSuite = new SignatureSuite(memStore, handlerMap);
    await document.sign(
        signable,
        fragment,
        signatureSuite,
        new ProofOptions({ challenge: "1234-5678-0000" }),
    );

    console.log(JSON.stringify(signable.toJSON(), null, 2));
}

function testCredential(): Credential {
    const subjectDid = "did:iota:0x0001";
    const issuerDid = "did:iota:0x0002";

    const subject = {
        id: subjectDid,
        name: "Alice",
        degree: "Bachelor of Science and Arts",
        GPA: "4.0",
    };

    return new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuerDid,
        credentialSubject: subject,
    });
}

class JcsEd25519Signature implements SignatureHandler {
    async sign(value: Signable, keyStorage: KeyStorage): Promise<ProofValue> {
        // Hardcoded keyAlias while we don't have a mechanism to store the mappings from method fragments to key aliases.
        const keyAlias = new KeyAlias("very_random_key");

        // TODO: Not a proper JCS serialization because POC.
        const encoder = new TextEncoder();
        const json = encoder.encode(JSON.stringify(value.toJSON()));
        const proof: Uint8Array = await keyStorage.sign(keyAlias, "Ed25519", json);

        const signature: string = Base58.encode(proof);
        return ProofValue.Signature(signature);
    }

    signatureName(): string {
        return "JcsEd25519Signature2020";
    }
}
