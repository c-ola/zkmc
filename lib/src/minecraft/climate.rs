pub type Climate = [u64; 6];

#[inline(always)]
pub fn quantize(v: f64) -> u64 {
    (10000.0 * v) as i64 as u64
}
