import { IdentityClient } from "~identity_wasm";
import { IotaTransactionBlockResponse } from "@iota/iota-sdk/client";

export interface TransactionInternalOutput<T> {
  response: IotaTransactionBlockResponse,
  output: T,
}

export interface TransactionInternal<T> {
  set gasBudget(value: bigint),
  withGasBudget(gasBudget: bigint): TransactionInternal<T>,
  execute(client: IdentityClient): Promise<TransactionInternalOutput<T>>,
}