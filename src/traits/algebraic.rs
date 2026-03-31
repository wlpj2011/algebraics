//! Traits defining the algebraic hierarchy.
//!
//! The hierarchy follows: `Magma → Semigroup → Monoid → Group →
//! AbelianGroup → Ring → CommutativeRing → IntegralDomain → Field`
use std::ops::{Add, Mul, Neg, Sub};
use crate::traits::*;

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