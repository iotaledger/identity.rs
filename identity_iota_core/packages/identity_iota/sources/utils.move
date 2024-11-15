module identity_iota::utils {
    use iota::vec_map::{Self, VecMap};

    const ELengthMismatch: u64 = 0;

    public fun vec_map_from_keys_values<K: store + copy, V: store>(
        mut keys: vector<K>,
        mut values: vector<V>,
    ): VecMap<K, V> {
        assert!(keys.length() == values.length(), ELengthMismatch);
        
        let mut map = vec_map::empty<K, V>();
        while (!keys.is_empty()) {
            let key = keys.swap_remove(0);
            let value = values.swap_remove(0);
            map.insert(key, value);
        };
        keys.destroy_empty();
        values.destroy_empty();

        map
    }

    #[test]
    fun from_keys_values_works() {
        let addresses = vector[@0x1, @0x2];
        let vps = vector[1, 1];

        let map = vec_map_from_keys_values(addresses, vps);
        assert!(map.size() == 2, 0);
    }
}