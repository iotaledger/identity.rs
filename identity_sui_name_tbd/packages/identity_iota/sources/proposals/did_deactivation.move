module identity_iota::did_deactivation_proposal {
    public struct DidDeactivation has store, copy, drop {}

    public fun new(): DidDeactivation {
        DidDeactivation {}
    }
}