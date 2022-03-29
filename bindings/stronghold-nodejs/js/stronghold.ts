import { NapiStronghold, NapiDID, NapiKeyLocation, NapiChainState, NapiIdentityState } from '../napi-dist/napi';
import { DID, KeyLocation, Signature, ChainState, IdentityState, Storage } from "@iota/identity-wasm/node";

export class Stronghold implements Storage {
    private napiStronghold: NapiStronghold;

    constructor() {}

    public async init(snapshot: string, password: string, dropsave?: boolean) {
        this.napiStronghold = await NapiStronghold.new(snapshot, password, dropsave);
    }

    public static async build(snapshot: string, password: string, dropsave?: boolean) {
        const stronghold = new Stronghold();
        await stronghold.init(snapshot, password, dropsave)
        return stronghold
    }

    public async flushChanges() {
        return this.napiStronghold.flushChanges()
    }

    public async keyNew(did: DID, keyLocation: KeyLocation) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        let publicKey = await this.napiStronghold.keyNew(napiDID, napiKeyLocation);
        return Uint8Array.from(publicKey)
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: Uint8Array) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        let publicKey = await this.napiStronghold.keyInsert(napiDID, napiKeyLocation, Array.from(privateKey));
        return Uint8Array.from(publicKey)
    }

    public async keyExists(did: DID, keyLocation: KeyLocation) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyExists(napiDID, napiKeyLocation)
    }

    public async keyGet(did: DID, keyLocation: KeyLocation) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        let publicKey = await this.napiStronghold.keyGet(napiDID, napiKeyLocation);
        return Uint8Array.from(publicKey);
    }

    public async keyDel(did: DID, keyLocation: KeyLocation) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyDel(napiDID, napiKeyLocation)
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        let napiSignature = await this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data));
        return Signature.fromJSON(napiSignature.toJSON())
    }

    public async chainState(did: DID) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiChainState = await this.napiStronghold.chainState(napiDID);
        return ChainState.fromJSON(napiChainState.toJSON())
    }

    public async setChainState(did: DID, chainState: ChainState) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiChainState = NapiChainState.fromJSON(chainState.toJSON());
        return this.napiStronghold.setChainState(napiDID, napiChainState);
    }

    public async state(did: DID) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiIdentityState = await this.napiStronghold.state(napiDID);
        return IdentityState.fromJSON(napiIdentityState.toJSON())
    }

    public async setState(did: DID, identityState: IdentityState) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiIdentityState = NapiIdentityState.fromJSON(identityState.toJSON());
        return this.napiStronghold.setState(napiDID, napiIdentityState);
    }

    public async purge(did: DID) {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        return this.napiStronghold.purge(napiDID);
    }
}
