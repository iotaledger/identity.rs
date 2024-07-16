// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::nft {

    use std::string;

    use iota::display;
    use iota::package;

    use stardust::irc27::{Self, Irc27Metadata};

    /// One Time Witness.
    public struct NFT has drop {}

    /// The Stardust NFT representation.
    public struct Nft has key, store {
        /// The Nft's ID is nested from Stardust.
        id: UID,

        /// The sender feature holds the last sender address assigned before the migration and
        /// is not supported by the protocol after it.
        legacy_sender: Option<address>,
        /// The metadata feature.
        metadata: Option<vector<u8>>,
        /// The tag feature.
        tag: Option<vector<u8>>,

        /// The immutable issuer feature.
        immutable_issuer: Option<address>,
        /// The immutable metadata feature.
        immutable_metadata: Irc27Metadata,
    }

    /// The `Nft` module initializer.
    fun init(otw: NFT, ctx: &mut TxContext) {
        // Claim the module publisher.
        let publisher = package::claim(otw, ctx);

        // Build a `Display` object.
        let keys = vector[
            // The Iota standard fields.
            string::utf8(b"name"),
            string::utf8(b"image_url"),
            string::utf8(b"description"),
            string::utf8(b"creator"),

            // The extra IRC27-nested fileds.
            string::utf8(b"version"),
            string::utf8(b"media_type"),
            string::utf8(b"collection_name"),
        ];

        let values = vector[
            // The Iota standard fields.
            string::utf8(b"{immutable_metadata.name}"),
            string::utf8(b"{immutable_metadata.uri}"),
            string::utf8(b"{immutable_metadata.description}"),
            string::utf8(b"{immutable_metadata.issuer_name}"),

            // The extra IRC27-nested fileds.
            string::utf8(b"{immutable_metadata.version}"),
            string::utf8(b"{immutable_metadata.media_type}"),
            string::utf8(b"{immutable_metadata.collection_name}"),
        ];

        let mut display = display::new_with_fields<Nft>(
            &publisher,
            keys,
            values,
            ctx
        );

        // Commit the first version of `Display` to apply changes.
        display.update_version();

        // Burn the publisher object.
        package::burn_publisher(publisher);

        // Freeze the display object.
        iota::transfer::public_freeze_object(display);
    }

    /// Permanently destroy an `Nft` object.
    public fun destroy(nft: Nft) {
        let Nft {
            id,
            legacy_sender: _,
            metadata: _,
            tag: _,
            immutable_issuer: _,
            immutable_metadata,
        } = nft;

        irc27::destroy(immutable_metadata);

        object::delete(id);
    }

    /// Get the Nft's `legacy_sender`.
    public fun legacy_sender(nft: &Nft): &Option<address> {
        &nft.legacy_sender
    }

    /// Get the Nft's `metadata`.
    public fun metadata(nft: &Nft): &Option<vector<u8>> {
        &nft.metadata
    }

    /// Get the Nft's `tag`.
    public fun tag(nft: &Nft): &Option<vector<u8>> {
        &nft.tag
    }

    /// Get the Nft's `immutable_sender`.
    public fun immutable_issuer(nft: &Nft): &Option<address> {
        &nft.immutable_issuer
    }

    /// Get the Nft's `immutable_metadata`.
    public fun immutable_metadata(nft: &Nft): &Irc27Metadata {
        &nft.immutable_metadata
    }

    /// Get the Nft's id.
    public(package) fun id(self: &mut Nft): &mut UID {
        &mut self.id
    }

    #[test_only]
    public fun create_for_testing(
        legacy_sender: Option<address>,
        metadata: Option<vector<u8>>,
        tag: Option<vector<u8>>,
        immutable_issuer: Option<address>,
        immutable_metadata: Irc27Metadata,
        ctx: &mut TxContext,
    ): Nft {
        Nft {
            id: object::new(ctx),
            legacy_sender,
            metadata,
            tag,
            immutable_issuer,
            immutable_metadata,
        }
    }
}
