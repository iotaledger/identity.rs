/// Returns the number of bits required to uniquely represt `len` different statuses
pub(crate) fn bit_required(len: usize) -> usize {
  std::mem::size_of::<usize>() * 8 - len.next_power_of_two().leading_zeros() as usize - 1
}
