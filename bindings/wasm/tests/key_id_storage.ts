const assert = require("assert");
import { CoreDID, KeyIdStorage, KeyPair, KeyType, MethodDigest, VerificationMethod } from "../node";

describe("Method digest", () => {
    it("should have consistent hashing", () => {
        let verificationMethodJson = {
            id: "did:example:HHoh9NQC9AUsK15Jyyq53VTujxEUizKDXRXd7zbT1B5u#frag_1",
            controller: "did:example:HHoh9NQC9AUsK15Jyyq53VTujxEUizKDXRXd7zbT1B5u",
            type: "Ed25519VerificationKey2018",
            publicKeyMultibase: "zHHoh9NQC9AUsK15Jyyq53VTujxEUizKDXRXd7zbT1B5u",
        };

        let verificationMethod = VerificationMethod.fromJSON(verificationMethodJson);
        let methodDigest = new MethodDigest(verificationMethod);

        let packed = methodDigest.pack();
        // Packed bytes must be consistent between Rust and Wasm, see Rust tests for `MethodDigest`.
        let packedExpected = new Uint8Array([0, 74, 60, 10, 199, 76, 205, 180, 133]);
        assert.deepStrictEqual(packed, packedExpected);
    });
});

describe("Key Id Storage", () => {
    it("should work", async () => {
        const KEY_ID = "my-key-id";
        let vm: VerificationMethod = createVerificationMethod();
        let methodDigest: MethodDigest = new MethodDigest(vm);

        let memstore = new KeyIdMemStore();

        // Deletion of non saved key id results in error.
        assert.rejects(memstore.deleteKeyId(methodDigest));

        // Store key id.
        await memstore.insertKeyId(methodDigest, KEY_ID);

        // Double insertion results in error.
        assert.rejects(memstore.insertKeyId(methodDigest, KEY_ID));

        // Restore key id from a `MethodDigest` with the same data but not the same reference.
        let methodDigestClone = MethodDigest.unpack(methodDigest.pack());
        let key_id_restored: string = await memstore.getKeyId(methodDigestClone);

        // Check restored key id.
        assert.equal(KEY_ID, key_id_restored);

        // Delete stored key id.
        await memstore.deleteKeyId(methodDigest);

        // Double deletion results in error.
        assert.rejects(memstore.deleteKeyId(methodDigest));
    });
});

function createVerificationMethod(): VerificationMethod {
    let id = CoreDID.parse("did:example:abc123");
    let keypair = new KeyPair(KeyType.Ed25519);
    let method = new VerificationMethod(id, keypair.type(), keypair.public(), "#key-1");
    return method;
}

/**
 * Converts a `MethodDigest` to a base64 encoded string.
 */
function methodDigestToString(methodDigest: MethodDigest): string {
    let arrayBuffer = methodDigest.pack().buffer;
    let buffer = Buffer.from(arrayBuffer);
    return buffer.toString("base64");
}

/**
 * Creates a `MethodDigest` from a base64 encoded string.
 */
function stringToMethodDigest(input: string): MethodDigest {
    let buffer = Buffer.from(input, "base64");
    let byteArray = Uint8Array.from(buffer);
    return MethodDigest.unpack(byteArray);
}

class KeyIdMemStore implements KeyIdStorage {
    private _keyIds: Map<string, string>;

    constructor() {
        this._keyIds = new Map<string, string>();
    }

    public async insertKeyId(methodDigest: MethodDigest, keyId: string): Promise<void> {
        let methodDigestAsString: string = methodDigestToString(methodDigest);
        let value = this._keyIds.get(methodDigestAsString);
        if (value !== undefined) {
            throw new Error("KeyId already exists");
        }
        this._keyIds.set(methodDigestAsString, keyId);
    }

    public async getKeyId(methodDigest: MethodDigest): Promise<string> {
        let methodDigestAsString: string = methodDigestToString(methodDigest);
        let value = this._keyIds.get(methodDigestAsString);
        if (value == undefined) {
            throw new Error("KeyId not found");
        }
        return value;
    }

    public async deleteKeyId(methodDigest: MethodDigest): Promise<void> {
        let methodDigestAsString: string = methodDigestToString(methodDigest);
        let success = this._keyIds.delete(methodDigestAsString);
        if (success) {
            return;
        } else {
            throw new Error("KeyId not found!");
        }
    }
}
