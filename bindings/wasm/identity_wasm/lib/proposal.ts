import { TransactionInternal } from "./transaction_internal";

export interface Proposal<T> {
  id: string,
  get action(): T,
  votes: bigint,
  voters: Set<string>,
  expirationEpoch?: bigint,
  approve: (client: unknown) => TransactionInternal<void>,
  intoTx: (client: unknown) => TransactionInternal<Proposal<T> | undefined>,
}