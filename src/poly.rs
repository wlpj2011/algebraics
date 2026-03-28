use crate::traits::*;
use std::cmp::max;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T: Ring> {
    coeffs: Vec<T>,
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

impl<T: Ring> Add for Poly<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_degree = max(self.degree(), rhs.degree());
        match max_degree {
            Some(max_degree) => {
                let mut result = Vec::with_capacity(max_degree);
                for i in 0..max_degree {
                    let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                    let b = rhs.coeffs.get(i).cloned().unwrap_or(T::zero());
                    result.push(a + b);
                }
                Self::new(result)
            }
            None => Self::zero(),
        }
    }
}

impl<T: Ring> Neg for Poly<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let degree = self.degree();
        match degree {
            Some(degree) => {
                let mut result = Vec::with_capacity(degree);
                for i in 0..degree {
                    let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                    result.push(-a);
                }
                Self::new(result)
            }
            None => Self::zero(),
        }
    }
}

impl<T: Ring> Sub for Poly<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: Ring> Mul for Poly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let degree = self.degree().unwrap_or_default() + rhs.degree().unwrap_or_default();

        let mut result = Vec::with_capacity(degree);
        for k in 0..degree {
            let mut kth_coeff = T::zero();
            for i in 0..k {
                let j = k - i;
                let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
                let b = self.coeffs.get(j).cloned().unwrap_or(T::zero());
                kth_coeff = kth_coeff + a * b;
            }
            result.push(kth_coeff);
        }
        Self::new(result)
    }
}

impl<T: Ring> Magma for Poly<T> {}
impl<T: Ring> Semigroup for Poly<T> {}
impl<T: Ring> Monoid for Poly<T> {}
impl<T: Ring> Group for Poly<T> {}
impl<T: Ring> AbelianGroup for Poly<T> {}
impl<T: Ring> Ring for Poly<T> {}

impl<T: CommutativeRing> CommutativeRing for Poly<T> {}
impl<T: IntegralDomain> IntegralDomain for Poly<T> {}
