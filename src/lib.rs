use crate::select64::select64;
use rand::{seq::SliceRandom, Rng};
use std::iter::{IntoIterator, Iterator, Take};

pub mod select64;

pub const MAX_PERIOD: u8 = 64;

////////////////////////////
// SmallIndexPermutations //
////////////////////////////

pub trait SmallIndexPermutations {
    fn period(&self) -> u8;

    fn next_index(&mut self) -> u8;

    fn reset(&mut self);

    #[inline]
    fn iter(&mut self) -> Iter<&mut Self> {
        Iter::new(self)
    }

    fn iter_period(&mut self) -> Take<Iter<&mut Self>> {
        let n = self.period();
        Iter::new(self).take(n as usize)
    }

    #[inline]
    fn into_iter(self) -> Iter<Self>
    where
        Self: Sized,
    {
        Iter::new(self)
    }

    fn into_iter_period(self) -> Take<Iter<Self>>
    where
        Self: Sized,
    {
        let n = self.period();
        Iter::new(self).take(n as usize)
    }
}

impl<T: SmallIndexPermutations + ?Sized> SmallIndexPermutations for &mut T {
    fn period(&self) -> u8 {
        (**self).period()
    }

    fn next_index(&mut self) -> u8 {
        (**self).next_index()
    }

    fn reset(&mut self) {
        (**self).reset()
    }
}

pub struct Iter<T: SmallIndexPermutations> {
    inner: T,
}

impl<T: SmallIndexPermutations> Iter<T> {
    fn new(mut inner: T) -> Self {
        inner.reset();
        Self { inner }
    }
}

impl<T: SmallIndexPermutations> Iterator for Iter<T> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_index())
    }
}

/////////////
// Shuffle //
/////////////

pub struct Shuffle<R> {
    rng: R,
    idxs: Vec<u8>,
    idx: u8,
}

impl<R: Rng> Shuffle<R> {
    pub fn new(rng: R, n: u8) -> Self {
        assert!(n <= MAX_PERIOD && n > 0);
        let idxs = (0..n).into_iter().collect::<Vec<_>>();
        Self { rng, idxs, idx: n }
    }
}

impl<R: Rng> SmallIndexPermutations for Shuffle<R> {
    #[inline]
    fn period(&self) -> u8 {
        self.idxs.len() as u8
    }

    fn next_index(&mut self) -> u8 {
        if self.idx == self.period() {
            self.reset();
        }
        let r = unsafe { *self.idxs.get_unchecked(self.idx as usize) };
        self.idx += 1;
        r
    }

    #[inline]
    fn reset(&mut self) {
        self.idx = 0;
        (&mut self.idxs).shuffle(&mut self.rng);
    }
}

//////////////////
// ShuffleArray //
//////////////////

const fn make_indexes() -> [u8; MAX_PERIOD as usize] {
    let mut i = 0_u8;
    let mut indexes = [0_u8; MAX_PERIOD as usize];
    // can't use for loop yet...
    loop {
        indexes[i as usize] = i;
        i += 1;
        if i == MAX_PERIOD {
            break;
        }
    }
    indexes
}

const INDEXES: [u8; MAX_PERIOD as usize] = make_indexes();

pub struct ShuffleArray<R> {
    rng: R,
    idxs: [u8; MAX_PERIOD as usize],
    n: u8,
    idx: u8,
}

impl<R: Rng> ShuffleArray<R> {
    pub fn new(rng: R, n: u8) -> Self {
        assert!(n <= MAX_PERIOD && n > 0);
        Self {
            rng,
            idxs: INDEXES,
            n,
            idx: n,
        }
    }
}

impl<R: Rng> SmallIndexPermutations for ShuffleArray<R> {
    #[inline]
    fn period(&self) -> u8 {
        self.n
    }

    #[inline]
    fn reset(&mut self) {
        self.idx = 0;
        (&mut self.idxs[..self.n as usize]).shuffle(&mut self.rng);
    }

