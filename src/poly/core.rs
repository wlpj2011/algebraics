use crate::traits::*;
use std::fmt::Display;
use std::ops::{Add, Mul, Neg, Sub};

use std::cmp::max;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T: Ring> {
    coeffs: Vec<T>,
}

impl<T: Ring + Display> Display for Poly<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.degree() {
            None => write!(f, "0"),
            Some(degree) => {
                let mut first = true;
                for i in (0..=degree).rev() {
                    let coeff = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                    if coeff == T::zero() {
                        continue;
                    }
                    if coeff == T::one() {
                        if !first {
                            write!(f, " + ")?;
                        }
                        match i {
                            0 => write!(f, "{}", coeff)?,
                            1 => write!(f, "x")?,
                            _ => write!(f, "x^{}", i)?,
                        }
                        first = false;
                    } else {
                        if !first {
                            write!(f, " + ")?;
                        }
                        match i {
                            0 => write!(f, "{}", coeff)?,
                            1 => write!(f, "{}*x", coeff)?,
                            _ => write!(f, "{}*x^{}", coeff, i)?,
                        }
                        first = false;
                    }
                }
                Ok(())
            }
        }
    }
}
impl<T: Ring> Poly<T> {
    /// Remove all trailing zeros
    fn normalize(mut coeffs: Vec<T>) -> Vec<T> {
        while coeffs.last().is_some_and(|c| c == &T::zero()) {
            coeffs.pop();
        }
        coeffs
    }

    pub fn new(coeffs: Vec<T>) -> Self {
        Poly {
            coeffs: Self::normalize(coeffs),
        }
    }

    /// Returns None for the 0 polynomial, otherwise returns a non-negative degree.
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }
}

impl<T: Ring> Zero for Poly<T> {
    fn zero() -> Self {
        Poly { coeffs: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }
}

impl<T: Ring> One for Poly<T> {
    fn one() -> Self {
        Self::new(vec![T::one()])
    }
}

impl<T: Ring> Add for &Poly<T> {
    type Output = Poly<T>;

    fn add(self, rhs: Self) -> Poly<T> {
        let max_degree = max(self.degree(), rhs.degree());
        match max_degree {
            Some(max_degree) => {
                let mut result = Vec::with_capacity(max_degree + 1);
                for i in 0..=max_degree {
                    let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                    let b = rhs.coeffs.get(i).cloned().unwrap_or(T::zero());
                    result.push(a + b);
                }
                Poly::new(result)
            }
            None => Poly::zero(),
        }
    }
}

impl<T: Ring> Add for Poly<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        &self + &rhs
    }
}

impl<T: Ring> Neg for &Poly<T> {
    type Output = Poly<T>;
    fn neg(self) -> Self::Output {
        let degree = self.degree();
        match degree {
            Some(degree) => {
                let mut result = Vec::with_capacity(degree + 1);
                for i in 0..=degree {
                    let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                    result.push(-a);
                }
                Poly::new(result)
            }
            None => Poly::zero(),
        }
    }
}

impl<T: Ring> Neg for Poly<T> {
    type Output = Self;
    fn neg(self) -> Self {
        -&self
    }
}

impl<T: Ring> Sub for &Poly<T> {
    type Output = Poly<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl<T: Ring> Sub for Poly<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self + &(-&rhs)
    }
}

impl<T: Ring> Mul for &Poly<T> {
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
                        let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                        let b = rhs.coeffs.get(j).cloned().unwrap_or(T::zero());
                        kth_coeff = kth_coeff + a * b;
                    }
                    result.push(kth_coeff);
                }
                Poly::new(result)
            }
        }
    }
}

impl<T: Ring> Mul for Poly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Magma for Poly<T> {}
impl<T: Ring> Semigroup for Poly<T> {}
impl<T: Ring> Monoid for Poly<T> {}
impl<T: Ring> Group for Poly<T> {}
impl<T: Ring> AbelianGroup for Poly<T> {}
impl<T: Ring> Ring for Poly<T> {
    fn characteristic() -> u64 {
        T::characteristic()
    }
}

impl<T: CommutativeRing> CommutativeRing for Poly<T> {}
impl<T: IntegralDomain> IntegralDomain for Poly<T> {}
