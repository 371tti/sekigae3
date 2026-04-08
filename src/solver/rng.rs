use getrandom::getrandom;

/// 疑似乱数き
/// seed = 0 のとき、システムから乱数とる
pub(crate) struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub(crate) fn new(seed: u64) -> Self {
        if seed != 0 {
            return Self {
                state: seed ^ 0x9E37_79B9_7F4A_7C15,
            };
        }

        let mut entropy = [0u8; 8];
        let system_seed = if getrandom(&mut entropy).is_ok() {
            u64::from_le_bytes(entropy)
        } else {
            0xA076_1D64_78BD_642F
        };
        Self {
            state: system_seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        // SplitMix64
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    #[inline]
    pub(crate) fn next_f32(&mut self) -> f32 {
        const SCALE: f32 = 1.0 / ((1u32 << 24) as f32);
        ((self.next_u64() >> 40) as u32 as f32) * SCALE
    }

    fn gen_below(&mut self, upper: usize) -> usize {
        debug_assert!(upper > 0);
        let upper = upper as u64;
        let zone = u64::MAX - (u64::MAX % upper);
        loop {
            let v = self.next_u64();
            if v < zone {
                return (v % upper) as usize;
            }
        }
    }

    #[inline]
    pub(crate) fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        debug_assert!(range.start < range.end);
        range.start + self.gen_below(range.end - range.start)
    }

    pub(crate) fn shuffle<T>(&mut self, slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }
        for i in (1..slice.len()).rev() {
            let j = self.gen_below(i + 1);
            slice.swap(i, j);
        }
    }
}
