pub mod spline;
pub mod fixed_noise;
pub mod noise_f32;

pub use spline::*;

pub struct Section;

impl Section {
    #[inline(always)]
    pub fn to_block_coord(i: i32) -> i32 {
        i << 4
    }
    #[inline(always)]
    pub fn to_block_coord_ex(i: i32, j: i32) -> i32 {
        (i << 4) + j
    }
}

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
