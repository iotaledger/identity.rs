use identity_integration::did_helper::did_iota_address;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_81_trytes() {
        assert_eq!(did_iota_address("").unwrap().len(), 81);
        assert_eq!(did_iota_address("123456789").unwrap().len(), 81);
        assert_eq!(did_iota_address("123456789abcdefghi").unwrap().len(), 81);
    }
    #[test]
    fn same_address() {
        assert_eq!(
            did_iota_address("123456789").unwrap(),
            String::from("WAHCTBYCXCABDDICVAQCSCXBQCLBLBBCIDLDZCQCPBFDCBADEDVCACUBFCNBBDRBGCECGDWCHCMDTCZBX")
        );
        assert_eq!(
            did_iota_address("123456789abcdefghi").unwrap(),
            String::from("XAVAUCWBCDTCBBPBOBVCFDBCNBPBZAWBVAICACXCLBPBFDXCQCWACBXBWA9CDDLBTCICWBSCQBTCWCSCN")
        );
    }
}
