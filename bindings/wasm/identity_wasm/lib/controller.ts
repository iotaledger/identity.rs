import { DelegationToken } from "~identity_wasm";

/**
 * Permissions of a {@link DelegationToken}.
 */
export enum DelegatePermissions {
    None = 0,
    CreateProposal = 1,
    ApproveProposal = 1 << 1,
    ExecuteProposal = 1 << 2,
    DeleteProposal = 1 << 3,
    RemoveApproval = 1 << 4,
    All = 0xffffffff,
}
