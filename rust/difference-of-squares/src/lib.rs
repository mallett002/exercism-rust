// (1 + 2 + ... + 10)² = 55² = 3025.
pub fn square_of_sum2(n: u32) -> u32 {
    (((1..n + 1).sum::<u32>() as f64) as u32).pow(2)
}

// 1² + 2² + ... + 10² = 385.
pub fn sum_of_squares2(n: u32) -> u32 {
    (1..n + 1).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

// --------------------------
// Manual Approach
// --------------------------

pub fn square_of_sum(n: u32) -> u32 {
    let mut curr = 1;
    let mut sum = 0;

    while curr <= n {
        sum += curr;
        curr += 1;
    }

    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut curr = 1;
    let mut total = 0;

    while curr <= n {
        total += curr.pow(2);
        curr += 1;
        println!("curr: {curr}");
    }

    total
}
