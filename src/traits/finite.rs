//! Traits defining the finite hierarchy
//!  `Finite → FiniteRing → FiniteField`
use crate::traits::*;

/// A type with finitely many values, which can be enumerated exhaustively.
///
/// # Contract
/// - **Completeness**: `enumerate()` yields every distinct value exactly once
/// - **Consistency**: `size()` equals the number of items yielded by `enumerate()`
pub trait Finite: Sized {
    /// Returns an iterator over all elements of this type, each exactly once.
    fn enumerate() -> impl Iterator<Item = Self>;

    /// Returns the total number of distinct elements.
    fn size() -> usize;
}

/// A [`Group`] with finitely many elements.
pub trait FiniteGroup: Finite + Group {}

/// A [`Ring`] with finitely many elements.
///
/// Provides enumeration of invertible elements via [`FiniteRing::units`].
pub trait FiniteRing: Finite + Ring {
    /// Returns `true` if `self` is a multiplicative unit (has a multiplicative inverse).
    fn is_unit(&self) -> bool;

    /// Returns an iterator over all multiplicative units.
    fn units() -> impl Iterator<Item = Self> {
        Self::enumerate().filter(|x| x.is_unit())
    }
}

/// A [`FiniteRing`] that is also a [`Field`].
///
/// The multiplicative group of a finite field is cyclic of order `size() - 1`.
pub trait FiniteField: FiniteRing + Field {
    /// Returns an iterator over all nonzero elements (the multiplicative group F^×).
    fn multiplicative_group() -> impl Iterator<Item = Self> {
        Self::units()
    }
}
impl<T: FiniteRing + Field> FiniteField for T {}
