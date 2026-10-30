pub fn square(s: u32) -> u64 {
    // 2^n-1
    (2 as u64).pow(s - 1) as u64
}

pub fn total_manual() -> u64 {
    let mut total: u64 = 0;

    for n in 1..65 {
        total += square(n);
    }

    total
}

pub fn total() -> u64 {
    (1..65).fold(0, |acc, n| acc + square(n))
}
