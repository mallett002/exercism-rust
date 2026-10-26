pub fn square_of_sum(n: u32) -> u32 {
    // (1 + 2 + ... + 10)² = 55² = 3025.
    (((1..n + 1).sum::<u32>() as f64) as u32).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // 1² + 2² + ... + 10² = 385.
    (1..n + 1)
        .map(|i| i.pow(2))
        .sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
