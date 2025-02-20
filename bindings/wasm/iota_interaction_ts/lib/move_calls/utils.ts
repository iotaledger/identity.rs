import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";
import { IOTA_CLOCK_OBJECT_ID } from "@iota/iota-sdk/utils";

const PLACEHOLDER_SENDER = '0x00000000000000090807060504030201';
const PLACEHOLDER_GAS_BUDGET = 9;
const PLACEHOLDER_GAS_PRICE = 8;
const PLACEHOLDER_GAS_PAYMENT: ObjectRef[] = [];

export function getClockRef(tx: Transaction): TransactionArgument {
    return tx.sharedObjectRef({ objectId: IOTA_CLOCK_OBJECT_ID, initialSharedVersion: 1, mutable: false });
}

export function getControllerDelegation(
    tx: Transaction,
    controllerCap: TransactionArgument,
    packageId: string,
): [TransactionArgument, TransactionArgument] {
    const [token, borrow] = tx.moveCall({
        target: `${packageId}::controller::borrow`,
        arguments: [controllerCap],
    });
    return [token, borrow];
}

export function putBackDelegationToken(
    tx: Transaction,
    controllerCap: TransactionArgument,
    delegationToken: TransactionArgument,
    borrow: TransactionArgument,
    packageId: string,
) {
    tx.moveCall({
        target: `${packageId}::controller::put_back`,
        arguments: [controllerCap, delegationToken, borrow],
    });
}

/**
 * Inserts placeholders related to sender and payment into transaction.
 * 
 * This is required if wanting to call `tx.build`, as this will check if these values have been set.
 *
 * @param tx transaction to update
 */
export function insertPlaceholders(tx: Transaction) {
    tx.setGasPrice(PLACEHOLDER_GAS_PRICE);
    tx.setGasBudget(PLACEHOLDER_GAS_BUDGET);
    tx.setGasPayment([...PLACEHOLDER_GAS_PAYMENT]); // make sure, we're not sharing the array between tx
    tx.setSender(PLACEHOLDER_SENDER);
}