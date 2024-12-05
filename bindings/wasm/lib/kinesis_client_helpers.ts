// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IotaObjectResponse,
    IotaTransactionBlockResponse,
    ExecutionStatus,
    OwnedObjectRef,
} from "@iota/iota.js/client";

export class IotaTransactionBlockResponseAdapter {
    response: IotaTransactionBlockResponse;

    constructor(response: IotaTransactionBlockResponse) {
        this.response = response;
    }

    effects_is_none(): boolean {
        return this.response.effects == null;
    }

    effects_is_some(): boolean {
        return !(typeof this.response.effects == null);
    }

    to_string(): string {
        return JSON.stringify(this.response);
    }

    effects_execution_status_inner(): null | ExecutionStatus {
        return this.response.effects != null ? this.response.effects.status : null;
    }

    effects_created_inner(): null | OwnedObjectRef[] {
        return this.response.effects != null && this.response.effects.created != null ? this.response.effects.created : null;
    }
}
