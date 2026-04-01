//! Arithmetic operators (`Add`, `Sub`, `Mul`, `Neg`) for `Poly<T>`.

use crate::field::FiniteSimpleExtension;
use crate::traits::*;
use std::ops::{Add, Mul, Neg, Sub};

/// Adds two polynomials by adding their coefficients.
///  
/// # Example
/// ```
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Add for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;

    fn add(self, rhs: Self) -> Self::Output {
        FiniteSimpleExtension::new(&self.repr + &rhs.repr)
    }
}

/// Adds two polynomials by reference; delegates to `&Poly + &Poly`.
impl<F: Field, M: IrreduciblePoly<F>> Add for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        &self + &rhs
    }
}

/// Negates a polynomial by negating all its coefficients.
///  
/// # Example
/// ```
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Neg for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;
    fn neg(self) -> Self::Output {
        FiniteSimpleExtension::new(-&self.repr)
    }
}

/// Negates a polynomial by reference; delegates to `-&Poly`.
impl<F: Field, M: IrreduciblePoly<F>> Neg for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn neg(self) -> Self {
        -&self
    }
}

/// Subtracts two polynomials by adding the negative of the second.
///  
/// # Example
/// ```
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Sub for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

/// Subtracts two polynomials by reference; delegates to `&Poly - &Poly`.
impl<F: Field, M: IrreduciblePoly<F>> Sub for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self + &(-&rhs)
    }
}

/// Multiplies two polynomials using standard coefficient convolution.
///  
/// # Example
/// ```
/// ```
impl<F: Field, M: IrreduciblePoly<F>> Mul for &FiniteSimpleExtension<F, M> {
    type Output = FiniteSimpleExtension<F, M>;
    fn mul(self, rhs: Self) -> Self::Output {
        FiniteSimpleExtension::new(&self.repr * &rhs.repr)
    }
}

/// Multiplies two polynomials by reference; delegates to `&Poly * &Poly`.
impl<F: Field, M: IrreduciblePoly<F>> Mul for FiniteSimpleExtension<F, M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
