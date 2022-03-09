
pub mod util {
    pub fn modu(dividend: u32, divisor: u32) -> u32 {
        return ((dividend % divisor) + divisor) % divisor;
    }
}
