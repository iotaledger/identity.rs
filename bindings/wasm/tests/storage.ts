const assert = require("assert");
import { Base64, RandomHelper } from "@iota/util.js";
import {
    CoreDocument,
    Ed25519,
    EdCurve,
    IJwkParams,
    IotaDocument,
    Jwk,
    JwkGenOutput,
    JwkOperation,
    JwkType,
    JwkUse,
    JwsAlgorithm,
    MethodDigest,
    MethodScope,
    Storage,
    VerificationMethod,
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

        await doc.generateMethod(
            storage,
            JwkMemStore.ed25519KeyType(),
            JwsAlgorithm.EdDSA,
            "#key-1",
            MethodScope.VerificationMethod(),
        );
        // Check that we can resolve the generated method.
        let method = doc.resolveMethod("#key-1");
        assert.deepStrictEqual(method instanceof VerificationMethod, true);
    });
});
