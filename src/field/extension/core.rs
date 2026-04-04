//! [`FiniteSimpleExtension`]: the quotient field F[x]/(M(x)).
//!
//! # Mathematical background
//!
//! Given a field F and an irreducible polynomial M ∈ F[x] of degree n, the
//! quotient ring F[x]/(M(x)) is a field. It is an n-dimensional F-vector space,
//! and the image of x — accessible via [`FiniteSimpleExtension::generator`] — is a root
//! of M in this field.
//!
//! This is the universal construction: every simple algebraic extension E/F is
//! isomorphic to F[x]/(m(x)) where m is the minimal polynomial of a primitive
//! element θ ∈ E with F(θ) = E.
//!
//! # Representation
//!
//! Every element of F[x]/(M(x)) is uniquely represented by a polynomial of degree
//! strictly less than deg(M). `FiniteSimpleExtension<F, M>` stores this canonical
//! representative in the `repr` field.
//!
//! **Invariant**: `repr.degree() < M::degree()` at all times.
//! This is maintained by reducing mod M on any operation that can raise degree
//! above deg(M) - 1 (namely, multiplication and [`from_poly`](FiniteSimpleExtension::from_poly)).
//!
//! # Type-level modulus
//!
//! The polynomial M is a *type parameter* rather than a runtime value. Two extensions
//! with different moduli are distinct Rust types, so the compiler prevents accidentally
//! adding elements from incompatible extensions. This is enforced even when the
//! moduli happen to have the same degree.
//!
//! # Constructors
//!
//! - [`zero()`](Zero::zero): the additive identity, the zero polynomial
//! - [`one()`](One::one): the multiplicative identity, the constant polynomial 1
//! - [`embed(k)`](FieldExtension::embed): the base field element k as a constant polynomial
//! - [`generator()`](FiniteSimpleExtension::generator): the image of x, a root of M
//! - [`new(p)`](FiniteSimpleExtension::new): an arbitrary polynomial reduced mod M
use std::fmt::Display;
use std::marker::PhantomData;

use crate::matrix_arithemetic::determinant;
use crate::poly::Poly;
use crate::traits::*;

/// An element of the finite degree simple field extension F\[x\]/(M(x)).
///
/// `F` is the base field and `M` is a zero-size marker type implementing
/// [`IrreduciblePoly<F>`].
pub struct FiniteSimpleExtension<F: Field, M: IrreduciblePoly<F>> {
    pub(crate) repr: Poly<F>,
    _m: PhantomData<M>, // M is only used as a const modulus, never stored
}

impl<F: Field, M: IrreduciblePoly<F>> FiniteSimpleExtension<F, M> {
    /// Constructs an element from a polynomial in F\[x\], reducing mod M.
    ///
    /// If `p` has degree ≥ deg(M), it is reduced via polynomial division, keeping
    /// the remainder. If `p` already has degree < deg(M), it is stored as-is.
    ///
    /// This is the general constructor; prefer [`embed`](FieldExtension::embed)
    /// for base field elements and [`generator`](Self::generator) for the generator.
    pub fn new(repr: Poly<F>) -> Self {
        let reduced_repr = repr.div_rem(M::modulus()).1;
        FiniteSimpleExtension {
            repr: reduced_repr,
            _m: PhantomData,
        }
    }
    /// Returns the image of x in F\[x\]/(M(x)) — a root of M.
    ///
    /// This is the standard generator of the extension. Every element is a
    /// polynomial in the generator with coefficients in F.
    pub fn generator() -> Self {
        let repr = Poly::new(vec![F::zero(), F::one()]);
        let reduced_repr = repr.div_rem(M::modulus()).1;
        FiniteSimpleExtension {
            repr: reduced_repr,
            _m: PhantomData,
        }
    }

