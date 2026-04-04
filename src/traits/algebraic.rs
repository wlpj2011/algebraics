//! Traits defining the algebraic hierarchy.
//!
//! The hierarchy follows: `Magma → Semigroup → Monoid → Group →
//! AbelianGroup → Ring → CommutativeRing → IntegralDomain → Field`
use crate::traits::*;
use std::ops::{Add, Mul, Neg, Sub};

/// A set with a closed binary operation (here, addition).
///
/// # Contract
/// - **Closure**: `a + b` is defined and lies in the set for all `a`, `b`.
pub trait Magma: Sized + Clone + PartialEq + Add<Output = Self> {}

/// A [`Magma`] whose operation is associative.
///
/// # Contract
/// - **Associativity**: `(a + b) + c == a + (b + c)` for all `a`, `b`, `c`
pub trait Semigroup: Magma {}

/// A [`Semigroup`] with an additive identity element.
///
/// # Contract
/// - **Left identity**: `Zero::zero() + a == a` for all `a`
/// - **Right identity**: `a + Zero::zero() == a` for all `a`
pub trait Monoid: Semigroup + Zero {}

/// A [`Monoid`] where every element has an additive inverse.
///
/// # Contract
/// - **Left inverse**: `(-a) + a == Zero::zero()` for all `a`
/// - **Right inverse**: `a + (-a) == Zero::zero()` for all `a`
pub trait Group: Monoid + Neg<Output = Self> + Sub<Output = Self> {}

/// A [`Group`] whose operation is commutative.
///
/// # Contract
/// - **Commutativity**: `a + b == b + a` for all `a`, `b`
pub trait AbelianGroup: Group {}

/// A set with two binary operations satisfying the ring axioms.
///
/// # Contract
/// - **Additive structure**: forms an [`AbelianGroup`] under `+`
/// - **Multiplicative structure**: forms a [`Monoid`] under `*` with identity [`One::one()`]
/// - **Left distributivity**: `a * (b + c) == a * b + a * c` for all `a`, `b`, `c`
/// - **Right distributivity**: `(a + b) * c == a * c + b * c` for all `a`, `b`, `c`
/// - **Characteristic**: `characteristic()` returns the smallest `n > 0` such that
///   `n · 1 == 0`, or `0` if no such `n` exists
pub trait Ring: AbelianGroup + Mul<Output = Self> + One {
    /// Returns the characteristic of the ring, or `0` for characteristic zero.
    fn characteristic() -> u64;
}

/// A [`Ring`] whose multiplication is commutative.
///
/// # Contract
/// - **Commutativity**: `a * b == b * a` for all `a`, `b`
pub trait CommutativeRing: Ring {}

/// A [`CommutativeRing`] with no zero divisors.
///
/// # Contract
/// - **No zero divisors**: `a * b == Zero::zero()` implies `a == Zero::zero()`
///   or `b == Zero::zero()`
pub trait IntegralDomain: CommutativeRing {}

/// A [`IntegralDomain`] with a Euclidean Function
/// This allows a well-defined division algorithm and gcd
/// Note that gcds can be defined far more generally than this!
/// # Contract
/// - There exists a Euclidean function `f` such that for `a`, `b` in the Domain, `b` non-zero
///   then there exits `q` and `r` in the domain with `a = b * q + r` and either `r == Zero::zero()` or `f(r) < f(b)`.
/// - For the above, we have `(q, r) = div_rem(a, b)`.
/// - Note that `f` does not need to be stated anywhere, the particular `f` is not important, just that div_rem returns as above.
pub trait EuclideanDomain: IntegralDomain {
    /// # Panics
    /// Panics if `other` is zero — this is a programmer error, not a
    /// recoverable condition. Check before calling.
    fn div_rem(self, other: Self) -> (Self, Self);

    fn gcd(mut a: Self, mut b: Self) -> Self {
        // guard: gcd(0,0) is undefined
        while !b.is_zero() {
            let (_, r) = a.div_rem(b.clone());
            a = b;
            b = r;
        }
        a
    }

    fn ext_gcd(a: Self, b: Self) -> (Self, Self, Self)
    where
        Self: Clone,
    {
        // Invariant: old_r = old_s * a + old_t * b  at all times.
        let mut old_r = a;
        let mut r = b;
        let mut old_s = Self::one();
        let mut s = Self::zero();
        let mut old_t = Self::zero();
        let mut t = Self::one();

        while !r.is_zero() {
            let (q, rem) = old_r.clone().div_rem(r.clone());
            old_r = r;
            r = rem;
            let new_s = old_s.clone() - q.clone() * s.clone();
            old_s = s;
            s = new_s;
            let new_t = old_t.clone() - q * t.clone();
            old_t = t;
            t = new_t;
        }

        // old_r = gcd, old_s and old_t are the Bezout coefficients
        (old_r, old_s, old_t)
    }
}

/// An [`IntegralDomain`] where every nonzero element has a multiplicative inverse.
///
/// # Contract
/// - **Multiplicative inverse**: for all `a` with `!a.is_zero()`,
///   `a.inv()` returns `Some(b)` where `a * b == One::one()`
pub trait Field: IntegralDomain {
    /// Returns the multiplicative inverse of `self`, or `None` if `self` is zero.
    fn inv(&self) -> Option<Self>;

    /// Divides `self` by `other`. Returns `None` if `other` is zero.
    fn div(&self, other: &Self) -> Option<Self> {
        other.inv().map(|inv| self.clone() * inv)
    }
}

/// All [`Field`] are trivially [`EuclideanDomain`]
impl<T: Field> EuclideanDomain for T {
    fn div_rem(self, other: Self) -> (Self, Self) {
        (self * other.inv().unwrap(), Self::zero())
    }
}

/// A [`Ring`] of finite characteristic.
/// Specifically characteristic()  ≠ 0.
/// A marker trait to make simple and separable extensions easier.
pub trait CharPField: Field {}

/// A [`Field`] of finite characteristic.
/// Specifically characteristic() == 0.
/// A marker trait to make simple and separable extensions easier.
pub trait Char0Field: Field {}

/// A [`Field`] where every algebraic extension is automatically separable.
/// Char 0 fields and finite fields are perfect.
// Can't use blanket impl for FiniteField because Rust doesn't realize Char0Field and FiniteField are disjoint
pub trait PerfectField: Field {}
impl<T: Char0Field> PerfectField for T {}
