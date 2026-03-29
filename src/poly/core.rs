use crate::traits::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T> {
    coeffs: Vec<T>,
}

impl<T> Poly<T> {
    /// Returns None for the 0 polynomial, otherwise returns a non-negative degree.
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    pub(crate) fn coeff(&self, i: usize) -> T
    where
        T: Clone + Zero,
    {
        self.coeffs.get(i).cloned().unwrap_or(T::zero())
    }
}

impl<T: Zero + PartialEq> Poly<T> {
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
}

impl<T: Zero> Zero for Poly<T> {
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
