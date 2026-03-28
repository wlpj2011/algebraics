use std::ops::{Add, Mul, Neg, Sub};

/// A set with a closed binary operation (here, addition).
pub trait Magma: Sized + Clone + PartialEq + Add<Output = Self> {}

// A Magma whose operation is associative.
/// Identical to Magma as a Rust trait — the distinction is semantic.
/// Associativity is a contract — the type system cannot enforce it.
pub trait Semigroup: Magma {}

/// A Semigroup with an additive identity.
pub trait Monoid: Semigroup + Zero {}

/// A Monoid where every element has an additive inverse.
pub trait Group: Monoid + Neg<Output = Self> + Sub<Output = Self> {}

/// A Group whose operation is commutative.
/// Commutativity is semantic — the type system cannot enforce it.
pub trait AbelianGroup: Group {}

/// A set with two binary operations satisfying the ring axioms:
///   - additive AbelianGroup
///   - multiplicative Monoid (with One)
///   - multiplication distributes over addition
/// Distributivity is a semantic contract.
pub trait Ring: AbelianGroup + Mul<Output = Self> + One {}

/// A Ring whose multiplication is commutative.
/// Commutativity is a semantic contract.
pub trait CommutativeRing: Ring {}

/// A CommutativeRing with no zero divisors.
/// Again, this is a semantic contract.
pub trait IntegralDomain: CommutativeRing {}

/// An IntegralDomain where every nonzero element has a multiplicative inverse.
pub trait Field: IntegralDomain {
    /// Returns None for the zero element.
    fn inv(&self) -> Option<Self>;

    fn div(&self, other: &Self) -> Option<Self> {
        other.inv().map(|inv| self.clone() * inv)
    }
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}
