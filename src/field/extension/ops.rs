//! Arithmetic operators (`Add`, `Sub`, `Mul`, `Neg`) for `Poly<T>`.

use crate::field::FiniteSimpleExtension;
use crate::traits::*;
use std::ops::{Add, Mul, Neg, Sub};

/// Adds two extension field elements.
///  
/// # Example
/// ```
/// # use algebraics::field::{Fp, FiniteSimpleExtension};
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{IrreduciblePoly, One, Zero, FieldExtension};
/// # struct ConwayGF4;
/// # impl IrreduciblePoly<Fp<2>> for ConwayGF4 {
/// #     fn modulus() -> Poly<Fp<2>> {
/// #         Poly::new(vec![Fp::<2>::one(), Fp::<2>::one(), Fp::<2>::one()])
/// #     }
/// # }
/// # type GF4 = FiniteSimpleExtension<Fp<2>, ConwayGF4>;
/// let alpha = GF4::generator();          // α, a root of x²+x+1
/// let one = GF4::one();
/// let sum = &alpha + &one;               // α+1
/// assert_eq!(sum, alpha + GF4::one());
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Add for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;

    fn add(self, rhs: Self) -> Self::Output {
        FiniteSimpleExtension::new(&self.repr + &rhs.repr)
    }
}

/// Adds two extension field elements by reference; delegates to `&FiniteSimpleExtension + &FiniteSimpleExtension`.
impl<F: Field, M: IrreduciblePoly<F>> Add for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        &self + &rhs
    }
}

/// Negates an extension field element.
///  
/// # Example
/// ```
/// # use algebraics::field::{Fp, FiniteSimpleExtension};
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{IrreduciblePoly, One, Zero, FieldExtension};
/// # struct ConwayGF4;
/// # impl IrreduciblePoly<Fp<2>> for ConwayGF4 {
/// #     fn modulus() -> Poly<Fp<2>> {
/// #         Poly::new(vec![Fp::<2>::one(), Fp::<2>::one(), Fp::<2>::one()])
/// #     }
/// # }
/// # type GF4 = FiniteSimpleExtension<Fp<2>, ConwayGF4>;
/// let alpha = GF4::generator();
/// // In characteristic 2, negation is the identity
/// assert_eq!(-&alpha, alpha);
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Neg for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;
    fn neg(self) -> Self::Output {
        FiniteSimpleExtension::new(-&self.repr)
    }
}

/// Negates an extension field elements by reference; delegates to `-&FiniteSimpleExtension`.
impl<F: Field, M: IrreduciblePoly<F>> Neg for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn neg(self) -> Self {
        -&self
    }
}

/// Subtracts two extension field elements.
///  
/// # Example
/// ```
/// # use algebraics::field::{Fp, FiniteSimpleExtension};
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{IrreduciblePoly, One, Zero, FieldExtension};
/// # struct ConwayGF4;
/// # impl IrreduciblePoly<Fp<2>> for ConwayGF4 {
/// #     fn modulus() -> Poly<Fp<2>> {
/// #         Poly::new(vec![Fp::<2>::one(), Fp::<2>::one(), Fp::<2>::one()])
/// #     }
/// # }
/// # type GF4 = FiniteSimpleExtension<Fp<2>, ConwayGF4>;
/// let alpha = GF4::generator();
/// // In characteristic 2, subtraction equals addition
/// assert_eq!(&alpha - &GF4::one(), &alpha + &GF4::one());
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Sub for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

/// Subtracts two extension field elements by reference; delegates to `&FiniteSimpleExtension - &FiniteSimpleExtension`.
impl<F: Field, M: IrreduciblePoly<F>> Sub for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self + &(-&rhs)
    }
}

/// Multiplies two extension field elements, reducing mod M.
///
/// Multiplication first computes the product in F\[x\], then reduces
/// mod M using polynomial division. This is the key operation that
/// distinguishes extension field arithmetic from polynomial arithmetic.
/// # Example
/// ```
/// # use algebraics::field::{Fp, FiniteSimpleExtension};
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{IrreduciblePoly, One, Zero, FieldExtension};
/// # struct ConwayGF4;
/// # impl IrreduciblePoly<Fp<2>> for ConwayGF4 {
/// #     fn modulus() -> Poly<Fp<2>> {
/// #         Poly::new(vec![Fp::<2>::one(), Fp::<2>::one(), Fp::<2>::one()])
/// #     }
/// # }
/// # type GF4 = FiniteSimpleExtension<Fp<2>, ConwayGF4>;
/// let alpha = GF4::generator();
/// // α² = α+1, since x² ≡ x+1 mod (x²+x+1)
/// assert_eq!(&alpha * &alpha, alpha + GF4::one());
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Mul for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;
    fn mul(self, rhs: Self) -> Self::Output {
        FiniteSimpleExtension::new(&self.repr * &rhs.repr)
    }
}

/// Multiplies two extension field elements by reference; delegates to `&FiniteSimpleExtension * &FiniteSimpleExtension`.
impl<F: Field, M: IrreduciblePoly<F>> Mul for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
