import { IotaTransactionBlockResponse, TransactionEffects, ExecutionStatus, OwnedObjectRef} from "@iota/iota.js/types";

export class IotaTransactionBlockResponseAdapter {
    response: IotaTransactionBlockResponse;

    constructor(response: IotaTransactionBlockResponse) {
        this.response = response;
    }

    effects_is_none(): boolean {
        return this.response.effects === null;
    }

    effects_is_some(): boolean {
        return typeof this.response.effects === TransactionEffects;
    }

    to_string(): string {
        return JSON.stringify(this.response);
    }

    effects_execution_status_inner(): null | ExecutionStatus {
        return this.response.effects?.status;
    }

    effects_created_inner(): null | OwnedObjectRef[] {
        return this.response.effects?.created;
    }
}
