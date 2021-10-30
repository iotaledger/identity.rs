use std::convert::TryInto;

#[derive(Copy, Clone)]
pub enum MessageVersion {
  V1 = 1
}
static CURRENT_VERSION: MessageVersion = MessageVersion::V1;

pub fn add_version_flag(mut compressed_data: Vec<u8>) -> Vec<u8> {
  let version_flag = CURRENT_VERSION as u8;
  compressed_data.splice(0..0, [version_flag].iter().cloned());
  compressed_data
}

#[test]
fn test_add_version_flag() {
  let mut message: Vec<u8> = vec![10, 4, 5, 5];
  let message = add_version_flag(message);
  assert_eq!(message, [1, 10, 4, 5, 5])
}