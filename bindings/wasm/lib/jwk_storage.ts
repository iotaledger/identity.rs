import { RandomHelper } from "@iota/util.js";
import {
    decodeB64,
    Ed25519,
    encodeB64,
    Jwk,
    JwkGenOutput,
    JwkStorage,
    KeyPair,
    KeyType,
} from "~identity_wasm";

import {
    EdCurve,
    JwkType,
    JwsAlgorithm,
} from "./jose";

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
        
        const publicJWK = jwk.toPublic();
        if (!publicJWK) {
            throw new Error(`JWK is not a public key`);
        }

        return new JwkGenOutput(keyId, publicJWK);
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

    public async delete(keyId: string): Promise<void> {
        this._keys.delete(keyId);
    }

    public async exists(keyId: string): Promise<boolean> {
        return this._keys.has(keyId);
    }

    public count(): number {
        return this._keys.size;
    }
}

// Encodes a Ed25519 keypair into a Jwk.
function encodeJwk(keyPair: KeyPair, alg: JwsAlgorithm): Jwk {
    let x = encodeB64(keyPair.public());
    let d = encodeB64(keyPair.private());

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
            let textEncoder = new TextEncoder();
            const secret = decodeB64(textEncoder.encode(d));
            const pub = decodeB64(textEncoder.encode(paramsOkp.x));
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
    return encodeB64(RandomHelper.generate(32));
}
