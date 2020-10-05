use identity_core::did::DID;
use identity_integration::helpers::get_iota_address;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_81_trytes() {
        let did = DID::parse_from_str("did:iota:a").unwrap();
        assert_eq!(get_iota_address(&did).unwrap().len(), 81);
        let did = DID::parse_from_str("did:iota:123456789").unwrap();
        assert_eq!(get_iota_address(&did).unwrap().len(), 81);
        let did = DID::parse_from_str("did:iota:123456789abcdefghi").unwrap();
        assert_eq!(get_iota_address(&did).unwrap().len(), 81);
    }
    #[test]
    fn same_address() {
        let did = DID::parse_from_str("did:iota:123456789").unwrap();
        assert_eq!(
            get_iota_address(&did).unwrap(),
            String::from("WAHCTBYCXCABDDICVAQCSCXBQCLBLBBCIDLDZCQCPBFDCBADEDVCACUBFCNBBDRBGCECGDWCHCMDTCZBX")
        );
        let did = DID::parse_from_str("did:iota:123456789abcdefghi").unwrap();
        assert_eq!(
            get_iota_address(&did).unwrap(),
            String::from("XAVAUCWBCDTCBBPBOBVCFDBCNBPBZAWBVAICACXCLBPBFDXCQCWACBXBWA9CDDLBTCICWBSCQBTCWCSCN")
        );
    }
}