    fn next_index(&mut self) -> u8 {
        if self.idx == self.period() {
            self.reset();
        }
        let r = unsafe { *self.idxs.get_unchecked(self.idx as usize) };
        self.idx += 1;
        r
    }
}

/////////////////////////////
// ShuffleArrayIncremental //
/////////////////////////////

pub struct ShuffleArrayIncremental<R> {
    rng: R,
    idxs: [u8; MAX_PERIOD as usize],
    n: u8,
    idx: u8,
}

impl<R: Rng> ShuffleArrayIncremental<R> {
    pub fn new(rng: R, n: u8) -> Self {
        assert!(n <= MAX_PERIOD && n > 0);
        Self {
            rng,
            idxs: INDEXES,
            n,
            idx: 0,
        }
    }
}

impl<R: Rng> SmallIndexPermutations for ShuffleArrayIncremental<R> {
    #[inline(always)]
    fn period(&self) -> u8 {
        self.n
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.idx = 0;
    }

    #[inline]
    fn next_index(&mut self) -> u8 {
        // as mentioned below, don't care about modulo bias for small period.
        let idx_swap = fastmap32(self.rng.next_u32(), (self.n - self.idx) as u32) + self.idx as u32;
        self.idxs.swap(self.idx as usize, idx_swap as usize);

        // ~20-50% faster than: let r = self.idxs[self.idx as usize];
        debug_assert!(self.idx < self.n);
        let r = unsafe { *self.idxs.get_unchecked(self.idx as usize) };
        self.idx = if self.idx + 1 == self.n {
            0
        } else {
            self.idx + 1
        };
        r
    }
}

////////////////
// BitScatter //
////////////////

pub struct BitScatter<R> {
    rng: R,
    unchosen_mask: u64,
    n: u8,
    // number of remaining indices, i.e., number of one bits in the unchosen mask
    m: u8,
}

impl<R: Rng> BitScatter<R> {
    pub fn new(rng: R, n: u8) -> Self {
        assert!(n <= MAX_PERIOD && n > 0);
        Self {
            rng,
            unchosen_mask: index_mask(n as u32),
            n,
            m: n,
        }
    }
}

impl<R: Rng> SmallIndexPermutations for BitScatter<R> {
    #[inline(always)]
    fn period(&self) -> u8 {
        self.n
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.m = self.n;
        self.unchosen_mask = index_mask(self.n as u32);
    }

    #[inline]
    fn next_index(&mut self) -> u8 {
        if self.m == 0 {
            self.reset();
        }

        // map sample to an index in [0, m)
        // benched 2x slower: let idx_mspace = self.rng.gen_range(0..m) as u8;
        // benched ~17-60% faster than: let idx_mspace = (next_u32 % m) as u8;
        let idx_mspace = fastmap32(self.rng.next_u32(), self.m as u32) as u8;
        // get the index in [0, n) of the corresponding 1-bit in the unchosen mask
        let idx_nspace = select64(idx_mspace, self.unchosen_mask);
        // unset the newly sampled index
        self.unchosen_mask &= !(1_u64 << (idx_nspace as u32));
        self.m -= 1;

        idx_nspace
    }
}

#[inline(always)]
const fn index_mask(idx: u32) -> u64 {
    u64::MAX >> (64 - idx)
}

// Map `x` uniformly into the range `[0, n)`.
// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
#[inline(always)]
const fn fastmap32(x: u32, n: u32) -> u32 {
    let mul = (x as u64).wrapping_mul(n as u64);
    (mul >> 32) as u32
}

// TODO(philiphayes): disable outside of test/bench

pub fn fill_mask(idxs: impl Iterator<Item = u8>) -> (u64, u8) {
    let mut mask = 0;
    let mut count = 0;
    for idx in idxs {
        mask |= 1 << (idx as u64);
        count += 1;
    }
    (mask, count)
}

