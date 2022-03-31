import { NapiStronghold, NapiDID, NapiKeyLocation, NapiChainState, NapiDocument, NapiKeyType, NapiDidLocation } from '../napi-dist/napi';
import { DID, KeyLocation, Signature, ChainState, Storage, KeyType, Document } from "@iota/identity-wasm/node";

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
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        return this.napiStronghold.didPurge(napiDID);
    }

    public async didExists(did: DID): Promise<boolean> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        return this.napiStronghold.didExists(napiDID);
    }

    public async didList(): Promise<Array<DID>> {
        const napiDids: Array<NapiDID> = await this.napiStronghold.didList();
        const dids: Array<DID> = napiDids.map((did) => DID.fromJSON(did.toJSON()))
        return dids;
    }

    public async keyGenerate(did: DID, keyType: KeyType, fragment: string): Promise<KeyLocation> {
        const napiDID = NapiDID.fromJSON(did.toJSON());

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
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiKeyLocation: NapiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        await this.napiStronghold.keyInsert(napiDID, napiKeyLocation, Array.from(privateKey));
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiKeyLocation: NapiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyExists(napiDID, napiKeyLocation)
    }

    public async keyPublic(did: DID, keyLocation: KeyLocation): Promise<Uint8Array> {
        const napiDID = NapiDID.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        const publicKey = await this.napiStronghold.keyPublic(napiDID, napiKeyLocation);
        return Uint8Array.from(publicKey);
    }

    public async keyDelete(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyDelete(napiDID, napiKeyLocation);
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        const napiSignature = await this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data));
        return Signature.fromJSON(napiSignature.toJSON());
    }

    public async chainStateGet(did: DID): Promise<ChainState | undefined | null> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiChainState: NapiChainState = await this.napiStronghold.chainStateGet(napiDID);
        return ChainState.fromJSON(napiChainState.toJSON())
    }

    public async chainStateSet(did: DID, chainState: ChainState): Promise<void> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiChainState: NapiChainState = NapiChainState.fromJSON(chainState.toJSON());
        return this.napiStronghold.chainStateSet(napiDID, napiChainState);
    }

    public async documentGet(did: DID): Promise<Document | undefined | null> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiDocument: NapiDocument = await this.napiStronghold.documentGet(napiDID);
        return Document.fromJSON(napiDocument.toJSON())
    }

    public async documentSet(did: DID, document: Document): Promise<void> {
        const napiDID: NapiDID = NapiDID.fromJSON(did.toJSON());
        const napiDocument: NapiDocument = NapiDocument.fromJSON(document.toJSON());
        return this.napiStronghold.documentSet(napiDID, napiDocument);
    }

    public async flushChanges() {
        return this.napiStronghold.flushChanges()
    }
}
