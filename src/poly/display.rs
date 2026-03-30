//! Display formatting for `Poly<T>`.
use super::core::Poly;
use crate::traits::*;
use std::fmt::Display;

/// Implements pretty-printing for polynomials in standard mathematical notation.
///
/// Omits zero coefficients, formats `x` powers appropriately, and handles
/// the additive identity (`0`) correctly.  
/// Coefficients require `T: Display + Ring`.
///
/// # Examples
/// ```
/// # use algebraics::poly::Poly;
/// # use algebraics::traits::{Zero, One};
/// # type F = algebraics::finite_field::Fp<7>;
///
/// let p = Poly::<F>::new(vec![F::one(), F::zero(), F::one()]);
/// assert_eq!(format!("{}", p), "x^2 + 1");
///
/// let zero = Poly::<F>::zero();
/// assert_eq!(format!("{}", zero), "0");
/// ```
impl<T: Clone + Zero + One + PartialEq + Display> Display for Poly<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.degree() {
            None => write!(f, "0"),
            Some(degree) => {
                let mut first = true;
                for i in (0..=degree).rev() {
                    let coeff = self.coeff(i);
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
