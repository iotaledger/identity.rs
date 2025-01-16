import { Transaction, TransactionArgument, TransactionResult } from "@iota/iota-sdk/transactions";
import { IOTA_CLOCK_OBJECT_ID } from "@iota/iota-sdk/utils";

export function getClockRef(tx: Transaction): TransactionArgument {
  return tx.sharedObjectRef({ objectId: IOTA_CLOCK_OBJECT_ID, initialSharedVersion: 1, mutable: false });
}

export function getControllerDelegation(tx: Transaction, controllerCap: TransactionArgument, packageId: string): [TransactionArgument, TransactionArgument] {
  const [token, borrow] = tx.moveCall({
    target: `${packageId}::controller::borrow`,
    arguments: [controllerCap],
  });
  return [token, borrow];
}

export function putBackDelegationToken(tx: Transaction, controllerCap: TransactionArgument, delegationToken: TransactionArgument, borrow: TransactionArgument, packageId: string) {
  tx.moveCall({
    target: `${packageId}::controller::put_back`,
    arguments: [controllerCap, delegationToken, borrow],
  });
}