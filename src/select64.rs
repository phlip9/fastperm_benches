/// Select the index of the `idx`'th bit in `mask`.
///
/// Uses the `_pdep_u64` or `_pdep_u32` intrinsics when available. See the
/// [Intel Intrinsics Guide] for more details.
///
///
/// ## Example
///
/// ```rust
/// use fastperm::select64::select64;
///
/// assert_eq!(1, select64(0, 0b10110));
/// assert_eq!(2, select64(1, 0b10110));
/// assert_eq!(4, select64(2, 0b10110));
/// ```
///
/// [Intel Intrinsics Guide]: https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_pdep_u64&expand=4152
#[inline]
pub fn select64(idx: u8, mask: u64) -> u8 {
    debug_assert!(
        mask.count_ones() > idx as u32,
        "select is undefined when the index is greater than or equal to the \
         number of set bits in the mask: index: {}, mask: {:64b}",
        idx,
        mask,
    );

    #[cfg(target_arch = "x86")]
    {
        if is_x86_feature_detected!("bmi2") {
            select64_via_pdep32(idx, mask)
        } else {
            select64_fallback(idx, mask)
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            use std::arch::x86_64::_pdep_u64;
            let idx_scatter = unsafe { _pdep_u64(1 << (idx as u64), mask) };
            idx_scatter.trailing_zeros() as u8
        } else {
            select64_fallback(idx, mask)
        }
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        select64_fallback(idx, mask)
    }
}

pub fn select64_fallback(idx: u8, mask: u64) -> u8 {
    let b0 = mask;
    let b1 = (b0 & 0x5555_5555_5555_5555) + ((b0 >> 1) & 0x5555_5555_5555_5555);
    let b2 = (b1 & 0x3333_3333_3333_3333) + ((b1 >> 2) & 0x3333_3333_3333_3333);
    let b3 = (b2 + (b2 >> 4)) & 0x0F0F_0F0F_0F0F_0F0F;
    let b4 = (b3 + (b3 >> 8)) & 0x00FF_00FF_00FF_00FF;
    let b5 = (b4 + (b4 >> 16)) & 0x0000_FFFF_0000_FFFF;
    // let b6 = (b5 + (b5 >> 32)) & 0x0000_0000_FFFF_FFFF;
    // observe: b6 == mask.count_ones()

    let mut idx = idx as u64;
    let mut r = 0;

    let b = (b5 >> r) & 0xFFFF_FFFF;
    if idx >= b {
        idx -= b;
        r += 32;
    }

    let b = (b4 >> r) & 0xFFFF;
    if idx >= b {
        idx -= b;
        r += 16;
    }

    let b = (b3 >> r) & 0xFF;
    if idx >= b {
        idx -= b;
        r += 8;
    }

    let b = (b2 >> r) & 0xF;
    if idx >= b {
        idx -= b;
        r += 4;
    }

    let b = (b1 >> r) & 0x3;
    if idx >= b {
        idx -= b;
        r += 2;
    }

    let b = (b0 >> r) & 0x1;
    if idx >= b {
        // idx -= b;
        r += 1;
    }

    r
}

// TODO(philiphayes): add "bench" feature to disable these fn's outside of benching.

pub fn pdep32_fallback(src: u32, mut mask: u32) -> u32 {
    // iterate bit from lsb -> msb
    let mut bit = 1;
    let mut r = 0;
    while mask != 0 {
        if src & bit != 0 {
            // add the lsb from mask to r
            r |= mask & mask.wrapping_neg();
        }
        // clear lsb
        mask &= mask - 1;
        bit <<= 1;
    }
    r
}

#[inline(always)]
pub fn pdep32(src: u32, mask: u32) -> u32 {
    #[cfg(target_arch = "x86")]
    {
        if is_x86_feature_detected!("bmi2") {
            use std::arch::x86::_pdep_u32;
            unsafe { _pdep_u32(src, mask) }
        } else {
            pdep32_fallback(src, mask)
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            use std::arch::x86_64::_pdep_u32;
            unsafe { _pdep_u32(src, mask) }
        } else {
            pdep32_fallback(src, mask)
        }
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        pdep32_fallback(src, mask)
    }
}

#[inline]
pub fn select64_via_pdep32(idx: u8, mask: u64) -> u8 {
    let mask_hi = (mask >> 32) as u32;
    let mask_lo = mask as u32;
    let num_bits_lo = mask_lo.count_ones();
    let idx = idx as u32;
    if idx < num_bits_lo {
        pdep32(1 << idx, mask_lo).trailing_zeros() as u8
    } else {
        (pdep32(1 << (idx - num_bits_lo), mask_hi) as u64).trailing_zeros() as u8 + 32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{fill_mask, test::arb_small_rng, BitScatter, SmallIndexPermutations, MAX_PERIOD};
    use proptest::prelude::*;

    fn arb_bit_idx_and_mask64() -> impl Strategy<Value = (u8, u64)> {
        (0..MAX_PERIOD)
            .prop_flat_map(|idx| (Just(idx), (idx + 1..=MAX_PERIOD), arb_small_rng()))
            .prop_map(|(idx, mask_bits, mut small_rng)| {
                let mut bit_scatter = BitScatter::new(&mut small_rng, 64);
                let bits_perm = bit_scatter.iter().take(mask_bits as usize);
                let mask = fill_mask(bits_perm).0;
                (idx, mask)
            })
    }

    fn assert_select64(out_idx: u8, in_idx: u8, mask: u64) {
        assert_eq!(out_idx, select64(in_idx, mask));
        assert_eq!(out_idx, select64_via_pdep32(in_idx, mask));
        assert_eq!(out_idx, select64_fallback(in_idx, mask));
    }

    #[test]
    fn test_select64() {
        let mask = 0b1101001100001101;
        assert_select64(0, 0, mask);
        assert_select64(2, 1, mask);
        assert_select64(3, 2, mask);
        assert_select64(8, 3, mask);
        assert_select64(9, 4, mask);
        assert_select64(12, 5, mask);
        assert_select64(14, 6, mask);
        assert_select64(15, 7, mask);
    }

    #[test]
    fn test_select64_max_mask() {
        for idx in 0..64_u8 {
            assert_select64(idx, idx, u64::MAX);
        }
    }

    #[test]
    fn test_select64_one_idx() {
        for idx in 0..64_u8 {
            assert_select64(idx, 0, 1 << (idx as u64));
        }
    }

    #[test]
    fn test_select64_one_idx_smear_high() {
        for idx in 0..64_u8 {
            assert_select64(idx, 0, (1_u64 << (idx as u64)).wrapping_neg());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20000))]

        #[test]
        fn test_pdep32_equiv(src in any::<u32>(), mask in any::<u32>()) {
            let expected = pdep32(src, mask);
            let actual = pdep32_fallback(src, mask);
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_select64_equiv((idx, mask) in arb_bit_idx_and_mask64()) {
            assert!(idx as u32 <= mask.count_ones());

            let expected = select64(idx, mask);
            let actual1 = select64_via_pdep32(idx, mask);
            let actual2 = select64_fallback(idx, mask);
            assert_eq!(expected, actual1);
            assert_eq!(expected, actual2);
        }
    }
}
