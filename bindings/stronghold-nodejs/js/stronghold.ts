import { NapiStronghold, NapiDid, NapiKeyLocation, NapiChainState, NapiDocument, NapiKeyType, NapiDidLocation, NapiEncryptedData, NapiEncryptionAlgorithm, NapiCekAlgorithm } from '../napi-dist/napi';
import { DID, KeyLocation, Signature, ChainState, Storage, KeyType, Document, EncryptedData, EncryptionAlgorithm, CekAlgorithm } from "@iota/identity-wasm/node";

export class Stronghold implements Storage {
    private napiStronghold: NapiStronghold;

    constructor() { }

    public async init(snapshot: string, password: string, dropsave?: boolean) {
        this.napiStronghold = await NapiStronghold.new(snapshot, password, dropsave);
    }

    public static async build(snapshot: string, password: string, dropsave?: boolean) {
        const stronghold = new Stronghold();
        await stronghold.init(snapshot, password, dropsave)
        return stronghold
    }

    public async didCreate(network: string, fragment: string, private_key?: Uint8Array): Promise<[DID, KeyLocation]> {
        let optPrivateKey = undefined;
        if (private_key) {
            optPrivateKey = Array.from(private_key)
        }

        const napiDidLocation: NapiDidLocation = await this.napiStronghold.didCreate(network, fragment, optPrivateKey);

        const did: DID = DID.fromJSON(napiDidLocation.did().toJSON());
        const location: KeyLocation = KeyLocation.fromJSON(napiDidLocation.keyLocation().toJSON());

        return [did, location];
    }

    public async didPurge(did: DID): Promise<boolean> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        return this.napiStronghold.didPurge(napiDID);
    }

    public async didExists(did: DID): Promise<boolean> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        return this.napiStronghold.didExists(napiDID);
    }

    public async didList(): Promise<Array<DID>> {
        const napiDids: Array<NapiDid> = await this.napiStronghold.didList();
        const dids: Array<DID> = napiDids.map((did) => DID.fromJSON(did.toJSON()))
        return dids;
    }

    public async keyGenerate(did: DID, keyType: KeyType, fragment: string): Promise<KeyLocation> {
        const napiDID = NapiDid.fromJSON(did.toJSON());

        let napiKeyType: NapiKeyType | undefined = undefined;
        switch (keyType) {
            case KeyType.Ed25519:
                napiKeyType = NapiKeyType.Ed25519
                break;
            case KeyType.X25519:
                napiKeyType = NapiKeyType.X25519
                break;
            default:
                throw new Error("unexpected key type");
        }

        const napiKeyLocation = await this.napiStronghold.keyGenerate(napiDID, napiKeyType, fragment);
        return KeyLocation.fromJSON(napiKeyLocation.toJSON());
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: Uint8Array): Promise<void> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiKeyLocation: NapiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        await this.napiStronghold.keyInsert(napiDID, napiKeyLocation, Array.from(privateKey));
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiKeyLocation: NapiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyExists(napiDID, napiKeyLocation)
    }

    public async keyPublic(did: DID, keyLocation: KeyLocation): Promise<Uint8Array> {
        const napiDID = NapiDid.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        const publicKey = await this.napiStronghold.keyPublic(napiDID, napiKeyLocation);
        return Uint8Array.from(publicKey);
    }

    public async keyDelete(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyDelete(napiDID, napiKeyLocation);
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        const napiSignature = await this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data));
        return Signature.fromJSON(napiSignature.toJSON());
    }

    public async dataEncrypt(did: DID, plaintext: Uint8Array, associatedData: Uint8Array, encryptionAlgorithm: EncryptionAlgorithm, cekAlgorithm: CekAlgorithm, publicKey: Uint8Array): Promise<EncryptedData> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiCekAlgorithm = NapiCekAlgorithm.fromJSON(cekAlgorithm.toJSON());
        const napiEncryptionAlgorithm = NapiEncryptionAlgorithm.fromJSON(encryptionAlgorithm.toJSON());
        const napiEncryptedData = await this.napiStronghold.dataEncrypt(napiDID, Array.from(plaintext), Array.from(associatedData),  napiEncryptionAlgorithm, napiCekAlgorithm, Array.from(publicKey));
        return EncryptedData.fromJSON(napiEncryptedData.toJSON());
    }

    public async dataDecrypt(did: DID, data: EncryptedData, encryptionAlgorithm: EncryptionAlgorithm, cekAlgorithm: CekAlgorithm, privateKey: KeyLocation): Promise<Uint8Array> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiPrivateKeyLocation = NapiKeyLocation.fromJSON(privateKey.toJSON());
        const napiCekAlgorithm = NapiCekAlgorithm.fromJSON(cekAlgorithm.toJSON());
        const napiEncryptionAlgorithm = NapiEncryptionAlgorithm.fromJSON(encryptionAlgorithm.toJSON());
        const napiEncryptedData = NapiEncryptedData.fromJSON(data.toJSON());
        const decryptedData = await this.napiStronghold.dataDecrypt(napiDID, napiEncryptedData, napiEncryptionAlgorithm, napiCekAlgorithm, napiPrivateKeyLocation);
        return Uint8Array.from(decryptedData);
    }

    public async chainStateGet(did: DID): Promise<ChainState | undefined> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiChainState: NapiChainState | undefined = await this.napiStronghold.chainStateGet(napiDID);

        if (napiChainState) {
            return ChainState.fromJSON(napiChainState.toJSON())
        } else {
            return undefined;
        }
    }

    public async chainStateSet(did: DID, chainState: ChainState): Promise<void> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiChainState: NapiChainState = NapiChainState.fromJSON(chainState.toJSON());
        return this.napiStronghold.chainStateSet(napiDID, napiChainState);
    }

    public async documentGet(did: DID): Promise<Document | undefined> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiDocument: NapiDocument | undefined = await this.napiStronghold.documentGet(napiDID);

        if (napiDocument) {
            return Document.fromJSON(napiDocument.toJSON())
        } else {
            return undefined;
        }
    }

    public async documentSet(did: DID, document: Document): Promise<void> {
        const napiDID: NapiDid = NapiDid.fromJSON(did.toJSON());
        const napiDocument: NapiDocument = NapiDocument.fromJSON(document.toJSON());
        return this.napiStronghold.documentSet(napiDID, napiDocument);
    }

    public async flushChanges() {
        return this.napiStronghold.flushChanges()
    }
}
