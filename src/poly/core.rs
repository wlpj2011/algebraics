use crate::traits::*;

/// A polynomial with coefficients in an algebraic ring.
///
/// # Type parameters
/// - `T`: the coefficient type. `T: Zero + PartialEq` necessary
///   for construction and normalization. 
/// - Most operations require `T: Ring` or stronger.
///
/// # Representation
/// Internally, polynomials are stored as a `Vec<T>` of coefficients in increasing degree order:
/// `coeffs[0] + coeffs[1]*x + ... + coeffs[n]*x^n`.
/// Trailing zeros are automatically removed on construction.
///
/// The zero polynomial has an empty `coeffs` vector and `degree()` returns `None`.
///
/// # Examples
/// ```
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{Zero, One};
/// # type F = algebraics::finite_field::Fp<7>;
///
/// let p = Poly::<F>::new(vec![F::one(), F::zero(), F::one()]); // 1 + x^2
/// assert_eq!(p.degree(), Some(2));
/// let zero = Poly::<F>::zero();
/// assert!(zero.degree().is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T> {
    /// Coefficients in increasing order of degree. Trailing zeros are removed.
    coeffs: Vec<T>,
}

impl<T> Poly<T> {
    /// Returns the degree of the polynomial, or `None` if it is the zero polynomial.
    ///
    /// # Examples
    /// ```
    /// # use algebraics::poly::Poly;
    /// # use algebraics::traits::{Zero, One};
    /// type F = algebraics::finite_field::Fp<7>;
    /// let p = Poly::<F>::new(vec![F::one(), F::zero(), F::one()]);
    /// assert_eq!(p.degree(), Some(2));
    /// let zero = Poly::<F>::zero();
    /// assert!(zero.degree().is_none());
    /// ```
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    /// Returns the coefficient of x^i.
    ///
    /// If `i` is greater than the polynomial's degree, returns `T::zero()`.
    ///
    /// This is a `pub(crate)` method intended for use within the `poly` module (e.g., for
    /// arithmetic operations or display formatting).
    pub(crate) fn coeff(&self, i: usize) -> T
    where
        T: Clone + Zero,
    {
        self.coeffs.get(i).cloned().unwrap_or(T::zero())
    }
    /// Returns the coefficient of x^i.
    ///
    /// If `i` is greater than the polynomial's degree, returns `T::zero()`.
    ///
    /// This is a `pub(crate)` method intended for use within the `poly` module (e.g., for
    /// arithmetic operations or display formatting).
    pub(crate) fn lead_coeff(&self) -> T
    where T: Clone + Zero {
        let degree = self.degree();
        match degree {
            None => T::zero(),
            Some(degree) => self.coeffs.get(degree).cloned().unwrap(),
        }
        
    }
}

impl<T: Zero + PartialEq> Poly<T> {
    /// Remove all trailing zeros from a coefficient vector.
    ///
    /// # Panics
    /// None. Always succeeds if `T` implements `Zero + PartialEq`.
    fn normalize(mut coeffs: Vec<T>) -> Vec<T> {
        while coeffs.last().is_some_and(|c| c == &T::zero()) {
            coeffs.pop();
        }
        coeffs
    }

    /// Creates a new polynomial from a vector of coefficients, removing trailing zeros.
    ///
    /// # Examples
    /// ```
    /// # use algebraics::poly::Poly;
    /// # use algebraics::traits::{Zero, One};
    /// type F = algebraics::finite_field::Fp<7>;
    /// let p = Poly::<F>::new(vec![F::one(), F::zero(), F::one()]);
    /// ```
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
