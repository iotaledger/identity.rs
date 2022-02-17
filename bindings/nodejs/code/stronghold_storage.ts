import { NapiStronghold, NapiDID, NapiKeyLocation, NapiChainState, NapiIdentityState } from '../index.js'
import { DID, KeyLocation, DIDLease, Signature, ChainState, IdentityState, Storage } from "../../wasm/node/identity_wasm.js";

export class Stronghold implements Storage {
    private napiStronghold: NapiStronghold;

    constructor(snapshot: string, password: string, dropsave: boolean) {
        this.napiStronghold = NapiStronghold.create(snapshot, password, dropsave);
    }

    public async setPassword(encryptionKey: Uint8Array): Promise<void> {
        return this.napiStronghold.setPassword(Array.from(encryptionKey))
    }

    public async flushChanges(): Promise<void> {
        return this.napiStronghold.flushChanges()
    }

    public async leaseDid(did: DID): Promise<DIDLease> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        // TODO how to get a DIDLease from a NapiDIDLease
        let napiDIDLease = await this.napiStronghold.leaseDid(napiDID);
        let didLease = new DIDLease();
        didLease.store(napiDIDLease.load())
        return didLease
    }

    public async keyNew(did: DID, keyLocation: KeyLocation): Promise<string> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyNew(napiDID, napiKeyLocation)
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: string): Promise<string> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyInsert(napiDID, napiKeyLocation, privateKey)
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyExists(napiDID, napiKeyLocation)
    }

    public async keyGet(did: DID, keyLocation: KeyLocation): Promise<string> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyGet(napiDID, napiKeyLocation)
    }

    public async keyDel(did: DID, keyLocation: KeyLocation): Promise<void> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        return this.napiStronghold.keyDel(napiDID, napiKeyLocation)
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiKeyLocation = NapiKeyLocation.fromJSON(keyLocation.toJSON());
        let napiSignature = await this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data));
        return Signature.fromJSON(napiSignature.toJSON())
    }

    public async chainState(did: DID): Promise<ChainState> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiChainState = await this.napiStronghold.chainState(napiDID);
        return ChainState.fromJSON(napiChainState.toJSON())
    }

    public async setChainState(did: DID, chainState: ChainState): Promise<void> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiChainState = NapiChainState.fromJSON(chainState.toJSON());
        return this.napiStronghold.setChainState(napiDID, napiChainState);
    }

    public async state(did: DID): Promise<IdentityState> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiIdentityState = await this.napiStronghold.state(napiDID);
        return IdentityState.fromJSON(napiIdentityState.toJSON())
    }

    public async setState(did: DID, identityState: IdentityState): Promise<void> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        let napiIdentityState = NapiIdentityState.fromJSON(identityState.toJSON());
        return this.napiStronghold.setState(napiDID, napiIdentityState);
    }

    public async purge(did: DID): Promise<void> {
        let napiDID = NapiDID.fromJSON(did.toJSON());
        return this.napiStronghold.purge(napiDID);
    }
}