//! Concrete finite field extensions `FpN<P, N> = GF(p^n)`.
//!
//! This module provides two public items:
//!
//! - **[`ConwayPoly<P, N>`]** — a zero-size marker type implementing
//!   [`IrreduciblePoly<Fp<P>>`] via the precomputed Conway polynomial table.
//! - **[`FpN<P, N>`]** — a type alias for
//!   `FiniteSimpleExtension<Fp<P>, ConwayPoly<P, N>>`, the finite field GF(p^n).
//!
//! # Mathematical background
//!
//! The finite field GF(p^n) is the unique (up to isomorphism) field with p^n
//! elements. It can be constructed as the quotient F_p\[x\]/(f(x)) for any
//! irreducible polynomial f of degree n over F_p. The choice of f affects the
//! explicit representation of elements but not the isomorphism class of the field.
//!
//! This module uses **Conway polynomials** as the canonical choice of f. The
//! Conway polynomial ConPol(p, n) is the lexicographically minimal monic
//! irreducible **primitive** polynomial of degree n over F_p. Primitivity means
//! the image of x in the quotient generates the multiplicative group GF(p^n)^×,
//! which has order p^n - 1.
//!
//! The key advantage of Conway polynomials over an arbitrary irreducible is
//! **tower compatibility**: when k | n, the embedding GF(p^k) ↪ GF(p^n) is
//! canonical, making subfield arithmetic coherent across extensions of different
//! degrees. This property is essential for Gauss sum computations that involve
//! restriction of characters along subfield inclusions.
//!
//! # The generator
//!
//! [`FpN::generator()`](crate::field::extension::FiniteSimpleExtension::generator)
//! returns the image of x in F_p\[x\]/(ConPol(p, n)). By primitivity of the
//! Conway polynomial, this element has multiplicative order exactly p^n - 1 and
//! generates GF(p^n)^×.
//!
//! # Table coverage
//!
//! Conway polynomials are sourced from Frank Lübeck's precomputed table, embedded
//! at compile time via `build.rs` and `data/conway_polynomials.txt`. The table is
//! parsed into a sorted static array; lookups use binary search and are O(log N)
//! in the number of table entries. If [`ConwayPoly::<P, N>::modulus()`] is called
//! for a pair (p, n) absent from the table, it panics.
//!
//! # Examples
//! ```
//! use algebraics::field::FpN;
//! use algebraics::traits::{Finite, FiniteExtension, Ring, Zero, One};
//!
//! // GF(9) = F_3[x]/(x²+2x+2)
//! type GF9 = FpN<3, 2>;
//!
//! assert_eq!(GF9::size(), 9);
//! assert_eq!(GF9::characteristic(), 3);
//! assert_eq!(<GF9 as FiniteExtension>::degree(), 2);
//!
//! // The generator α satisfies α² = α+1 in GF(9)
//! let alpha = GF9::generator();
//! assert_eq!(alpha.clone() * alpha.clone(), alpha + GF9::one());
//! ```

use crate::conway::conway_poly_fp;
use crate::field::FiniteSimpleExtension;
use crate::field::Fp;
use crate::poly::Poly;
use crate::traits::*;

/// A zero-size marker type carrying the Conway polynomial ConPol(P, N) as its
/// [`IrreduciblePoly`] implementation.
///
/// This type is never instantiated. It exists solely to supply a modulus
/// polynomial to [`FiniteSimpleExtension`] at the type level, following the
/// same pattern as user-defined marker structs such as `ConwayGF4` in the
/// integration tests. Distinct const parameters `(P, N)` produce distinct
/// Rust types, preventing accidental mixing of elements from incompatible
/// extensions at compile time.
///
/// # Contract
/// All contracts of [`IrreduciblePoly`] are satisfied: [`modulus()`] returns
/// an irreducible polynomial of degree `N` over `Fp<P>`, and [`degree()`]
/// returns `N`.
///
/// # Panics
/// [`modulus()`] panics if `(P, N)` is not present in the embedded Conway
/// polynomial table.
///
/// [`modulus()`]: IrreduciblePoly::modulus
/// [`degree()`]: IrreduciblePoly::degree
pub struct ConwayPoly<const P: u64, const N: u64>;

impl<const P: u64, const N: u64> IrreduciblePoly<Fp<P>> for ConwayPoly<P, N> {
    fn modulus() -> Poly<Fp<P>> {
        conway_poly_fp::<P>(N).expect("Conway polynomial not in table")
    }
}

/// The finite field GF(p^n) = F_p\[x\]/(ConPol(p, n)).
///
/// A type alias for [`FiniteSimpleExtension<Fp<P>, ConwayPoly<P, N>>`]. All
/// arithmetic, enumeration, trace, norm, and Frobenius operations are inherited
/// from [`FiniteSimpleExtension`] and the blanket trait impls in
/// `field::extension`.
///
/// # Type parameters
/// - `P`: the field characteristic, which must be prime (enforced by [`Fp<P>`]
///   at the first call to [`Fp::new`](crate::field::Fp::new))
/// - `N`: the degree \[GF(p^n) : F_p\]
///
/// # Invariants
/// - `size()` returns p^n
/// - `characteristic()` returns p  
/// - `FiniteExtension::degree()` returns n
/// - `generator()` has multiplicative order exactly p^n - 1
///
/// # Examples
/// ```
/// use algebraics::field::FpN;
/// use algebraics::traits::{FiniteField, Field, One};
///
/// type GF8 = FpN<2, 3>;
///
/// // The generator of GF(8) satisfies x³+x+1 = 0, so α³ = α+1
/// let alpha = GF8::generator();
/// let alpha3 = alpha.clone() * alpha.clone() * alpha.clone();
/// assert_eq!(alpha3, alpha + GF8::one());
///
/// // Every nonzero element satisfies Fermat: x^(2³-1) = x^7 = 1
/// for x in GF8::multiplicative_group() {
///     let x7 = (0..6).fold(x.clone(), |acc, _| acc * x.clone());
///     assert_eq!(x7, GF8::one());
/// }
/// ```
pub type FpN<const P: u64, const N: u64> = FiniteSimpleExtension<Fp<P>, ConwayPoly<P, N>>;
