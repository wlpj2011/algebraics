//! Arithmetic operators (`Add`, `Sub`, `Mul`, `Neg`) for `Poly<T>`.

use super::core::Poly;
use crate::traits::*;
use std::cmp::max;
use std::ops::{Add, Mul, Neg, Sub};

/// Adds two polynomials by adding their coefficients.
///  
/// # Example
/// ```
/// # use algebraics::poly::Poly;
/// # use algebraics::field::Fp;
/// # use algebraics::traits::{Zero, One};
/// # type F7 = Fp<7>;
/// let p1 = Poly::<F7>::new(vec![F7::one(), F7::one()]); // x + 1
/// let p2 = Poly::<F7>::new(vec![F7::new(2), F7::one()]); // x + 2
/// let sum = &p1 + &p2;                                    // 2*x + 3 mod 7
/// assert_eq!(sum.to_string(), "2*x + 3");
/// ```
impl<T> Add for &Poly<T>
where
    T: Clone + Zero + Add<Output = T> + PartialEq,
{
    type Output = Poly<T>;

    fn add(self, rhs: Self) -> Poly<T> {
        let max_degree = max(self.degree(), rhs.degree());
        match max_degree {
            Some(max_degree) => {
                let mut result = Vec::with_capacity(max_degree + 1);
                for i in 0..=max_degree {
                    let a = self.coeff(i);
                    let b = rhs.coeff(i);
                    result.push(a + b);
                }
                Poly::new(result)
            }
            None => Poly::zero(),
        }
    }
}

/// Adds two polynomials by reference; delegates to `&Poly + &Poly`.
impl<T> Add for Poly<T>
where
    T: Clone + Zero + Add<Output = T> + PartialEq,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        &self + &rhs
    }
}

/// Negates a polynomial by negating all its coefficients.
///  
/// # Example
/// ```
/// # use algebraics::poly::Poly;
/// # use algebraics::field::Fp;
/// # use algebraics::traits::{Zero, One};
/// # type F7 = Fp<7>;
/// let p = Poly::<F7>::new(vec![F7::one(), F7::one()]); // x + 1
/// let n = -&p;                                         // -x - 1 mod 7
/// assert_eq!(n.to_string(), "6*x + 6");               // descending order
/// ```
impl<T> Neg for &Poly<T>
where
    T: Clone + Zero + Neg<Output = T> + PartialEq,
{
    type Output = Poly<T>;
    fn neg(self) -> Self::Output {
        let degree = self.degree();
        match degree {
            Some(degree) => {
                let mut result = Vec::with_capacity(degree + 1);
                for i in 0..=degree {
                    let a = self.coeff(i);
                    result.push(-a);
                }
                Poly::new(result)
            }
            None => Poly::zero(),
        }
    }
}

/// Negates a polynomial by reference; delegates to `-&Poly`.
impl<T> Neg for Poly<T>
where
    T: Clone + Zero + Neg<Output = T> + PartialEq,
{
    type Output = Self;
    fn neg(self) -> Self {
        -&self
    }
}

/// Subtracts two polynomials by adding the negative of the second.
///  
/// # Example
/// ```
/// # use algebraics::poly::Poly;
/// # use algebraics::field::Fp;
/// # use algebraics::traits::{Zero, One};
/// # type F7 = Fp<7>;
/// let p1 = Poly::<F7>::new(vec![F7::one(), F7::one()]); // x + 1
/// let p2 = Poly::<F7>::new(vec![F7::new(2), F7::one()]); // x + 2
/// let diff = &p1 - &p2;                                  // -1 mod 7, 0*x
/// assert_eq!(diff.to_string(), "6");
/// ```
impl<T> Sub for &Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Neg<Output = T> + PartialEq,
{
    type Output = Poly<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

/// Subtracts two polynomials by reference; delegates to `&Poly - &Poly`.
impl<T> Sub for Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Neg<Output = T> + PartialEq,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self + &(-&rhs)
    }
}

/// Multiplies two polynomials using standard coefficient convolution.
///  
/// # Example
/// ```
/// # use algebraics::poly::Poly;
/// # use algebraics::field::Fp;
/// # use algebraics::traits::{Zero, One};
/// # type F7 = Fp<7>;
/// let p1 = Poly::<F7>::new(vec![F7::one(), F7::one()]); // x + 1
/// let p2 = Poly::<F7>::new(vec![F7::new(2), F7::one()]); // x + 2
/// let prod = &p1 * &p2;                                   // x^2 + 3*x + 2 mod 7
/// assert_eq!(prod.to_string(), "x^2 + 3*x + 2");
/// ```
impl<T> Mul for &Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    type Output = Poly<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self.degree(), rhs.degree()) {
            (None, _) | (_, None) => Poly::zero(),
            (Some(d1), Some(d2)) => {
                let result_degree = d1 + d2;
                let mut result = Vec::with_capacity(result_degree + 1);
                for k in 0..=result_degree {
                    let mut kth_coeff = T::zero();
                    for i in 0..=k {
                        let j = k - i;
                        let a = self.coeff(i);
                        let b = rhs.coeff(j);
                        kth_coeff = kth_coeff + a * b;
                    }
                    result.push(kth_coeff);
                }
                Poly::new(result)
            }
        }
    }
}

/// Multiplies two polynomials by reference; delegates to `&Poly * &Poly`.
impl<T> Mul for Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
