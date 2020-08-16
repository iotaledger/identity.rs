use identity_integration::did_helper::did_iota_address;

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn has_81_trytes() {
    assert_eq!(did_iota_address(String::from("123456789abcdefghi")).len(), 81);
  }
  #[test]
  fn same_address() {
    assert_eq!(
      did_iota_address(String::from("123456789abcdefghi")),
      String::from("TBCIWFJ9VBBDN9IGEEKCLCCBIDCDWCGIAIZ9AFHCZBDFCGQBLFQGXBSAKATAVA9GPDCDOHZDQ9I9XGACE")
    );
  }
}
