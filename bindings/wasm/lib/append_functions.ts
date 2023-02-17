import { CoreDocument, IotaDocument, IToCoreDocument } from "~identity_wasm";
type GetCoreDocument = (arg: IToCoreDocument) => CoreDocument;
type MaybeGetIotaDocument = (arg: IToCoreDocument) => IotaDocument | void;

declare global {
    var _getCoreDocumentInternal: GetCoreDocument;
    var _maybeGetIotaDocumentInternal: MaybeGetIotaDocument;
}
function _getCoreDocumentInternal(arg: IToCoreDocument): CoreDocument {
    if (arg instanceof CoreDocument) {
        return arg._shallowCloneInternal();
    } else {
        return arg.toCoreDocument()._shallowCloneInternal();
    }
}

function _maybeGetIotaDocumentInternal(arg: IToCoreDocument): IotaDocument | void {
    if (arg instanceof IotaDocument) {
        return arg._shallowCloneInternal();
    } else {
        return;
    }
}

globalThis._getCoreDocumentInternal = _getCoreDocumentInternal;

globalThis._maybeGetIotaDocumentInternal = _maybeGetIotaDocumentInternal;
