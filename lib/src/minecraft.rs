pub mod chunk;
pub mod biome_btree;
pub mod biome;

pub struct QuartPos;

impl QuartPos {
    #[inline(always)]
    pub fn from_block(i: i32) -> i32 { i >> 2 }
    #[inline(always)]
    pub fn quart_local(i: i32) -> i32 { i & 3 }
    #[inline(always)]
    pub fn to_block(i: i32) -> i32 { i << 2 }
    #[inline(always)]
    pub fn from_section(i: i32) -> i32 { i << 2 }
    #[inline(always)]
    pub fn to_section(i: i32) -> i32 { i >> 2 }
}

pub struct RandomState {
    state: i64
}

pub struct RandomSupport {
}

impl RandomSupport {
    const GOLDEN_RATIO_64: i64 = -7046029254386353131;
    const SILVER_RATIO_64: i64 = 7640891576956012809;
}


// this is an interface in the java code
pub struct RandomSource {
}

pub struct LegacyRandomSource {
    seed: i64,
}


// this is the same as JavaUtilRandom?
impl LegacyRandomSource {
    const MODULUS_BITS: i32 = 48;
    const MODULUS_MASK: i64 = 281474976710655;
    const MULTIPLIER: i64 = 25214903917;
    const INCREMENT: i64 = 11;
    
    pub fn new(l: i64) -> Self {
        Self {
            seed: l
        }
    }

    pub fn set_seed(&mut self, l: i64) {
        self.seed = l
    }

    pub fn next(&mut self, i: i32) -> i32 {
        let l = self.seed;
        let m = l * 25214903917 + 11 & 281474976710655;
        todo!()
    }

    pub fn next_double(&mut self) -> f64 {
        todo!()
    }
}