//////////
// Test //
//////////

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use rand::{rngs::SmallRng, SeedableRng};

    pub fn arb_small_rng() -> impl Strategy<Value = SmallRng> {
        any::<u64>().prop_map(SmallRng::seed_from_u64).no_shrink()
    }

    fn small_rng() -> SmallRng {
        SmallRng::seed_from_u64(0xDEAD_BEEF_F000_BA55)
    }

    fn assert_permutation(n: u8, idxs: impl Iterator<Item = u8>) {
        let (mask, count) = fill_mask(idxs);
        assert_eq!(count, n);

        let expected_mask: u64 = index_mask(n as u32);
        assert_eq!(mask, expected_mask);
    }

    #[test]
    fn test_index_mask() {
        assert_eq!(0b0000_0001, index_mask(1));
        assert_eq!(0b0001_1111, index_mask(5));
        assert_eq!(u64::MAX, index_mask(64));
    }

    #[test]
    fn test_shuffle() {
        let n = 10;
        let mut s = Shuffle::new(small_rng(), n);
        let idxs = s.iter_period();
        assert_permutation(n, idxs);
    }

    #[test]
    fn test_shuffle_array() {
        let n = 10;
        let mut s = ShuffleArray::new(small_rng(), n);
        let idxs = s.iter_period();
        assert_permutation(n, idxs);
    }

    #[test]
    fn test_shuffle_array_incremental() {
        let n = 10;
        let mut s = ShuffleArrayIncremental::new(small_rng(), n);
        let idxs = s.iter_period();
        assert_permutation(n, idxs);
    }

    #[test]
    fn test_bit_scatter() {
        let n = 10;
        let mut s = BitScatter::new(small_rng(), n);
        let idxs = s.iter_period();
        assert_permutation(n, idxs);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        #[test]
        fn sorted_shuffle_eq_range(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut shuffle = Shuffle::new(rng, n);
            let mut idxs = shuffle.iter_period().collect::<Vec<_>>();
            idxs.sort_unstable();

            let expected_idxs = (0..n).into_iter().collect::<Vec<_>>();
            assert_eq!(expected_idxs, idxs);
        }

        #[test]
        fn shuffle_is_permutation(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut shuffle = Shuffle::new(rng, n);
            assert_permutation(n, shuffle.iter_period());
        }

        #[test]
        fn sorted_shuffle_array_eq_range(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut shuffle = ShuffleArray::new(rng, n);
            let mut idxs = shuffle.iter_period().collect::<Vec<_>>();
            idxs.sort_unstable();

            let expected_idxs = (0..n).into_iter().collect::<Vec<_>>();
            assert_eq!(expected_idxs, idxs);
        }

        #[test]
        fn shuffle_array_is_permutation(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut shuffle = ShuffleArray::new(rng, n);
            assert_permutation(n, shuffle.iter_period());
        }

        #[test]
        fn sorted_shuffle_array_incremental_eq_range(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut shuffle = ShuffleArrayIncremental::new(rng, n);
            let mut idxs = shuffle.iter_period().collect::<Vec<_>>();
            idxs.sort_unstable();

            let expected_idxs = (0..n).into_iter().collect::<Vec<_>>();
            assert_eq!(expected_idxs, idxs);
        }

        #[test]
        fn shuffle_array_incremental_is_permutation(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut shuffle = ShuffleArrayIncremental::new(rng, n);
            assert_permutation(n, shuffle.iter_period());
        }

        #[test]
        fn sorted_bit_scatter_eq_range(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut bit_scatter = BitScatter::new(rng, n);
            let mut idxs = bit_scatter.iter_period().collect::<Vec<_>>();
            idxs.sort_unstable();

            let expected_idxs = (0..n).into_iter().collect::<Vec<_>>();
            assert_eq!(expected_idxs, idxs);
        }

        #[test]
        fn bit_scatter_is_permutation(n in 1..=MAX_PERIOD, rng in arb_small_rng()) {
            let mut bit_scatter = BitScatter::new(rng, n);
            assert_permutation(n, bit_scatter.iter_period());
        }
    }
}