    pub(crate) fn multiplication_matrix(&self) -> Vec<Vec<F>> {
        let n = M::degree();
        let mut cols = Vec::with_capacity(n);
        let mut basis_elem = Self::one(); // α^0 = 1
        let generator = Self::generator();
        for _ in 0..n {
            let product = self.clone() * basis_elem.clone();
            // coefficients of product give the column, padding with zeros if needed
            let col: Vec<F> = (0..n).map(|i| product.repr.coeff(i)).collect();
            cols.push(col);
            basis_elem = basis_elem * generator.clone();
        }
        cols // cols[k][i] is the (i,k) entry of the matrix
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Clone for FiniteSimpleExtension<F, M> {
    fn clone(&self) -> Self {
        FiniteSimpleExtension {
            repr: self.repr.clone(),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> PartialEq for FiniteSimpleExtension<F, M> {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Eq for FiniteSimpleExtension<F, M> {}

impl<F: Field + std::fmt::Debug, M: IrreduciblePoly<F>> std::fmt::Debug
    for FiniteSimpleExtension<F, M>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.repr)
    }
}

impl<F: Field + Display, M: IrreduciblePoly<F>> Display for FiniteSimpleExtension<F, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Zero for FiniteSimpleExtension<F, M> {
    fn zero() -> Self {
        FiniteSimpleExtension {
            repr: Poly::zero(),
            _m: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.repr.is_zero()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> One for FiniteSimpleExtension<F, M> {
    fn one() -> Self {
        FiniteSimpleExtension {
            repr: Poly::one(),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Magma for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Semigroup for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Monoid for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Group for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> AbelianGroup for FiniteSimpleExtension<F, M> {}

impl<F: Field, M: IrreduciblePoly<F>> Ring for FiniteSimpleExtension<F, M> {
    fn characteristic() -> u64 {
        F::characteristic()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> CommutativeRing for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> IntegralDomain for FiniteSimpleExtension<F, M> {}

impl<F: Field, M: IrreduciblePoly<F>> Field for FiniteSimpleExtension<F, M> {
    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }
        let (_, u, _) = Poly::ext_gcd(self.repr.clone(), M::modulus());
        Some(Self::new(u))
    }
}

impl<F: Field, M: IrreduciblePoly<F>> FieldExtension for FiniteSimpleExtension<F, M> {
    type BaseField = F;
    /// Embeds a base field element as the constant polynomial with that value.
    fn embed(x: Self::BaseField) -> Self {
        Self {
            repr: Poly::new_constant(x),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> FiniteExtension for FiniteSimpleExtension<F, M> {
    /// The degree \[F\[x\]/(M) : F\] = deg(M).
    fn degree() -> usize {
        M::degree()
    }

    /// Returns `Some(k)` if this element equals `embed(k)` for some k ∈ F,
    /// i.e. if `repr` is a constant polynomial.
    fn project_to_base(&self) -> Option<Self::BaseField> {
        let repr_degree = self.repr.degree();
        match repr_degree {
            None => Some(Self::BaseField::zero()),
            Some(repr_degree) => {
                if repr_degree >= 1 {
                    None
                } else {
                    Some(self.repr.coeff(0))
                }
            }
        }
    }

    /// The field norm N_{E/F}(self).
    ///
    /// Equals the product of all conjugates (Frobenius orbit) of self, or
    /// equivalently the constant term (up to sign and degree) of the
    /// characteristic polynomial of the multiplication-by-self map.
    fn norm(&self) -> Self::BaseField {
        determinant(self.multiplication_matrix())
    }
}

/// Every algebraic extension of a perfect field is separable.
///
/// `SimpleExtension<F, M>` is an algebraic extension of F, so this blanket impl
/// marks it separable whenever F is perfect. Perfect fields include all finite
/// fields and all characteristic-0 fields.
impl<F: PerfectField, M: IrreduciblePoly<F>> SeparableExtension for FiniteSimpleExtension<F, M> {}

/// Trace for separable finite extensions of perfect fields.
impl<F: PerfectField, M: IrreduciblePoly<F>> SeparableFiniteExtension
    for FiniteSimpleExtension<F, M>
{
    fn trace(&self) -> Self::BaseField {
        let multiplication_matrix = self.multiplication_matrix();
        let mut result = Self::BaseField::zero();
        for i in 0..Self::degree() {
            result = result.clone() + multiplication_matrix[i][i].clone();
        }
        result
    }
}

/// The characteristic-p property propagates from base field to extension.
impl<F: CharPField, M: IrreduciblePoly<F>> CharPField for FiniteSimpleExtension<F, M> {}

/// Frobenius for extensions of perfect characteristic-p fields.
impl<F: PerfectField + CharPField, M: IrreduciblePoly<F>> CharPFiniteExtension
    for FiniteSimpleExtension<F, M>
{
    fn frobenius(&self) -> Self {
        let p = Self::characteristic();
        let x = self.clone();
        let mut result = self.clone();
        for _ in 1..p {
            result = &result * &x;
        }
        result
    }
}

/// Trace-via-Frobenius for extensions of perfect characteristic-p fields.
impl<F: PerfectField + CharPField, M: IrreduciblePoly<F>> SeparableCharPFiniteExtension
    for FiniteSimpleExtension<F, M>
{
}
