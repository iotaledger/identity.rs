module stardust::alias {

    /// The persisted Alias object from Stardust, without tokens and assets.
    /// Outputs owned the AliasID/Address in Stardust will be sent to this object and
    /// have to be received via this object once extracted from `AliasOutput`.
    public struct Alias has key, store {
        /// The ID of the Alias = hash of the Output ID that created the Alias Output in Stardust.
        /// This is the AliasID from Stardust.
        id: UID,

        /// The last State Controller address assigned before the migration.
        legacy_state_controller: Option<address>,
        /// A counter increased by 1 every time the alias was state transitioned.
        state_index: u32,
        /// State metadata that can be used to store additional information.
        state_metadata: Option<vector<u8>>,

        /// The sender feature.
        sender: Option<address>,
        /// The metadata feature.
        metadata: Option<vector<u8>>,

        /// The immutable issuer feature.
        immutable_issuer: Option<address>,
        /// The immutable metadata feature.
        immutable_metadata: Option<vector<u8>>,
    }

    // === Public-Mutative Functions ===
    public fun destructure(self: Alias):
        (UID, Option<address>, u32, Option<vector<u8>>, Option<address>, Option<vector<u8>>, Option<address>, Option<vector<u8>>) {
        let Alias {
            id,
            legacy_state_controller,
            state_index,
            state_metadata,
            sender,
            metadata,
            immutable_issuer,
            immutable_metadata,
        } = self;
        (id, legacy_state_controller, state_index, state_metadata, sender, metadata, immutable_issuer, immutable_metadata)
    }

    /// Destroy the `Alias` object, equivalent to `burning` an Alias Output in Stardust.
    public fun destroy(self: Alias) {
        let Alias {
            id,
            legacy_state_controller: _,
            state_index: _,
            state_metadata: _,
            sender: _,
            metadata: _,
            immutable_issuer: _,
            immutable_metadata: _,
        } = self;

        object::delete(id);
    }

    // === Public-Mutative Functions ===

    /// Get the Alias's `legacy_state_controller`.
    public fun legacy_state_controller(self: &Alias): &Option<address> {
        &self.legacy_state_controller
    }

    /// Get the Alias's `state_index`.
    public fun state_index(self: &Alias): u32 {
        self.state_index
    }

    /// Get the Alias's `state_metadata`.
    public fun state_metadata(self: &Alias): &Option<vector<u8>> {
        &self.state_metadata
    }

    /// Get the Alias's `sender`.
    public fun sender(self: &Alias): &Option<address> {
        &self.sender
    }

    /// Get the Alias's `metadata`.
    public fun metadata(self: &Alias): &Option<vector<u8>> {
        &self.metadata
    }

    /// Get the Alias's `immutable_sender`.
    public fun immutable_issuer(self: &Alias): &Option<address> {
        &self.immutable_issuer
    }

    /// Get the Alias's `immutable_metadata`.
    public fun immutable_metadata(self: &Alias): &Option<vector<u8>> {
        &self.immutable_metadata
    }

    // === Public-Package Functions ===

    /// Get the Alias's id.
    public(package) fun id(self: &mut Alias): &mut UID {
        &mut self.id
    }

    // === Test Functions ===

    // #[test_only]
    public fun create_for_testing(
        legacy_state_controller: Option<address>,
        state_index: u32,
        state_metadata: Option<vector<u8>>,
        sender: Option<address>,
        metadata: Option<vector<u8>>,
        immutable_issuer: Option<address>,
        immutable_metadata: Option<vector<u8>>,
        ctx: &mut TxContext
    )  {
        let alias = Alias {
            id: object::new(ctx),
            legacy_state_controller,
            state_index,
            state_metadata,
            sender,
            metadata,
            immutable_issuer,
            immutable_metadata,
        };
        transfer::transfer(alias, ctx.sender());
    }
}
