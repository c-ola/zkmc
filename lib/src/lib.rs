pub mod xoroshiro;
pub mod spline;
pub mod minecraft;
pub mod noise;
pub mod tree;

use std::collections::HashSet;

use alloy_sol_types::sol;

use crate::minecraft::chunk::{ChunkGeneratorStructureState, ConcentricRingPlacement};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

pub fn generate_strongholds() {
    // preferred_biomes is something else ill figure it out later
    let placement = ConcentricRingPlacement::new(32, 128, 3, HashSet::new());
    let seed = 1;
    let chunk_generator_structure_state = ChunkGeneratorStructureState::new(seed); 
    let positions = chunk_generator_structure_state.generate_ring_positions(placement);
    for (i, position) in positions.iter().enumerate() {
        println!("{i}: {}, {}", ((position.x * 16) & !15) + 4, (((position.y * 16) & !15) + 4) * -1);
    }

}


#[derive(Clone)]
pub struct JavaUtilRandom {
    pub seed: i64,
    next_next_gaussian: Option<f64>,
}

impl JavaUtilRandom {

    const M: i64 = 0x5deece66d;
    pub fn new() -> Self {
        Self { seed: 0, next_next_gaussian: None }
    }

    pub fn with_seed(seed: i64) -> Self {
        Self {
            seed: Self::init_seed(seed),
            next_next_gaussian: None
        }
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = Self::init_seed(seed);
    }

    pub fn fork(&mut self) -> Self {
        Self::with_seed(self.next_long())
    }

    #[inline]
    fn init_seed(seed: i64) -> i64 {
        (seed ^ Self::M) & ((1 << 48) - 1)
    }

    #[inline]
    pub fn next(&mut self, bits: i32) -> i32 {
        self.seed = (self.seed.wrapping_mul(Self::M).wrapping_add(0xb)) & ((1 << 48) - 1);
        (self.seed >> (48 - bits as i64)) as i32
    }

    #[inline]
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    #[inline]
    pub fn next_int_bound(&mut self, bound: i32) -> i32 {
        assert_ne!(bound, 0);
        if bound & (-1 * bound) == bound {
            return (bound as i64 * (self.next(31) as i64) >> 31) as i32;
        }
        let mut bits;
        let mut val;
        loop {
            bits = self.next(31);
            val = bits % bound;
            if bits - val + (bound - 1) >= 0 {
                break
            }
        }
        return val;
    }

    #[inline]
    pub fn next_bytes<T: AsMut<[u8]>>(&mut self, bytes: &mut T) {
        let bytes = bytes.as_mut();
        let mut i = 0;
        while i < bytes.len() {
            let rnd = self.next_int();
            let mut n = (bytes.len() - i).min(4);
            while n > 0 {
                bytes[i] = rnd as u8;
                i += 1;
                n -= 1;
            }
        }
    }

    #[inline]
    pub fn next_long(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) + self.next(32) as i64
    }

    #[inline]
    pub fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    #[inline]
    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }

    #[inline]
    pub fn next_double(&mut self) -> f64 {
        (((self.next(26) as i64) << 27i64) + self.next(27) as i64) as f64 / (1i64 << 53i64) as f64
            /*let i = self.next(26) as i64;
              let j = self.next(27) as i64;
              let l = (i << 27) + j;
              l as f64 * 1.110223e-16*/
    }

    pub fn next_gaussian(&mut self) -> f64 {
        if let Some(next_next_gaussian) = self.next_next_gaussian.take() {
            return next_next_gaussian;
        } else {
            let (mut v1, mut v2, mut s): (f64, f64, f64);
            loop {
                v1 = 2.0 * self.next_double() - 1.0;
                v2 = 2.0 * self.next_double() - 1.0;
                s = v1 * v1 + v2 * v2;
                if !(s >= 1.0 || s == 0.0) {
                    break
                }
            }

            let multiplier = f64::sqrt(-2.0 * s.ln() / s);
            self.next_next_gaussian = Some(v2 * multiplier);
            return v1 * multiplier;
        }
    }

    // this can be implemented as an iterator or something like that, take in some kinda generator
    // maybe
    pub fn ints(&mut self, n: usize) -> Vec<i32> {
        (0..n).into_iter().map(|_| self.next_int()).collect()
    }
}

#[cfg(test)]
mod rand_tests {
    use crate::JavaUtilRandom;

    #[test]
    pub fn test_next_int() {
        let mut random = JavaUtilRandom::with_seed(1);
        let values = [-1155869325,431529176,1761283695,1749940626,892128508,155629808,1429008869,-1465154083,-138487339,-1242363800,26273138,655996946,-155886662,685382526,-258276172,-1915244828];
        for i in 0..16 {
            assert_eq!(values[i], random.next_int());
        }
    }

    #[test]
    pub fn test_next_long() {
        let mut random = JavaUtilRandom::with_seed(1);
        let values = [-4964420948893066024,7564655870752979346,3831662765844904176,6137546356583794141,-594798593157429144,112842269129291794,-669528114487223426,-1109287713991315740,-974081879987450628,-1160629452687687109,7326573195622447256,6410576364588137014,5424394867226112926,-9103770306483490189,2139215297105423308,-4232865876030345843];
        for i in 0..16 {
            assert_eq!(values[i], random.next_long());
        }
    }

    #[test]
    pub fn test_next_float() {
        let mut random = JavaUtilRandom::with_seed(1);
        let values = [0.7308782,0.100473166,0.4100808,0.40743977,0.2077148,0.036235332,0.332717,0.6588672,0.96775585,0.7107396,0.006117165,0.15273619,0.96370476,0.15957803,0.93986535,0.55407226];
        for i in 0..16 {
            assert_eq!(values[i], random.next_float());
        }
    }

    #[test]
    pub fn test_next_double() {
        let mut random = JavaUtilRandom::with_seed(1);
        let values = [0.7308781907032909,0.41008081149220166,0.20771484130971707,0.3327170559595112,0.9677559094241207,0.006117182265761301,0.9637047970232077,0.9398653887819098,0.9471949176631939,0.9370821488959696,0.3971743421847056,0.34751802920311026,0.29405703200403677,0.5064836273262351,0.11596708803265776,0.7705358800791777];
        for i in 0..16 {
            assert_eq!(values[i], random.next_double());
        }
    }

}
