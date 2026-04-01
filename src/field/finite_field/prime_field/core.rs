//! `Fp<P>` is an implementation of the finite prime fields.
//! `Fp<P>` Implements Field and FiniteRing, with all the other traits that implies.
//! There are compile time checks for primality of P.
use crate::arithmetic::*;
use crate::traits::*;

/// The prime field `F_p = Z/pZ` for a prime `P`.
///
/// Elements are stored as integers in `0..P` and are always reduced on
/// construction. Primality of `P` is verified at compile time — `Fp<4>`
/// will not compile.
///
/// # Type parameters
/// - `P`: the field characteristic, which must be prime
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

impl<const P: u64> Magma for Fp<P> {}
impl<const P: u64> Semigroup for Fp<P> {}
impl<const P: u64> Monoid for Fp<P> {}
impl<const P: u64> Group for Fp<P> {}
impl<const P: u64> AbelianGroup for Fp<P> {}
impl<const P: u64> Ring for Fp<P> {
    fn characteristic() -> u64 {
        P
    }
}
impl<const P: u64> CommutativeRing for Fp<P> {}
impl<const P: u64> IntegralDomain for Fp<P> {}

impl<const P: u64> Field for Fp<P> {
    /// Inverts an element of F_p.
    ///
    /// # Examples
    ///
    /// non-zero elements return Some(inverse)
    /// ```
    /// # use algebraics::field::Fp;
    /// # use algebraics::traits::Field;
    /// let a = Fp::<7>::new(3);
    /// assert_eq!(a.inv(), Some(Fp::<7>::new(5))); // 3 * 5 = 1 mod 7
    /// ```
    ///
    /// 0 has no inverse and returns None
    ///  ```
    /// # use algebraics::field::Fp;
    /// # use algebraics::traits::Field;
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
    /// # use algebraics::field::Fp;
    /// let a = Fp::<7>::new(10);  // reduces to 3
    /// assert_eq!(a, Fp::<7>::new(3));
    /// ```
    /// This panics because 4 is not prime:
    /// ```compile_fail
    /// # use algebraics::field::Fp;
    /// let a = Fp::<4>::new(1);  // compile-time assert fires
    /// ```
    pub fn new(n: u64) -> Self {
        const { assert!(is_prime(P), "P must be prime") };
        Fp(n % P)
    }

    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}

impl<const P: u64> Finite for Fp<P> {
    fn enumerate() -> impl Iterator<Item = Self> {
        (0..P).map(Fp::<P>::new)
    }

    fn size() -> usize {
        P as usize
    }
}

impl<const P: u64> FiniteRing for Fp<P> {
    fn is_unit(&self) -> bool {
        !self.is_zero()
    }
}
