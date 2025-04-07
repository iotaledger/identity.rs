/*
 * Modifications Copyright 2024 Fondazione LINKS.
 */
import * as ed from "@noble/ed25519";
import { decodeB64, encodeB64, Jwk, JwkGenOutput, JwkStorage, ProofAlgorithm, ProofUpdateCtx, JwkStoragePQ } from "~identity_wasm";
import { EdCurve, JwkType, JwsAlgorithm } from "./jose";
import { ml_dsa44, ml_dsa65, ml_dsa87 } from '@noble/post-quantum/ml-dsa';

type Ed25519PrivateKey = Uint8Array;
type Ed25519PublicKey = Uint8Array;

export class JwkMemStore implements JwkStorage,  JwkStoragePQ{
    /** The map from key identifiers to Jwks. */
    private _keys: Map<string, Jwk>;

    /** Creates a new, empty `MemStore` instance. */
    constructor() {
        this._keys = new Map();
    }

    public async generatePQKey(keyType: String, algorithm: JwsAlgorithm):  Promise<JwkGenOutput> {
        if (keyType !== JwkMemStore.mldsaKeyType()) {
            throw new Error(`unsupported key type ${keyType}`);
        }

        const seed = new TextEncoder().encode(randomKeyId())
        let keys;
        if (algorithm === JwsAlgorithm.MLDSA44) {
            keys = ml_dsa44.keygen(seed);
        } else if(algorithm === JwsAlgorithm.MLDSA65) {
            keys = ml_dsa65.keygen(seed);
        } else if(algorithm === JwsAlgorithm.MLDSA87) {
            keys = ml_dsa87.keygen(seed);
        } else {
            throw new Error(`unsupported algorithm`);
        }

        const keyId = randomKeyId();
        const jwk = await encodeJwk(keys.secretKey, keys.publicKey, algorithm);

        if(jwk == undefined)
            throw new Error("Unexpected error: await encodeJwk(privKey, publicKey, algorithm)");
        
        this._keys.set(keyId, jwk);

        const publicJWK = jwk?.toPublic();
        if (!publicJWK) {
            throw new Error(`JWK is not a public key`);
        }
        return new JwkGenOutput(keyId, publicJWK);

    }

    public static ed25519KeyType(): string {
        return "Ed25519";
    }

    public static mldsaKeyType(): string {
        return "ML-DSA";
    }

    private _get_key(keyId: string): Jwk | undefined {
        return this._keys.get(keyId);
    }

    public async generate(keyType: string, algorithm: JwsAlgorithm): Promise<JwkGenOutput> {
        if (keyType !== JwkMemStore.ed25519KeyType()) {
            throw new Error(`unsupported key type ${keyType}`);
        }

        if (algorithm !== JwsAlgorithm.EdDSA) {
            throw new Error(`unsupported algorithm`);
        }

        const keyId = randomKeyId();
        const privKey: Ed25519PrivateKey = ed.utils.randomPrivateKey();
        const publicKey = await ed.getPublicKey(privKey);

        const jwk = await encodeJwk(privKey, publicKey, algorithm);
        if(jwk == undefined)
            throw new Error("Unexpected error: await encodeJwk(privKey, publicKey, algorithm)");

        this._keys.set(keyId, jwk);

        const publicJWK = jwk.toPublic();
        if (!publicJWK) {
            throw new Error(`JWK is not a public key`);
        }

        return new JwkGenOutput(keyId, publicJWK);
    }

    public async sign(keyId: string, data: Uint8Array, publicKey: Jwk): Promise<Uint8Array> {
        if (publicKey.alg()! !== JwsAlgorithm.EdDSA) {
            throw new Error("unsupported JWS algorithm");
        } else {
            if (publicKey.paramsOkp()?.crv !== (EdCurve.Ed25519 as string)) {
                throw new Error("unsupported Okp parameter");
            }
        }

        const jwk = this._keys.get(keyId);

        if (jwk) {
            const [privateKey, _] = decodeJwk(jwk);
            return ed.sign(data, privateKey);
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
async function encodeJwk(
    privateKey: Uint8Array,
    publicKey: Uint8Array,
    alg: JwsAlgorithm
): Promise<Jwk | undefined> {
    const x = encodeB64(publicKey);
    const d = encodeB64(privateKey);

    if (alg === JwsAlgorithm.EdDSA) {
        return new Jwk({
            kty: JwkType.Okp,
            crv: "Ed25519",
            d,
            x,
            alg,
        });
    } else if (alg === JwsAlgorithm.MLDSA44 || alg === JwsAlgorithm.MLDSA65 || alg === JwsAlgorithm.MLDSA87) {
        return new Jwk({
            "kty": JwkType.Akp,
            pub: x,
            priv: d,
            alg,
        });
    }

    return undefined;
}

function decodeJwk(jwk: Jwk): [Uint8Array, Uint8Array] {
    if (jwk.alg()! !== JwsAlgorithm.EdDSA) {
        throw new Error("unsupported `alg`");
    }

    if (jwk.alg()! === JwsAlgorithm.EdDSA) {
        const paramsOkp = jwk.paramsOkp();
        if (paramsOkp) {
            const d = paramsOkp.d;

            if (d) {
                const textEncoder = new TextEncoder();
                const privateKey = decodeB64(textEncoder.encode(d));
                const publicKey = decodeB64(textEncoder.encode(paramsOkp.x));
                return [privateKey, publicKey];
            } else {
                throw new Error("missing private key component");
            }
        } else {
            throw new Error("expected Okp params");
        }
    } else if (jwk.alg()! === JwsAlgorithm.MLDSA44 || jwk.alg()! === JwsAlgorithm.MLDSA65 || jwk.alg()! === JwsAlgorithm.MLDSA87) {
        const paramsPQ = jwk.paramsAkp();
        if (paramsPQ) {
            const priv = paramsPQ.priv;

            if (priv) {
                const textEncoder = new TextEncoder();
                const privateKey = decodeB64(textEncoder.encode(priv));
                const publicKey = decodeB64(textEncoder.encode(paramsPQ.pub));
                return [privateKey, publicKey];
            } else {
                throw new Error("missing private key component");
            }
        } else {
            throw new Error("expected Okp params");
        }
    } else {
        throw new Error("unsupported `alg`");
    }
}

//TODO: non sembra servire a nulla
export interface JwkStorageBBSPlusExt {
    // Generate a new BLS12381 key represented as a JSON Web Key.
    generateBBS: (algorithm: ProofAlgorithm) => Promise<JwkGenOutput>;
    /** Signs a chunk of data together with an optional header
     * using the private key corresponding to the given `keyId` and according
     * to `publicKey`'s requirements.
     */
    signBBS: (keyId: string, data: Uint8Array[], publicKey: Jwk, header?: Uint8Array) => Promise<Uint8Array>;
    // Updates the timeframe validity period information of a given signature.
    updateBBSSignature: (
        keyId: string,
        publicKey: Jwk,
        signature: Uint8Array,
        proofCtx: ProofUpdateCtx,
    ) => Promise<Uint8Array>;
}

// Returns a random number between `min` and `max` (inclusive).
// SAFETY NOTE: This is not cryptographically secure randomness and thus not suitable for production use.
// It suffices for our testing implementation however and avoids an external dependency.
function getRandomNumber(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

// Returns a random key id.
function randomKeyId(): string {
    const randomness = new Uint8Array(20);
    for (let index = 0; index < randomness.length; index++) {
        randomness[index] = getRandomNumber(0, 255);
    }

    return encodeB64(randomness);
}