function getCoreDocument(arg) {
    if (arg instanceof CoreDocument) {
        return arg.shallowClone();
    } else {
        return arg.asCoreDocument().shallowClone();
    }
}

function maybeGetIotaDocument(arg) {
    if (arg instanceof IotaDocument) {
        return arg.shallowClone()
    } else {
        return;
    }
}