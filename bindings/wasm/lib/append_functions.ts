import { CoreDocument, IAsCoreDocument, IotaDocument } from "~identity_wasm";
type GetCoreDocument = (arg: IAsCoreDocument) => CoreDocument;
type MaybeGetIotaDocument = (arg: IAsCoreDocument) => IotaDocument | void;

declare global {
    var _getCoreDocumentInternal: GetCoreDocument;
    var _maybeGetIotaDocumentInternal: MaybeGetIotaDocument;
}
function _getCoreDocumentInternal(arg: IAsCoreDocument): CoreDocument {
    if (arg instanceof CoreDocument) {
        return arg._shallowCloneInternal();
    } else {
        return arg.asCoreDocument()._shallowCloneInternal();
    }
}

function _maybeGetIotaDocumentInternal(arg: IAsCoreDocument): IotaDocument | void {
    if (arg instanceof IotaDocument) {
        return arg._shallowCloneInternal();
    } else {
        return;
    }
}

globalThis._getCoreDocumentInternal = _getCoreDocumentInternal;

globalThis._maybeGetIotaDocumentInternal = _maybeGetIotaDocumentInternal;
