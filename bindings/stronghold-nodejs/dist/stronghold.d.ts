import { DID, KeyLocation, Signature, ChainState, IdentityState, Storage } from "@iota/identity-wasm/node";
export declare class Stronghold implements Storage {
    private napiStronghold;
    constructor();
    init(snapshot: string, password: string, dropsave?: boolean): Promise<void>;
    static build(snapshot: string, password: string, dropsave?: boolean): Promise<Stronghold>;
    setPassword(encryptionKey: Uint8Array): Promise<void>;
    flushChanges(): Promise<void>;
    keyNew(did: DID, keyLocation: KeyLocation): Promise<string>;
    keyInsert(did: DID, keyLocation: KeyLocation, privateKey: string): Promise<string>;
    keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean>;
    keyGet(did: DID, keyLocation: KeyLocation): Promise<string>;
    keyDel(did: DID, keyLocation: KeyLocation): Promise<void>;
    keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature>;
    chainState(did: DID): Promise<ChainState>;
    setChainState(did: DID, chainState: ChainState): Promise<void>;
    state(did: DID): Promise<IdentityState>;
    setState(did: DID, identityState: IdentityState): Promise<void>;
    purge(did: DID): Promise<void>;
}
//# sourceMappingURL=stronghold.d.ts.map