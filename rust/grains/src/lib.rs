pub fn square(s: u32) -> u64 {
    // 2^n-1
    ( 2 as u32 ).pow(s - 1) as u64
}

pub fn total() -> u64 {
    ( 2 as u64 ).pow(63) as u64
}
