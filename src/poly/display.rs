use super::core::Poly;
use crate::traits::*;
use std::fmt::Display;

impl<T: Clone + Zero + One +  PartialEq + Display> Display for Poly<T> {
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
