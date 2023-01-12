
/// Implementation of an algorithm for primality test
/// The general idea is to try to divide the number N by
/// all numbers between 2 and N - 1, and see if it can be
/// divided by any. But there are optimizations that allow
/// to test for a much reduced list of numbers (2 for each
/// 6 numbers between 6 and sqaured root of N).
pub fn primality_test_v1(n: u64) -> Option<bool> {
    if n == 0 || n == 1 { return None }
    if n == 2 || n == 3 { return Some(true) }
    if n % 2 == 0 || n % 3 == 0 { return Some(false) }

    let limit = (n as f64).powf(0.5).ceil() as u64 + 1;
    for i in (6..limit).step_by(6) {
        if n % (i - 1) == 0 || n % (i + 1) == 0 {
            return Some(false);
        }
    }
    return Some(true);
}

/// Same as `primality_test_v2`, but with a minor optimization
/// that avoids one addition inside the loop. This should be
/// useful to test if our benchmark system can detect something
/// as subtle.
pub fn primality_test_v2(n: u64) -> Option<bool> {
    if n == 0 || n == 1 { return None }
    if n == 2 || n == 3 { return Some(true) }
    if n % 2 == 0 || n % 3 == 0 { return Some(false) }

    let limit = (n as f64).powf(0.5).ceil() as u64;
    for i in (5..limit).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return Some(false);
        }
    }
    return Some(true);
}

pub fn num_primes_until(n: u64) -> u64 {
    let mut count = 0;
    for i in 2..n {
        if primality_test_v1(i) == Some(true) { count += 1 }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        assert_eq!(primality_test_v1(0), None);
        assert_eq!(primality_test_v1(1), None);
        assert_eq!(primality_test_v1(2), Some(true));
        assert_eq!(primality_test_v1(3), Some(true));
        assert_eq!(primality_test_v1(4), Some(false));
        assert_eq!(primality_test_v1(5), Some(true));
        assert_eq!(primality_test_v1(6), Some(false));
        assert_eq!(primality_test_v1(7), Some(true));
        assert_eq!(primality_test_v1(8), Some(false));
        assert_eq!(primality_test_v1(9), Some(false));
        assert_eq!(primality_test_v1(451), Some(false));
        assert_eq!(primality_test_v1(599), Some(true));
        assert_eq!(primality_test_v1(600), Some(false));
        assert_eq!(primality_test_v1(601), Some(true));
        assert_eq!(primality_test_v1(99991), Some(true));
        assert_eq!(primality_test_v1(99997), Some(false));
        assert_eq!(primality_test_v1(9007199254740881), Some(true));
        assert_eq!(primality_test_v1(9007199254740991), Some(false));
    }

    #[test]
    fn test_v2() {
        assert_eq!(primality_test_v2(0), None);
        assert_eq!(primality_test_v2(1), None);
        assert_eq!(primality_test_v2(2), Some(true));
        assert_eq!(primality_test_v2(3), Some(true));
        assert_eq!(primality_test_v2(4), Some(false));
        assert_eq!(primality_test_v2(5), Some(true));
        assert_eq!(primality_test_v2(6), Some(false));
        assert_eq!(primality_test_v2(7), Some(true));
        assert_eq!(primality_test_v2(8), Some(false));
        assert_eq!(primality_test_v2(9), Some(false));
        assert_eq!(primality_test_v2(451), Some(false));
        assert_eq!(primality_test_v2(599), Some(true));
        assert_eq!(primality_test_v2(600), Some(false));
        assert_eq!(primality_test_v2(601), Some(true));
        assert_eq!(primality_test_v2(99991), Some(true));
        assert_eq!(primality_test_v2(99997), Some(false));
        assert_eq!(primality_test_v2(9007199254740881), Some(true));
        assert_eq!(primality_test_v2(9007199254740991), Some(false));
    }
}
