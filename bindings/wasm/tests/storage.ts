const assert = require("assert");
import { RandomHelper } from "@iota/util.js";
import {
    CoreDocument,
    Ed25519,
    EdCurve,
    IJwkParams,
    IJwsSignatureVerifier,
    IotaDocument,
    Jwk,
    JwkGenOutput,
    JwkOperation,
    JwkType,
    JwkUse,
    JwsAlgorithm,
    JwsSignatureOptions,
    JwsVerificationOptions,
    MethodDigest,
    MethodScope,
    Storage,
    VerificationMethod,
    verifyEdDSA,
} from "../node";
import { JwkMemStore } from "./jwk_storage";
import { createVerificationMethod, KeyIdMemStore } from "./key_id_storage";

describe("#JwkStorageDocument", function() {
    it("storage getters should work", async () => {
        const keystore = new JwkMemStore();
        // Put some data in the keystore
        let genOutput = await keystore.generate(JwkMemStore.ed25519KeyType(), JwsAlgorithm.EdDSA);
        const keyId = genOutput.keyId();
        const jwk = genOutput.jwk();
        assert.ok(genOutput.jwk());
        assert.ok(keyId);

        const keyIdStore = new KeyIdMemStore();
        // Put some data in the keyIdStore
        let vm: VerificationMethod = createVerificationMethod();
        let methodDigest: MethodDigest = new MethodDigest(vm);
        keyIdStore.insertKeyId(methodDigest, keyId);

        // Create new storage
        const storage = new Storage(keystore, keyIdStore);

        // Check that we can retrieve the separate storages and their state is preserved
        const retrievedKeyIdStore = storage.keyIdStorage();
        assert.deepStrictEqual(retrievedKeyIdStore instanceof KeyIdMemStore, true);

        const retrievedKeyId = await retrievedKeyIdStore.getKeyId(methodDigest);
        assert.deepStrictEqual(keyId as string, retrievedKeyId);

        const retrievedKeyStore = storage.keyStorage();
        assert.deepStrictEqual(retrievedKeyStore instanceof JwkMemStore, true);
        assert.deepStrictEqual(await retrievedKeyStore.exists(retrievedKeyId), true);
    });

    it("The JwkStorageDocument extension should work: CoreDocument", async () => {
        const keystore = new JwkMemStore();
        const keyIdStore = new KeyIdMemStore();
        const storage = new Storage(keystore, keyIdStore);
        const VALID_DID_EXAMPLE = "did:example:123";
        const doc = new CoreDocument({
            id: VALID_DID_EXAMPLE,
        });
        const fragment = "#key-1";
        await doc.generateMethod(
            storage,
            JwkMemStore.ed25519KeyType(),
            JwsAlgorithm.EdDSA,
            fragment,
            MethodScope.VerificationMethod(),
        );
        // Check that we can resolve the generated method.
        let method = doc.resolveMethod(fragment);
        assert.deepStrictEqual(method instanceof VerificationMethod, true);

        // Check that signing works
        let testString = "test";
        const jws = await doc.createJwt(storage, fragment, testString, new JwsSignatureOptions());

        // Verify the signature and obtain a decoded token.
        const token = doc.verifyJws(jws, new JwsVerificationOptions());
        assert.deepStrictEqual(testString, token.claims());

        // Check that we can also verify it using a custom verifier
        let customVerifier = new CustomVerifier();
        const tokenFromCustomVerification = doc.verifyJws(jws, new JwsVerificationOptions(), customVerifier);
        assert.deepStrictEqual(token.toJSON(), tokenFromCustomVerification.toJSON());
        assert.deepStrictEqual(customVerifier.verifications(), 1);

        // Delete the method
        const methodId = (method as VerificationMethod).id();
        await doc.purgeMethod(storage, methodId);
        // Check that the method can no longer be resolved.
        assert.deepStrictEqual(doc.resolveMethod(fragment), undefined);
        // The storage should now be empty
        assert.deepStrictEqual((storage.keyIdStorage() as KeyIdMemStore).count(), 0);
        assert.deepStrictEqual((storage.keyStorage() as JwkMemStore).count(), 0);
    });

    it("The JwkStorageDocument extension should work: IotaDocument", async () => {
        const keystore = new JwkMemStore();
        const keyIdStore = new KeyIdMemStore();
        const storage = new Storage(keystore, keyIdStore);
        const networkName = "smr";
        const doc = new IotaDocument(networkName);
        const fragment = "#key-1";
        await doc.generateMethod(
            storage,
            JwkMemStore.ed25519KeyType(),
            JwsAlgorithm.EdDSA,
            fragment,
            MethodScope.VerificationMethod(),
        );
        // Check that we can resolve the generated method.
        let method = doc.resolveMethod(fragment);
        assert.deepStrictEqual(method instanceof VerificationMethod, true);

        // Check that signing works.
        let testString = "test";
        const jws = await doc.createJwt(storage, fragment, testString, new JwsSignatureOptions());

        // Verify the signature and obtain a decoded token.
        const token = doc.verifyJws(jws, new JwsVerificationOptions());
        assert.deepStrictEqual(testString, token.claims());

        // Check that we can also verify it using a custom verifier
        let customVerifier = new CustomVerifier();
        const tokenFromCustomVerification = doc.verifyJws(jws, new JwsVerificationOptions(), customVerifier);
        assert.deepStrictEqual(token.toJSON(), tokenFromCustomVerification.toJSON());
        assert.deepStrictEqual(customVerifier.verifications(), 1);

        // Delete the method
        const methodId = (method as VerificationMethod).id();
        await doc.purgeMethod(storage, methodId);
        // Check that the method can no longer be resolved.
        assert.deepStrictEqual(doc.resolveMethod(fragment), undefined);
        // The storage should now be empty
        assert.deepStrictEqual((storage.keyIdStorage() as KeyIdMemStore).count(), 0);
        assert.deepStrictEqual((storage.keyStorage() as JwkMemStore).count(), 0);
    });
});

class CustomVerifier implements IJwsSignatureVerifier {
    private _verifications: number;

    constructor() {
        this._verifications = 0;
    }

    public verifications(): number {
        return this._verifications;
    }

    public verify(alg: JwsAlgorithm, signingInput: Uint8Array, decodedSignature: Uint8Array, publicKey: Jwk): void {
        verifyEdDSA(alg, signingInput, decodedSignature, publicKey);
        this._verifications += 1;
        return;
    }
}
