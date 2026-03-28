use crate::traits::*;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fp<const P: u64>(u64);

impl<const P: u64> Zero for Fp<P> {
    fn zero() -> Self {
        Fp(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const P: u64> One for Fp<P> {
    fn one() -> Self {
        Fp(1)
    }
}

impl<const P: u64> Add for Fp<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Fp((self.0 + rhs.0) % P)
    }
}

impl<const P: u64> Neg for Fp<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fp((P - self.0) % P)
    }
}

impl<const P: u64> Sub for Fp<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<const P: u64> Mul for Fp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Fp((self.0 * rhs.0) % P)
    }
}

impl<const P: u64> Magma for Fp<P> {}
impl<const P: u64> Semigroup for Fp<P> {}
impl<const P: u64> Monoid for Fp<P> {}
impl<const P: u64> Group for Fp<P> {}
impl<const P: u64> AbelianGroup for Fp<P> {}
impl<const P: u64> Ring for Fp<P> {}
impl<const P: u64> CommutativeRing for Fp<P> {}
impl<const P: u64> IntegralDomain for Fp<P> {}

impl<const P: u64> Field for Fp<P> {
    /// Inverts an element of F_p.
    ///
    /// # Examples
    ///
    /// non-zero elements return Some(inverse)
    /// ```
    /// use algebraics::prime_field::Fp;
    /// use algebraics::traits::Field;
    /// let a = Fp::<7>::new(3);
    /// assert_eq!(a.inv(), Some(Fp::<7>::new(5))); // 3 * 5 = 1 mod 7
    /// ```
    ///
    /// 0 has no inverse and returns None
    ///  ```
    /// use algebraics::prime_field::Fp;
    /// use algebraics::traits::Field;
    /// let a = Fp::<7>::new(0);
    /// assert_eq!(a.inv(), None); // 3 * 5 = 1 mod 7
    /// ```
    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(Fp(mod_pow(self.0, P - 2, P)))
        }
    }
}

impl<const P: u64> Fp<P> {
    /// Creates a new element of F_p.
    ///
    /// # Examples
    /// ```
    /// use algebraics::prime_field::Fp;
    /// let a = Fp::<7>::new(10);  // reduces to 3
    /// assert_eq!(a, Fp::<7>::new(3));
    /// ```
    /// This panics because 4 is not prime:
    /// ```compile_fail
    /// use algebraics::prime_field::Fp;
    /// let a = Fp::<4>::new(1);  // compile-time assert fires
    /// ```
    pub fn new(n: u64) -> Self {
        const { assert!(is_prime(P), "P must be prime") };
        Fp(n % P)
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

const fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
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
