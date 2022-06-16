import {AgreementInfo, AccountBuilder, Client, MethodContent, Storage, MethodScope, EncryptionAlgorithm, CekAlgorithm, Resolver} from './../../node/identity_wasm.js';

/**
 * This example demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange
 * by encrypting and decrypting data with a shared key.
 */
async function encryption(storage?: Storage) {
    let builder = new AccountBuilder({
        storage,
    });
    
    // Alice creates and publishes their DID Document (see create_did and manipulate_did examples).
    let aliceAccount = await builder.createIdentity();
    await aliceAccount.createMethod({
        fragment: "kex-0",
        scope: MethodScope.KeyAgreement(),
        content: MethodContent.GenerateX25519(),
    })

    // Bob creates and publishes their DID Document (see create_did and manipulate_did examples).
    let bobAccount = await builder.createIdentity();
    await bobAccount.createMethod({
        fragment: "kex-0",
        scope: MethodScope.KeyAgreement(),
        content: MethodContent.GenerateX25519(),
    })

    // Alice and Bob tell each other their DIDs. They each resolve the DID Document of the other
    // to obtain their X25519 public key. Note that in practice, they would run this code completely
    // separately.
    const resolver = new Resolver();

    // Alice: resolves Bob's DID Document and extracts their public key.
    const bobDocument = await resolver.resolve(bobAccount.did());
    const bobMethod = bobDocument.intoDocument().resolveMethod("kex-0", MethodScope.KeyAgreement())!;
    const bobPublicKey = bobMethod.data().tryDecode();

    // Alice encrypts the data using Diffie-Hellman key exchange
    const agreementInfo = new AgreementInfo(Buffer.from("Alice"), Buffer.from("Bob"), new Uint8Array(0), new Uint8Array(0));
    const encryptionAlgorithm = EncryptionAlgorithm.A256GCM();
    const cekAlgorithm = CekAlgorithm.EcdhEs(agreementInfo);
    const message = Buffer.from("This msg will be encrypted and decrypted");
    const associatedData = Buffer.from("associatedData");

    const encryptedData = await aliceAccount.encryptData(message, associatedData, encryptionAlgorithm, cekAlgorithm, bobPublicKey);

    // Bob must be able to decrypt the message using the shared secret.
    const decryptedMessage = await bobAccount.decryptData(encryptedData, encryptionAlgorithm, cekAlgorithm, "kex-0");

    if(!isArrayEqual(message, decryptedMessage)) throw new Error("decrypted message does not match original message!");
    console.log(`Diffie-Hellman key exchange successful!`);
}

function isArrayEqual(a: Buffer, b: Uint8Array) {
    if(a.length !== b.length) return false;
    for(let i = 0; i < a.length; i++) {
        if(a[i] !== b[i]) return false;
    }
    return true;
}

export {encryption};