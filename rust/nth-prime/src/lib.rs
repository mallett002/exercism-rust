// Prime - has only 1 and itself as factors and is greater than 1
// ex: 2, 3, 5, 7, 11, 13
// not: 4 (2), 6(3,2), 8(4,2), 9(3)
// 2 is only even #

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    // now we know it's odd
    // see if we have a factor other than 1 and itself
    // factors, start at 3, go up to next odd number (+2) each time.
    // see if n is divisble by that factor, if so, not prime.
    // once we reach a number that is passed midpoint of n, we're done (no factor found, so return true - is prime)
    let mut factor = 3;
    let mid = n / 2;

    println!("mid: {}", mid);

    while factor <= mid {
        if n % factor == 0 {
            return false;
        }

        factor += 2;
    }

    true
}

// takes in a number, find the prime number at that position (2 would be 5)
pub fn nth(nth_prime: u32) -> u32 {
    // 0: 2, 1: 3: 2: 5 
    // nth_prime = 2?
    // answer = 5

    is_prime(nth_prime);
    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_prime_returns_false_1() {
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn is_prime_returns_true_2() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn is_prime_returns_true_3() {
        assert_eq!(is_prime(3), true);
    }

    #[test]
    fn is_prime_returns_false_4() {
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn is_prime_returns_true_11() {
        assert_eq!(is_prime(11), true);
    }

    #[test]
    fn is_prime_returns_false_32() {
        assert_eq!(is_prime(32), false);
    }
}
