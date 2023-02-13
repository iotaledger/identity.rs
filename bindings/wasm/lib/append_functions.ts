import {CoreDocument, IotaDocument, IAsCoreDocument} from "~identity_wasm" ; 
type GetCoreDocument = (arg: IAsCoreDocument) => CoreDocument;
type MaybeGetIotaDocument = (arg: IAsCoreDocument) => IotaDocument | void; 

declare global {
    var getCoreDocument: GetCoreDocument;
    var maybeGetIotaDocument: MaybeGetIotaDocument; 
}
function getCoreDocument(arg: IAsCoreDocument): CoreDocument {
    if (arg instanceof CoreDocument) {
         
        return arg._shallowCloneInternal();
    } else {
        
        return arg.asCoreDocument()._shallowCloneInternal();
    }
}

function maybeGetIotaDocument(arg: IAsCoreDocument) : IotaDocument | void {
    if (arg instanceof IotaDocument) {
        
        return arg._shallowCloneInternal()
    } else {
        return;
    }
}

globalThis.getCoreDocument = getCoreDocument;

globalThis.maybeGetIotaDocument = maybeGetIotaDocument; 