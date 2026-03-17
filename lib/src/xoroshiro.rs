pub struct Xoroshiro {
    lo: u64,
    hi: u64,
}

impl Xoroshiro {
    const XL: u64 = 0x9e3779b97f4a7c15;
    const XH: u64 = 0x6a09e667f3bcc909;
    const A: u64 = 0xbf58476d1ce4e5b9;
    const B: u64 = 0x94d049bb133111eb;
    const DOUBLE_MUL: f64 = 1.1102230246251565E-16;
    pub fn with_seed(seed: u64) -> Self {
        let mut l = seed ^ Xoroshiro::XH;
        let mut h = l + Xoroshiro::XL;
        l = (l ^ (l >> 30)) * Xoroshiro::A;
        h = (h ^ (h >> 30)) * Xoroshiro::A;
        l = (l ^ (l >> 27)) * Xoroshiro::B;
        h = (h ^ (h >> 27)) * Xoroshiro::B;
        l = l ^ (l >> 31);
        h = h ^ (h >> 31);
        Self {
            lo: l,
            hi: h
        }
    }

    #[inline]
    pub fn from_parts(lo: u64, hi: u64) -> Self {
        Self {
            lo, hi
        }
    }
    
    #[inline]
    pub fn next_int(&mut self, n: u32) -> i32 {
        let mut r: u64 = (self.next_long() & 0xffffffff) * n as u64;
        if (r as u32) < n {
            while r < (!n + 1) as u64 % n as u64 {
                r = (self.next_long() & 0xffffffff) * n as u64;
            }
        }
        (r >> 32) as i32
    }

    #[inline]
    pub fn next_long(&mut self) -> u64 {
        let n = (self.lo + self.hi).rotate_left(17) + self.lo;
        let h = self.hi ^ self.lo;
        self.lo = self.lo.rotate_left(49) ^ h ^ (h << 21);
        self.hi = h.rotate_left(28);
        n
    }

    #[inline]
    pub fn next_double(&mut self) -> f64 {
        (self.next_long() >> (64 - 53)) as f64 * Self::DOUBLE_MUL
    }
}

