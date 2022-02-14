import { Buffer } from 'buffer';
import { NapiStronghold, NapiDID, NapiKeyLocation, NapiChainState, NapiIdentityState, NapiGeneration } from '../index.js'
import { DID, KeyLocation, Generation, DIDLease, Signature, ChainState, IdentityState, Storage } from "../../../wasm/node/identity_wasm.js";

class Stronghold implements Storage {
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
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        // TODO how to get a DIDLease from a NapiDIDLease
        let napiDIDLease = await this.napiStronghold.leaseDid(napiDID);
        let didLease = new DIDLease();
        didLease.store(napiDIDLease.load())
        return didLease
    }

    public async keyNew(did: DID, keyLocation: KeyLocation): Promise<string> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiKeyLocation = NapiKeyLocation.fromBuffer(Buffer.from(keyLocation.asBytes()));
        return this.napiStronghold.keyNew(napiDID, napiKeyLocation)
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: string): Promise<string> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiKeyLocation = NapiKeyLocation.fromBuffer(Buffer.from(keyLocation.asBytes()));
        return this.napiStronghold.keyInsert(napiDID, napiKeyLocation, privateKey)
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiKeyLocation = NapiKeyLocation.fromBuffer(Buffer.from(keyLocation.asBytes()));
        return this.napiStronghold.keyExists(napiDID, napiKeyLocation)
    }

    public async keyGet(did: DID, keyLocation: KeyLocation): Promise<string> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiKeyLocation = NapiKeyLocation.fromBuffer(Buffer.from(keyLocation.asBytes()));
        return this.napiStronghold.keyGet(napiDID, napiKeyLocation)
    }

    public async keyDel(did: DID, keyLocation: KeyLocation): Promise<void> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiKeyLocation = NapiKeyLocation.fromBuffer(Buffer.from(keyLocation.asBytes()));
        return this.napiStronghold.keyDel(napiDID, napiKeyLocation)
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiKeyLocation = NapiKeyLocation.fromBuffer(Buffer.from(keyLocation.asBytes()));
        let napiSignature = await this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data));
        return Signature.fromBytes(Uint8Array.from(napiSignature.asBytes()))
    }

    public async chainState(did: DID): Promise<ChainState> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiChainState = await this.napiStronghold.chainState(napiDID);
        return ChainState.fromBytes(Uint8Array.from(napiChainState.asBytes()))
    }

    public async setChainState(did: DID, chainState: ChainState): Promise<void> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiChainState = NapiChainState.fromBuffer(Buffer.from(chainState.asBytes()));
        return this.napiStronghold.setChainState(napiDID, napiChainState);
    }

    public async state(did: DID): Promise<IdentityState> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiIdentityState = await this.napiStronghold.state(napiDID);
        return IdentityState.fromBytes(Uint8Array.from(napiIdentityState.asBytes()))
    }

    public async setState(did: DID, identityState: IdentityState): Promise<void> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiIdentityState = NapiIdentityState.fromBuffer(Buffer.from(identityState.asBytes()));;
        return this.napiStronghold.setState(napiDID, napiIdentityState);
    }

    public async purge(did: DID): Promise<void> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        return this.napiStronghold.purge(napiDID);
    }

    public async publishedGeneration(did: DID): Promise<Generation> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiGeneration = await this.napiStronghold.publishedGeneration(napiDID);
        return Generation.fromBytes(Uint8Array.from(napiGeneration.asBytes()))
    }

    public async setPublishedGeneration(did: DID, generation: Generation): Promise<void> {
        let napiDID = NapiDID.fromBuffer(Buffer.from(did.asBytes()));
        let napiGeneration = NapiGeneration.fromBuffer(Buffer.from(generation.asBytes()));
        return this.napiStronghold.setPublishedGeneration(napiDID, napiGeneration);
    }
}