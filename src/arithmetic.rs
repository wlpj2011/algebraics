// Computes `base^exp mod modulus` using binary exponentiation.
///
/// # Panics
/// Does not panic, but output is unspecified if `modulus == 0`.
pub const fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}

/// Returns `true` if `n` is prime.
///
/// Uses a deterministic Miller-Rabin test with witnesses sufficient to
/// correctly classify all `n < 2^64`.
pub const fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    if n < 9 {
        return true;
    }
    if n.is_multiple_of(3) {
        return false;
    }

    // Write n-1 as 2^r * d with d odd
    let mut d = n - 1;
    let mut r = 0u64;
    while d.is_multiple_of(2) {
        d /= 2;
        r += 1;
    }

    // Witnesses sufficient for all n < 2^64
    let witnesses: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut i = 0;
    while i < witnesses.len() {
        let a = witnesses[i];
        if a >= n {
            i += 1;
            continue;
        }
        if !miller_rabin_check(n, a, d, r) {
            return false;
        }
        i += 1;
    }
    true
}

/// Performs one Miller-Rabin witness check for `n`, given witness `a`
/// and the decomposition `n - 1 = 2^r * d` with `d` odd.
///
/// Returns `true` if `a` does not witness compositeness of `n`.
pub(crate) const fn miller_rabin_check(n: u64, a: u64, d: u64, r: u64) -> bool {
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    let mut i = 0;
    while i < r - 1 {
        x = ((x as u128 * x as u128) % n as u128) as u64;
        if x == n - 1 {
            return true;
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn test_small_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
    }

    #[test]
    fn test_small_composites() {
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(9));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }

    #[test]
    fn test_larger_prime() {
        assert!(is_prime(97));
        assert!(is_prime(7919)); // the 1000th prime
    }

    #[test]
    fn test_larger_composite() {
        assert!(!is_prime(100));
        assert!(!is_prime(7921)); // 89^2
    }
}
