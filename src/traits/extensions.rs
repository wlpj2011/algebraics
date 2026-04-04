//! Traits for field extensions and their structure.
//!
//! # Trait hierarchy
//!
//! ```text
//!                  FieldExtension
//!                 /              \
//!       FiniteExtension      SeparableExtension
//!            |    \                    |
//!    CharPExtension \     SeparableFiniteExtension
//!            |       \   (SeparableExtension + FiniteExtension)
//!            |        \_______________|
//!            |         SeparableCharPExtension
//!            |         (CharPExtension + SeparableFiniteExtension)
//!                               |
//!                    FiniteFieldExtension
//!                         (+ FiniteField)
//! ```
//!
//! [`IrreduciblePoly`] is an auxiliary trait providing a type-level polynomial modulus;
//! it is not part of the extension hierarchy itself.
use crate::poly::Poly;
use crate::traits::*;

/// A type-level marker carrying an irreducible polynomial over a field `F`.
///
/// Implementors are zero-size marker types that are never instantiated; their
/// sole role is to carry a polynomial as associated data, analogously to how
/// a const generic carries a scalar value. Distinct marker types produce distinct
/// Rust types for the resulting quotient fields, preventing accidental mixing of
/// elements from incompatible extensions at the type level.
///
/// # Contract
/// - `modulus()` **must** return an irreducible polynomial over `F`.
///   This is an unchecked precondition. Violating it produces a quotient ring
///   that is not a field (the quotient will have zero divisors).
/// - `degree()` equals `modulus().degree().unwrap()` and is at least 1.
///
/// # Example
/// ```
/// # use algebraics::field::Fp;
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{IrreduciblePoly, One, Zero};
///
/// /// Conway polynomial for GF(4): x² + x + 1 over F₂.
/// struct ConwayGF4;
///
/// impl IrreduciblePoly<Fp<2>> for ConwayGF4 {
///     fn modulus() -> Poly<Fp<2>> {
///         // Coefficients in ascending degree order: 1 + x + x²
///         Poly::new(vec![Fp::<2>::one(), Fp::<2>::one(), Fp::<2>::one()])
///     }
/// }
///
/// assert_eq!(ConwayGF4::degree(), 2);
/// ```
pub trait IrreduciblePoly<F: Field> {
    /// Returns the modulus polynomial. Must be irreducible over `F`.
    fn modulus() -> Poly<F>;
    /// Returns the degree of the modulus. Guaranteed to be at least 1.
    fn degree() -> usize {
        Self::modulus().degree().unwrap()
    }
}

/// The base trait for a field extension E/K.
///
/// An extension E/K is a pair of fields with an embedding K → E. This trait
/// represents the extension field E, carrying K as an associated type.
///
/// # Contract
/// - `embed` must be a unital ring homomorphism: it preserves `+`, `*`,
///   `zero()`, and `one()`.
pub trait FieldExtension: Field {
    /// The base field K.
    type BaseField: Field;
    /// Embeds a base field element into the extension as a constant.
    fn embed(x: Self::BaseField) -> Self;
}

/// A field extension E/K of finite degree \[E:K\].
///
/// "Finite" here means the degree \[E:K\] = dim_K(E) is finite, not that E is a
/// finite set. For example, ℚ(√2)/ℚ is a degree-2 extension of infinite fields.
///
/// # Contract
/// - `degree()` returns \[E:K\], the dimension of E as a K-vector space.
/// - `norm(self)` returns N_{E/K}(self), the determinant of the K-linear map
///   "multiplication by self" on E. For a degree-n extension, this equals the
///   product of all conjugates of self over K.
/// - `project_to_base(self)` returns `Some(k)` if and only if `self == embed(k)`
///   for some `k ∈ K`, and `None` otherwise.
pub trait FiniteExtension: FieldExtension {
    /// The degree \[E:K\] of the extension.
    fn degree() -> usize;
    /// Projects to the base field if this element lies in K, otherwise `None`.
    fn project_to_base(&self) -> Option<Self::BaseField>;
    /// The field norm N_{E/K}(self) ∈ K.
    fn norm(&self) -> Self::BaseField;
}

/// A field extension in which every element is separable over K.
///
/// An element α ∈ E is separable over K if its minimal polynomial over K is
/// squarefree (equivalently, has no repeated roots in an algebraic closure).
/// An extension is separable if every element is separable.
///
/// This is a marker trait. The trace is not provided here because it is only
/// a well-defined map E → K for finite-degree extensions; see
/// [`SeparableFiniteExtension`] for the version with a computable trace.
///
/// In practice, separability is automatic over perfect fields (characteristic 0
/// or finite fields), and this is expressed by blanket impls in concrete types.
pub trait SeparableExtension: FieldExtension {}

/// A separable, finite-degree extension E/K.
///
/// The field trace Tr_{E/K}: E → K is the trace of the K-linear map
/// "multiplication by self" on E, or equivalently the sum of all conjugates
/// of self over K. Separability implies the trace form is non-degenerate.
///
/// # Contract
/// - `trace(self)` equals the sum of all conjugates of self over K.
/// - `trace` is K-linear: for all `k ∈ K`, `a, b ∈ E`,
///   `trace(embed(k) * a + b) == k * trace(a) + trace(b)`.
pub trait SeparableFiniteExtension: SeparableExtension + FiniteExtension {
    /// The field trace Tr_{E/K}(self) ∈ K.
    fn trace(&self) -> Self::BaseField;
}

/// A finite-degree extension of a characteristic-p field.
///
/// The Frobenius endomorphism φ: x ↦ x^p is a field automorphism of any
/// characteristic-p field and all its extensions. For an extension of degree n
/// over F_p, the Frobenius generates a cyclic group of order n acting on E over K.
///
/// # Contract
/// - `frobenius(x)` must equal x^p, where p = `Self::characteristic()`.
/// - `frobenius_iter(k)` equals the Frobenius applied k times: x ↦ x^{p^k}.
/// - For a degree-n extension of F_p, `frobenius_iter(n)` must be the identity.
pub trait CharPFiniteExtension: FiniteExtension + CharPField {
    /// The Frobenius endomorphism: x ↦ x^p.
    fn frobenius(&self) -> Self;

    /// The k-fold Frobenius: x ↦ x^{p^k}.
    fn frobenius_iter(&self, k: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..k {
            result = result.frobenius();
        }
        result
    }
}

/// A separable, finite-degree extension of a characteristic-p field.
///
/// Separability implies that the conjugates of x ∈ E over K are exactly the
/// Frobenius orbit x, φ(x), φ²(x), …, φ^{n-1}(x), giving the efficient formula
///
/// > Tr_{E/K}(x) = x + φ(x) + φ²(x) + ⋯ + φ^{n-1}(x)
///
/// which computes the trace without choosing a basis. This is the form used in
/// Gauss sum computations via the Gross–Koblitz formula.
///
/// # Contract
/// All contracts of [`CharPExtension`] and [`SeparableFiniteExtension`] apply.
/// `trace_via_frobenius(self)` must agree with `trace(self)` on all elements.
pub trait SeparableCharPFiniteExtension: CharPFiniteExtension + SeparableFiniteExtension {
    /// Computes Tr_{E/K}(self) as the sum of the Frobenius orbit.
    ///
    /// Default: sums `frobenius_iter(k)` for k in `0..Self::degree()`
    fn trace_via_frobenius(&self) -> Self::BaseField {
        todo!()
    }
}

// Finite field extension.
// pub trait FiniteFieldExtension: SeparableCharPExtension + FiniteField {}
