import { IotaClient } from "@iota/iota-sdk/client";
import { PublicKey } from "@iota/iota-sdk/cryptography";
import { TransactionSigner } from "~identity_wasm";

export interface CoreClientReadOnly {
    packageId(): string;
    network(): string;
    iotaClient(): IotaClient;
    // TODO: add all interface's methods.
}

export interface CoreClient<S extends TransactionSigner> extends CoreClientReadOnly {
    signer(): S;
    senderAddress(): string;
    senderPublicKey(): PublicKey;
}
