import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";
import { IOTA_CLOCK_OBJECT_ID } from "@iota/iota-sdk/utils";
import { ControllerTokenRef } from "./identity";

const PLACEHOLDER_SENDER = "0x00000000000000090807060504030201";
const PLACEHOLDER_GAS_BUDGET = 9;
const PLACEHOLDER_GAS_PRICE = 8;
const PLACEHOLDER_GAS_PAYMENT: ObjectRef[] = [];

export function getClockRef(tx: Transaction): TransactionArgument {
    return tx.sharedObjectRef({ objectId: IOTA_CLOCK_OBJECT_ID, initialSharedVersion: 1, mutable: false });
}

export interface ControllerTokenArg {
    cap?: TransactionArgument;
    token: TransactionArgument;
    borrow?: TransactionArgument;
}

export function controllerTokenRefToTxArgument(
    tx: Transaction,
    controllerToken: ControllerTokenRef,
    packageId: string,
): ControllerTokenArg {
    const token = tx.objectRef(controllerToken.objectRef);
    if (controllerToken.type == "DelegationToken") {
        return { token };
    } else {
        const [delegation_token, borrow] = tx.moveCall({
            target: `${packageId}::controller::borrow`,
            arguments: [token],
        });
        return { cap: token, token: delegation_token, borrow };
    }
}

export function putBackControllerToken(
    tx: Transaction,
    controllerToken: ControllerTokenArg,
    packageId: string,
) {
    if (controllerToken.cap && controllerToken.borrow) {
        tx.moveCall({
            target: `${packageId}::controller::put_back`,
            arguments: [controllerToken.cap, controllerToken.token, controllerToken.borrow],
        });
    }
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
