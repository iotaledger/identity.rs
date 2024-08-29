module identity_iota::public_vc {
    public struct PublicVc {
        data: vector<u8>,
    }

    public fun new(data: vector<u8>): PublicVc {
        PublicVc { data }
    }

    public fun data(self: &PublicVc): &vector<u8> {
        &self.data
    }

    public fun set_data(self: &mut PublicVc, data: vector<u8>) {
        self.data = data
    }
}