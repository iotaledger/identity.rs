use crate::crypto::merkle_tree::consts;

/// Return true if `value` is a power of two greater than `1`.
#[inline(always)]
pub const fn is_pow2(value: usize) -> bool {
    value > 1 && value.is_power_of_two()
}

/// Returns the base-2 logarithm of `value`, rounded up.
#[inline(always)]
pub fn log2c(value: usize) -> usize {
    (consts::SIZE_USIZE * 8) - value.leading_zeros() as usize - value.is_power_of_two() as usize
}

#[cfg(test)]
mod tests {
    use super::is_pow2;
    use super::log2c;

    #[test]
    fn test_is_pow2() {
        assert_eq!(is_pow2(0), false);
        assert_eq!(is_pow2(1), false);
        assert_eq!(is_pow2(2), true);
        assert_eq!(is_pow2(3), false);
    }

    #[test]
    fn test_log2c() {
        assert_eq!(log2c(0), 0);
        assert_eq!(log2c(1), 0);
        assert_eq!(log2c(1 << 1), 1);
        assert_eq!(log2c(1 << 2), 2);
        assert_eq!(log2c(1 << 3), 3);
        assert_eq!(log2c(1 << 4), 4);
        assert_eq!(log2c(1 << 5), 5);
        assert_eq!(log2c(1 << 6), 6);
        assert_eq!(log2c(1 << 7), 7);
        assert_eq!(log2c(1 << 8), 8);
        assert_eq!(log2c(1 << 9), 9);
        assert_eq!(log2c(1 << 10), 10);
        assert_eq!(log2c(1 << 11), 11);
    }
}
