import {
    CekAlgorithm,
    CoreDID,
    DIDType,
    EncryptedData,
    EncryptionAlgorithm,
    KeyLocation,
    KeyType,
    Signature,
    Storage,
} from "@iota/identity-wasm/node";
import {
    NapiCekAlgorithm,
    NapiCoreDid,
    NapiDidLocation,
    NapiDIDType,
    NapiEncryptedData,
    NapiEncryptionAlgorithm,
    NapiKeyLocation,
    NapiKeyType,
    NapiStronghold,
} from "../napi-dist/napi";

export class Stronghold implements Storage {
    private napiStronghold: NapiStronghold;

    constructor() {}

    public async init(snapshot: string, password: string, dropsave?: boolean) {
        this.napiStronghold = await NapiStronghold.new(snapshot, password, dropsave);
    }

    public static async build(snapshot: string, password: string, dropsave?: boolean) {
        const stronghold = new Stronghold();
        await stronghold.init(snapshot, password, dropsave);
        return stronghold;
    }

    public async didCreate(
        didType: DIDType,
        network: string,
        fragment: string,
        private_key?: Uint8Array,
    ): Promise<[CoreDID, KeyLocation]> {
        let optPrivateKey = undefined;
        if (private_key) {
            optPrivateKey = Array.from(private_key);
        }

        let napiDIDType: NapiDIDType | undefined = undefined;
        switch (didType) {
            case DIDType.IotaDID:
                napiDIDType = NapiDIDType.IotaDID;
                break;
            default:
                throw new Error("unexpected did type");
        }

        const napiDidLocation: NapiDidLocation = await this.napiStronghold.didCreate(
            napiDIDType,
            network,
            fragment,
            optPrivateKey,
        );

        const did: CoreDID = CoreDID.fromJSON(napiDidLocation.did().toJSON());
        const location: KeyLocation = KeyLocation.fromJSON(napiDidLocation.keyLocation().toJSON());

        return [did, location];
    }

    public async didPurge(did: CoreDID): Promise<boolean> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        return this.napiStronghold.didPurge(napiDID);
    }

    public async didExists(did: CoreDID): Promise<boolean> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        return this.napiStronghold.didExists(napiDID);
    }

    public async didList(): Promise<Array<CoreDID>> {
        const napiDids: Array<NapiCoreDid> = await this.napiStronghold.didList();
        const dids: Array<CoreDID> = napiDids.map((did) => CoreDID.fromJSON(did.toJSON()));
        return dids;
    }

    public async keyGenerate(did: CoreDID, keyType: KeyType, fragment: string): Promise<KeyLocation> {
        const napiDID = NapiCoreDid.fromJSON(did.toJSON());

        let napiKeyType: NapiKeyType | undefined = undefined;
        switch (keyType) {
            case KeyType.Ed25519:
                napiKeyType = NapiKeyType.Ed25519;
                break;
            case KeyType.X25519:
                napiKeyType = NapiKeyType.X25519;
                break;
            default:
                throw new Error("unexpected key type");
        }

        const napiKeyLocation = await this.napiStronghold.keyGenerate(napiDID, napiKeyType, fragment);
        return KeyLocation.fromJSON(napiKeyLocation.toJSON());
    }

    public async keyInsert(did: CoreDID, keyLocation: KeyLocation, privateKey: Uint8Array): Promise<void> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const napiKeyLocation: NapiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        await this.napiStronghold.keyInsert(napiDID, napiKeyLocation, Array.from(privateKey));
    }

    public async keyExists(did: CoreDID, keyLocation: KeyLocation): Promise<boolean> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const napiKeyLocation: NapiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyExists(napiDID, napiKeyLocation);
    }

    public async keyPublic(did: CoreDID, keyLocation: KeyLocation): Promise<Uint8Array> {
        const napiDID = NapiCoreDid.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        const publicKey = await this.napiStronghold.keyPublic(napiDID, napiKeyLocation);
        return Uint8Array.from(publicKey);
    }

    public async keyDelete(did: CoreDID, keyLocation: KeyLocation): Promise<boolean> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyDelete(napiDID, napiKeyLocation);
    }

    public async keySign(did: CoreDID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        const napiSignature = await this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data));
        return Signature.fromJSON(napiSignature.toJSON());
    }

    public async dataEncrypt(
        did: CoreDID,
        plaintext: Uint8Array,
        associatedData: Uint8Array,
        encryptionAlgorithm: EncryptionAlgorithm,
        cekAlgorithm: CekAlgorithm,
        publicKey: Uint8Array,
    ): Promise<EncryptedData> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const napiCekAlgorithm = NapiCekAlgorithm.fromJSON(cekAlgorithm.toJSON());
        const napiEncryptionAlgorithm = NapiEncryptionAlgorithm.fromJSON(encryptionAlgorithm.toJSON());
        const napiEncryptedData = await this.napiStronghold.dataEncrypt(
            napiDID,
            Array.from(plaintext),
            Array.from(associatedData),
            napiEncryptionAlgorithm,
            napiCekAlgorithm,
            Array.from(publicKey),
        );
        return EncryptedData.fromJSON(napiEncryptedData.toJSON());
    }

    public async dataDecrypt(
        did: CoreDID,
        data: EncryptedData,
        encryptionAlgorithm: EncryptionAlgorithm,
        cekAlgorithm: CekAlgorithm,
        privateKey: KeyLocation,
    ): Promise<Uint8Array> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const napiPrivateKeyLocation = NapiKeyLocation.fromJSON(privateKey.toJSON());
        const napiCekAlgorithm = NapiCekAlgorithm.fromJSON(cekAlgorithm.toJSON());
        const napiEncryptionAlgorithm = NapiEncryptionAlgorithm.fromJSON(encryptionAlgorithm.toJSON());
        const napiEncryptedData = NapiEncryptedData.fromJSON(data.toJSON());
        const decryptedData = await this.napiStronghold.dataDecrypt(
            napiDID,
            napiEncryptedData,
            napiEncryptionAlgorithm,
            napiCekAlgorithm,
            napiPrivateKeyLocation,
        );
        return Uint8Array.from(decryptedData);
    }

    public async blobGet(did: CoreDID): Promise<Uint8Array | undefined> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        const value: number[] | undefined = await this.napiStronghold.blobGet(napiDID);

        if (value) {
            return Uint8Array.from(value);
        } else {
            return undefined;
        }
    }

    public async blobSet(did: CoreDID, value: Uint8Array): Promise<void> {
        const napiDID: NapiCoreDid = NapiCoreDid.fromJSON(did.toJSON());
        return this.napiStronghold.blobSet(napiDID, Array.from(value));
    }

    public async flushChanges() {
        return this.napiStronghold.flushChanges();
    }
}
