const assert = require("assert");
import { Base64, RandomHelper } from "@iota/util.js";
import {
    Ed25519,
    EdCurve,
    IJwkParams,
    Jwk,
    JwkGenOutput,
    JwkOperation,
    JwkStorage,
    JwkType,
    JwkUse,
    JwsAlgorithm,
    KeyPair,
    KeyType,
} from "../node";

describe("#memstore", function() {
    it("should work", async () => {
        const testData = Uint8Array.from([0xff, 0xee, 0xdd, 0xcc]);
        const memstore = new JwkMemStore();

        let genOutput = await memstore.generate(JwkMemStore.ed25519KeyType(), JwsAlgorithm.EdDSA);
        const keyId = genOutput.keyId();
        const jwk = genOutput.jwk();
        assert.ok(genOutput.jwk());
        assert.ok(keyId);

        const signature = await memstore.sign(keyId, testData, jwk.toPublic());
        assert.deepStrictEqual(signature.length, Ed25519.SIGNATURE_LENGTH());

        const publicJwk = await memstore.public(keyId);
        assert.deepStrictEqual(publicJwk.toJSON(), jwk.toPublic().toJSON());

        assert.ok(await memstore.exists(keyId));
        assert.ok(!await memstore.exists("non-existent-key-id"));

        assert.doesNotReject(async () => {
            await memstore.delete(keyId);
        });
        assert.rejects(async () => {
            await memstore.delete("non-existent-key-id");
        });

        const jwkParams: IJwkParams = {
            kty: JwkType.Okp,
            use: JwkUse.Signature,
            alg: JwsAlgorithm.EdDSA,
            key_ops: [JwkOperation.Sign, JwkOperation.Verify],
            crv: EdCurve.Ed25519,
            d: "nWGxne_9WmC6hEr0kuwsxERJxWl7MmkZcDusAxyuf2A",
            x: "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo",
        };

        const localJwk = new Jwk(jwkParams);
        assert.ok(await memstore.insert(localJwk));

        const pubLocalJwk = new Jwk({
            ...jwkParams,
            // Null out the private key component
            d: undefined,
        });

        // INVALID: Inserting a JWK without the private key component should fail.
        assert.rejects(async () => {
            await memstore.insert(pubLocalJwk);
        });
    });
});

export class JwkMemStore implements JwkStorage {
    /** The map from key identifiers to Jwks. */
    private _keys: Map<string, Jwk>;

    /** Creates a new, empty `MemStore` instance. */
    constructor() {
        this._keys = new Map();
    }

    public static ed25519KeyType(): string {
        return "Ed25519";
    }

    public async generate(keyType: string, algorithm: JwsAlgorithm): Promise<JwkGenOutput> {
        if (keyType !== JwkMemStore.ed25519KeyType()) {
            throw new Error(`unsupported key type ${keyType}`);
        }

        if (algorithm !== JwsAlgorithm.EdDSA) {
            throw new Error(`unsupported algorithm`);
        }

        const keyId = randomKeyId();
        const keyPair = new KeyPair(KeyType.Ed25519);

        const jwk = encodeJwk(keyPair, algorithm);

        this._keys.set(keyId, jwk);

        return new JwkGenOutput(keyId, jwk);
    }

    public async sign(keyId: string, data: Uint8Array, publicKey: Jwk): Promise<Uint8Array> {
        if (publicKey.alg() !== JwsAlgorithm.EdDSA) {
            throw new Error("unsupported JWS algorithm");
        } else {
            if (publicKey.paramsOkp()?.crv !== (EdCurve.Ed25519 as string)) {
                throw new Error("unsupported Okp parameter");
            }
        }

        const jwk = this._keys.get(keyId);

        if (jwk) {
            const keyPair = decodeJwk(jwk);
            return Ed25519.sign(data, keyPair.private());
        } else {
            throw new Error(`key with id ${keyId} not found`);
        }
    }

    public async insert(jwk: Jwk): Promise<string> {
        const keyId = randomKeyId();

        if (!jwk.isPrivate) {
            throw new Error("expected a JWK with all private key components set");
        }

        if (!jwk.alg()) {
            throw new Error("expected a Jwk with an `alg` parameter");
        }

        this._keys.set(keyId, jwk);

        return keyId;
    }

    public async public(keyId: string): Promise<Jwk> {
        const jwk = this._keys.get(keyId);

        if (jwk) {
            return jwk.toPublic();
        } else {
            throw new Error("key not found");
        }
    }

    public async delete(keyId: string): Promise<void> {
        this._keys.delete(keyId);
    }

    public async exists(keyId: string): Promise<boolean> {
        return this._keys.has(keyId);
    }
}

// Encodes a Ed25519 keypair into a Jwk.
function encodeJwk(keyPair: KeyPair, alg: JwsAlgorithm): Jwk {
    let x = Base64.encode(keyPair.public());
    let d = Base64.encode(keyPair.private());

    return new Jwk({
        "kty": JwkType.Okp,
        "crv": "Ed25519",
        d,
        x,
        alg,
    });
}

function decodeJwk(jwk: Jwk): KeyPair {
    if (jwk.alg() !== JwsAlgorithm.EdDSA) {
        throw new Error("unsupported `alg`");
    }

    const paramsOkp = jwk.paramsOkp();
    if (paramsOkp) {
        const d = paramsOkp.d;

        if (d) {
            const secret = Base64.decode(d);
            const pub = Base64.decode(paramsOkp.x);
            return KeyPair.fromKeys(KeyType.Ed25519, pub, secret);
        } else {
            throw new Error("missing private key component");
        }
    } else {
        throw new Error("expected Okp params");
    }
}

// Returns a random key id.
function randomKeyId(): string {
    return Base64.encode(RandomHelper.generate(32));
}
