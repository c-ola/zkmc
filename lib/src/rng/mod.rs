pub mod xoroshiro;
pub mod java_rand;

pub use self::xoroshiro::Xoroshiro;
pub use self::java_rand::JavaUtilRandom;

pub trait RandomSource {
    fn next_i32(&mut self) -> i32;
    fn next_i32_bound(&mut self, bound: i32) -> i32;
    fn next_i64(&mut self) -> i64;
    fn next_u64(&mut self) -> u64;
    fn next_f64(&mut self) -> f64;
}
